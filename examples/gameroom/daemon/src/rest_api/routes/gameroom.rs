// Copyright 2018-2020 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::collections::HashMap;

use actix_web::{client::Client, error, http::StatusCode, web, Error, HttpResponse};
use supplychain_database::{
    helpers,
    models::{Supplychain, SupplychainMember as DbSupplychainMember},
    ConnectionPool,
};
use openssl::hash::{hash, MessageDigest};
use protobuf::Message;
use splinter::admin::messages::{
    CreateCircuit, CreateCircuitBuilder, SplinterNode, SplinterServiceBuilder,
};
use splinter::node_registry::Node;
use splinter::protocol;
use splinter::protos::admin::{
    CircuitManagementPayload, CircuitManagementPayload_Action as Action,
    CircuitManagementPayload_Header as Header,
};

use crate::application_metadata::ApplicationMetadata;
use crate::rest_api::{SupplychaindData, RestApiResponseError};

use super::{
    get_response_paging_info, validate_limit, ErrorResponse, SuccessResponse, DEFAULT_LIMIT,
    DEFAULT_OFFSET,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSupplychainForm {
    alias: String,
    members: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ApiSupplychain {
    circuit_id: String,
    authorization_type: String,
    persistence: String,
    routes: String,
    circuit_management_type: String,
    members: Vec<ApiSupplychainMember>,
    alias: String,
    status: String,
}

impl ApiSupplychain {
    fn from(db_supplychain: Supplychain, db_members: Vec<DbSupplychainMember>) -> Self {
        Self {
            circuit_id: db_supplychain.circuit_id.to_string(),
            authorization_type: db_supplychain.authorization_type.to_string(),
            persistence: db_supplychain.persistence.to_string(),
            routes: db_supplychain.routes.to_string(),
            circuit_management_type: db_supplychain.circuit_management_type.to_string(),
            members: db_members
                .into_iter()
                .map(ApiSupplychainMember::from)
                .collect(),
            alias: db_supplychain.alias.to_string(),
            status: db_supplychain.status,
        }
    }
}

#[derive(Debug, Serialize)]
struct ApiSupplychainMember {
    node_id: String,
    endpoints: Vec<String>,
}

impl ApiSupplychainMember {
    fn from(db_circuit_member: DbSupplychainMember) -> Self {
        ApiSupplychainMember {
            node_id: db_circuit_member.node_id.to_string(),
            endpoints: db_circuit_member.endpoints,
        }
    }
}

pub async fn propose_supplychain(
    pool: web::Data<ConnectionPool>,
    create_supplychain: web::Json<CreateSupplychainForm>,
    node_info: web::Data<Node>,
    client: web::Data<Client>,
    splinterd_url: web::Data<String>,
    supplychaind_data: web::Data<SupplychaindData>,
) -> HttpResponse {
    let response = fetch_node_information(&create_supplychain.members, &splinterd_url, client).await;

    let nodes = match response {
        Ok(nodes) => nodes,
        Err(err) => match err {
            RestApiResponseError::BadRequest(message) => {
                return HttpResponse::BadRequest().json(ErrorResponse::bad_request(&message));
            }
            _ => {
                debug!("Failed to fetch node information: {}", err);
                return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
            }
        },
    };

    let mut members = nodes
        .iter()
        .map(|node| SplinterNode {
            node_id: node.identity.to_string(),
            endpoints: node.endpoints.to_vec(),
        })
        .collect::<Vec<SplinterNode>>();

    members.push(SplinterNode {
        node_id: node_info.identity.to_string(),
        endpoints: node_info.endpoints.to_vec(),
    });

    let scabbard_admin_keys = vec![supplychaind_data.get_ref().public_key.clone()];

    let mut scabbard_args = vec![];
    scabbard_args.push((
        "admin_keys".into(),
        match serde_json::to_string(&scabbard_admin_keys) {
            Ok(s) => s,
            Err(err) => {
                debug!("Failed to serialize scabbard admin keys: {}", err);
                return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
            }
        },
    ));

    let service_and_node_ids = members
        .iter()
        .enumerate()
        .map(|(member_number, node)| (format!("gr{:02}", member_number), node.node_id.to_string()));

    let all_service_ids = service_and_node_ids
        .clone()
        .map(|(service_id, _)| service_id);

    let mut roster = vec![];
    for (service_id, node_id) in service_and_node_ids {
        let peer_services = match serde_json::to_string(
            &all_service_ids
                .clone()
                .filter(|other_service_id| other_service_id != &service_id)
                .collect::<Vec<_>>(),
        ) {
            Ok(s) => s,
            Err(err) => {
                debug!("Failed to serialize peer services: {}", err);
                return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
            }
        };

        let mut service_args = scabbard_args.clone();
        service_args.push(("peer_services".into(), peer_services));

        match SplinterServiceBuilder::new()
            .with_service_id(&service_id)
            .with_service_type("scabbard")
            .with_allowed_nodes(&[node_id])
            .with_arguments(&service_args)
            .build()
        {
            Ok(service) => roster.push(service),
            Err(err) => {
                debug!("Failed to build SplinterService: {}", err);
                return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
            }
        }
    }

    let application_metadata = match check_alias_uniqueness(pool, &create_supplychain.alias) {
        Ok(()) => match ApplicationMetadata::new(&create_supplychain.alias, &scabbard_admin_keys)
            .to_bytes()
        {
            Ok(bytes) => bytes,
            Err(err) => {
                debug!("Failed to serialize application metadata: {}", err);
                return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
            }
        },
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse::bad_request(&err.to_string()));
        }
    };

    let create_request = match CreateCircuitBuilder::new()
        .with_roster(&roster)
        .with_members(&members)
        .with_circuit_management_type("supplychain")
        .with_application_metadata(&application_metadata)
        .build()
    {
        Ok(create_request) => create_request,
        Err(err) => {
            debug!("Failed to build CreateCircuit: {}", err);
            return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
        }
    };

    let payload_bytes = match make_payload(create_request, node_info.identity.to_string()) {
        Ok(bytes) => bytes,
        Err(err) => {
            debug!("Failed to make circuit management payload: {}", err);
            return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
        }
    };

    HttpResponse::Ok().json(SuccessResponse::new(json!({
        "payload_bytes": payload_bytes
    })))
}

async fn fetch_node_information(
    node_ids: &[String],
    splinterd_url: &str,
    client: web::Data<Client>,
) -> Result<Vec<Node>, RestApiResponseError> {
    let node_ids = node_ids.to_owned();
    let mut response = client
        .get(&format!(
            "{}/admin/nodes?limit={}",
            splinterd_url,
            std::i64::MAX
        ))
        .header(
            "SplinterProtocolVersion",
            protocol::ADMIN_PROTOCOL_VERSION.to_string(),
        )
        .send()
        .await
        .map_err(|err| {
            RestApiResponseError::InternalError(format!("Failed to send request {}", err))
        })?;

    let body = response.body().await.map_err(|err| {
        RestApiResponseError::InternalError(format!("Failed to receive response body {}", err))
    })?;

    match response.status() {
        StatusCode::OK => {
            let list_reponse: SuccessResponse<Vec<Node>> =
                serde_json::from_slice(&body).map_err(|err| {
                    RestApiResponseError::InternalError(format!(
                        "Failed to parse response body {}",
                        err
                    ))
                })?;
            let nodes = node_ids.into_iter().try_fold(vec![], |mut acc, node_id| {
                if let Some(node) = list_reponse
                    .data
                    .iter()
                    .find(|node| node.identity == node_id)
                {
                    acc.push(node.clone());
                    Ok(acc)
                } else {
                    Err(RestApiResponseError::BadRequest(format!(
                        "Could not find node with id {}",
                        node_id
                    )))
                }
            })?;

            Ok(nodes)
        }
        StatusCode::BAD_REQUEST => {
            let message: String = serde_json::from_slice(&body).map_err(|err| {
                RestApiResponseError::InternalError(format!(
                    "Failed to parse response body {}",
                    err
                ))
            })?;
            Err(RestApiResponseError::BadRequest(message))
        }
        _ => {
            let message: String = serde_json::from_slice(&body).map_err(|err| {
                RestApiResponseError::InternalError(format!(
                    "Failed to parse response body {}",
                    err
                ))
            })?;

            Err(RestApiResponseError::InternalError(message))
        }
    }
}

fn check_alias_uniqueness(
    pool: web::Data<ConnectionPool>,
    alias: &str,
) -> Result<(), RestApiResponseError> {
    if let Some(supplychain) = helpers::fetch_supplychain_by_alias(&*pool.get()?, alias)? {
        return Err(RestApiResponseError::BadRequest(format!(
            "Supplychain with alias {} already exists",
            supplychain.alias
        )));
    }
    Ok(())
}

fn make_payload(
    create_request: CreateCircuit,
    local_node: String,
) -> Result<Vec<u8>, RestApiResponseError> {
    let circuit_proto = create_request.into_proto()?;
    let circuit_bytes = circuit_proto.write_to_bytes()?;
    let hashed_bytes = hash(MessageDigest::sha512(), &circuit_bytes)?;

    let mut header = Header::new();
    header.set_action(Action::CIRCUIT_CREATE_REQUEST);
    header.set_payload_sha512(hashed_bytes.to_vec());
    header.set_requester_node_id(local_node);
    let header_bytes = header.write_to_bytes()?;

    let mut circuit_management_payload = CircuitManagementPayload::new();
    circuit_management_payload.set_header(header_bytes);
    circuit_management_payload.set_circuit_create_request(circuit_proto);
    let payload_bytes = circuit_management_payload.write_to_bytes()?;
    Ok(payload_bytes)
}

pub async fn list_supplychains(
    pool: web::Data<ConnectionPool>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let mut base_link = "api/supplychains?".to_string();
    let offset: usize = query
        .get("offset")
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| DEFAULT_OFFSET.to_string())
        .parse()
        .unwrap_or_else(|_| DEFAULT_OFFSET);

    let limit: usize = query
        .get("limit")
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| DEFAULT_LIMIT.to_string())
        .parse()
        .unwrap_or_else(|_| DEFAULT_LIMIT);

    let status_optional = query.get("status").map(ToOwned::to_owned);

    if let Some(status) = status_optional.clone() {
        base_link.push_str(format!("status={}?", status).as_str());
    }

    match web::block(move || list_supplychains_from_db(pool, status_optional, limit, offset)).await {
        Ok((supplychains, query_count)) => {
            let paging_info =
                get_response_paging_info(limit, offset, "api/supplychains?", query_count as usize);
            Ok(HttpResponse::Ok().json(SuccessResponse::list(supplychains, paging_info)))
        }
        Err(err) => {
            debug!("Internal Server Error: {}", err);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse::internal_error()))
        }
    }
}

fn list_supplychains_from_db(
    pool: web::Data<ConnectionPool>,
    status_optional: Option<String>,
    limit: usize,
    offset: usize,
) -> Result<(Vec<ApiSupplychain>, i64), RestApiResponseError> {
    let db_limit = validate_limit(limit);
    let db_offset = offset as i64;

    if let Some(status) = status_optional {
        let supplychains = helpers::list_supplychains_with_paging_and_status(
            &*pool.get()?,
            &status,
            db_limit,
            db_offset,
        )?
        .into_iter()
        .map(|supplychain| {
            let circuit_id = supplychain.circuit_id.to_string();
            let members = helpers::fetch_supplychain_members_by_circuit_id_and_status(
                &*pool.get()?,
                &circuit_id,
                &supplychain.status,
            )?;
            Ok(ApiSupplychain::from(supplychain, members))
        })
        .collect::<Result<Vec<ApiSupplychain>, RestApiResponseError>>()?;
        Ok((supplychains, helpers::get_supplychain_count(&*pool.get()?)?))
    } else {
        let supplychains = helpers::list_supplychains_with_paging(&*pool.get()?, db_limit, db_offset)?
            .into_iter()
            .map(|supplychain| {
                let circuit_id = supplychain.circuit_id.to_string();
                let members = helpers::fetch_supplychain_members_by_circuit_id_and_status(
                    &*pool.get()?,
                    &circuit_id,
                    &supplychain.status,
                )?;
                Ok(ApiSupplychain::from(supplychain, members))
            })
            .collect::<Result<Vec<ApiSupplychain>, RestApiResponseError>>()?;
        Ok((supplychains, helpers::get_supplychain_count(&*pool.get()?)?))
    }
}

pub async fn fetch_supplychain(
    pool: web::Data<ConnectionPool>,
    circuit_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    match web::block(move || fetch_supplychain_from_db(pool, &circuit_id)).await {
        Ok(supplychain) => Ok(HttpResponse::Ok().json(supplychain)),
        Err(err) => {
            match err {
                error::BlockingError::Error(err) => match err {
                    RestApiResponseError::NotFound(err) => {
                        Ok(HttpResponse::NotFound().json(ErrorResponse::not_found(&err)))
                    }
                    _ => Ok(HttpResponse::BadRequest()
                        .json(ErrorResponse::bad_request(&err.to_string()))),
                },
                error::BlockingError::Canceled => {
                    debug!("Internal Server Error: {}", err);
                    Ok(HttpResponse::InternalServerError().json(ErrorResponse::internal_error()))
                }
            }
        }
    }
}

fn fetch_supplychain_from_db(
    pool: web::Data<ConnectionPool>,
    circuit_id: &str,
) -> Result<ApiSupplychain, RestApiResponseError> {
    if let Some(supplychain) = helpers::fetch_supplychain(&*pool.get()?, circuit_id)? {
        let members = helpers::fetch_supplychain_members_by_circuit_id_and_status(
            &*pool.get()?,
            &supplychain.circuit_id,
            &supplychain.status,
        )?;
        return Ok(ApiSupplychain::from(supplychain, members));
    }
    Err(RestApiResponseError::NotFound(format!(
        "Supplychain with id {} not found",
        circuit_id
    )))
}
