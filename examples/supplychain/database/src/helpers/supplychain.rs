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

use std::time::SystemTime;

use crate::models::{
    ActiveSupplychain, Supplychain, SupplychainMember, SupplychainProposal, SupplychainService, NewSupplychainMember,
    NewSupplychainProposal, NewSupplychainService, NewProposalVoteRecord,
};
use crate::schema::{
    supplychain, supplychain_member, supplychain_proposal, supplychain_service, proposal_vote_record,
};
use diesel::{
    dsl::insert_into, pg::PgConnection, prelude::*, result::Error::NotFound, QueryResult,
};

pub fn fetch_proposal_by_id(conn: &PgConnection, id: i64) -> QueryResult<Option<SupplychainProposal>> {
    supplychain_proposal::table
        .filter(supplychain_proposal::id.eq(id))
        .first::<SupplychainProposal>(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn fetch_supplychain_members_by_circuit_id_and_status(
    conn: &PgConnection,
    circuit_id: &str,
    status: &str,
) -> QueryResult<Vec<SupplychainMember>> {
    supplychain_member::table
        .filter(
            supplychain_member::circuit_id
                .eq(circuit_id)
                .and(supplychain_member::status.eq(status)),
        )
        .load::<SupplychainMember>(conn)
}

pub fn list_proposals_with_paging(
    conn: &PgConnection,
    limit: i64,
    offset: i64,
) -> QueryResult<Vec<SupplychainProposal>> {
    supplychain_proposal::table
        .select(supplychain_proposal::all_columns)
        .limit(limit)
        .offset(offset)
        .load::<SupplychainProposal>(conn)
}

pub fn get_proposal_count(conn: &PgConnection) -> QueryResult<i64> {
    supplychain_proposal::table.count().get_result(conn)
}

pub fn list_supplychain_members_with_status(
    conn: &PgConnection,
    status: &str,
) -> QueryResult<Vec<SupplychainMember>> {
    supplychain_member::table
        .filter(supplychain_member::status.eq(status))
        .load::<SupplychainMember>(conn)
}

pub fn insert_supplychain_proposal(
    conn: &PgConnection,
    proposal: NewSupplychainProposal,
) -> QueryResult<()> {
    insert_into(supplychain_proposal::table)
        .values(&vec![proposal])
        .execute(conn)
        .map(|_| ())
}

pub fn insert_supplychain(conn: &PgConnection, supplychain: Supplychain) -> QueryResult<()> {
    insert_into(supplychain::table)
        .values(&vec![supplychain])
        .execute(conn)
        .map(|_| ())
}

pub fn update_supplychain_proposal_status(
    conn: &PgConnection,
    proposal_id: i64,
    updated_time: &SystemTime,
    status: &str,
) -> QueryResult<()> {
    diesel::update(supplychain_proposal::table.find(proposal_id))
        .set((
            supplychain_proposal::updated_time.eq(updated_time),
            supplychain_proposal::status.eq(status),
        ))
        .execute(conn)
        .map(|_| ())
}

pub fn supplychain_service_is_active(conn: &PgConnection, circuit_id: &str) -> QueryResult<bool> {
    supplychain_service::table
        .filter(
            supplychain_service::circuit_id
                .eq(circuit_id)
                .and(supplychain_service::status.eq("Active")),
        )
        .first::<SupplychainService>(conn)
        .map(|_| true)
        .or_else(|err| if err == NotFound { Ok(false) } else { Err(err) })
}

pub fn get_last_updated_proposal_time(conn: &PgConnection) -> QueryResult<Option<SystemTime>> {
    supplychain_proposal::table
        .select(supplychain_proposal::updated_time)
        .order_by(supplychain_proposal::updated_time.desc())
        .first(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn fetch_active_supplychains(
    conn: &PgConnection,
    node_id: &str,
) -> QueryResult<Vec<ActiveSupplychain>> {
    supplychain_service::table
        .inner_join(
            supplychain_proposal::table
                .on(supplychain_service::circuit_id.eq(supplychain_proposal::circuit_id)),
        )
        .select((
            supplychain_service::circuit_id,
            supplychain_service::service_id,
            supplychain_service::status,
            supplychain_service::last_event,
            supplychain_proposal::requester,
            supplychain_proposal::requester_node_id,
        ))
        .filter(
            supplychain_service::status
                .eq("Active")
                .and(supplychain_service::allowed_nodes.contains(vec![node_id])),
        )
        .load(conn)
}

pub fn update_supplychain_status(
    conn: &PgConnection,
    circuit_id: &str,
    updated_time: &SystemTime,
    status: &str,
) -> QueryResult<()> {
    diesel::update(supplychain::table.find(circuit_id))
        .set((
            supplychain::updated_time.eq(updated_time),
            supplychain::status.eq(status),
        ))
        .execute(conn)
        .map(|_| ())
}

pub fn update_supplychain_member_status(
    conn: &PgConnection,
    circuit_id: &str,
    updated_time: &SystemTime,
    old_status: &str,
    new_status: &str,
) -> QueryResult<()> {
    diesel::update(
        supplychain_member::table.filter(
            supplychain_member::circuit_id
                .eq(circuit_id)
                .and(supplychain_member::status.eq(old_status)),
        ),
    )
    .set((
        supplychain_member::updated_time.eq(updated_time),
        supplychain_member::status.eq(new_status),
    ))
    .execute(conn)
    .map(|_| ())
}

pub fn update_supplychain_service_status(
    conn: &PgConnection,
    circuit_id: &str,
    updated_time: &SystemTime,
    old_status: &str,
    new_status: &str,
) -> QueryResult<()> {
    diesel::update(
        supplychain_service::table.filter(
            supplychain_service::circuit_id
                .eq(circuit_id)
                .and(supplychain_service::status.eq(old_status)),
        ),
    )
    .set((
        supplychain_service::updated_time.eq(updated_time),
        supplychain_service::status.eq(new_status),
    ))
    .execute(conn)
    .map(|_| ())
}

pub fn update_supplychain_service_last_event(
    conn: &PgConnection,
    circuit_id: &str,
    updated_time: &SystemTime,
    event_id: &str,
) -> QueryResult<()> {
    diesel::update(supplychain_service::table.filter(supplychain_service::circuit_id.eq(circuit_id)))
        .set((
            supplychain_service::updated_time.eq(updated_time),
            supplychain_service::last_event.eq(event_id),
        ))
        .execute(conn)
        .map(|_| ())
}

pub fn insert_proposal_vote_record(
    conn: &PgConnection,
    vote_records: &[NewProposalVoteRecord],
) -> QueryResult<()> {
    insert_into(proposal_vote_record::table)
        .values(vote_records)
        .execute(conn)
        .map(|_| ())
}

pub fn insert_supplychain_services(
    conn: &PgConnection,
    supplychain_services: &[NewSupplychainService],
) -> QueryResult<()> {
    insert_into(supplychain_service::table)
        .values(supplychain_services)
        .execute(conn)
        .map(|_| ())
}

pub fn insert_supplychain_members(
    conn: &PgConnection,
    supplychain_members: &[NewSupplychainMember],
) -> QueryResult<()> {
    insert_into(supplychain_member::table)
        .values(supplychain_members)
        .execute(conn)
        .map(|_| ())
}

pub fn fetch_supplychain_proposal_with_status(
    conn: &PgConnection,
    circuit_id: &str,
    status: &str,
) -> QueryResult<Option<SupplychainProposal>> {
    supplychain_proposal::table
        .select(supplychain_proposal::all_columns)
        .filter(
            supplychain_proposal::circuit_id
                .eq(circuit_id)
                .and(supplychain_proposal::status.eq(status)),
        )
        .first(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn list_supplychains_with_paging_and_status(
    conn: &PgConnection,
    status: &str,
    limit: i64,
    offset: i64,
) -> QueryResult<Vec<Supplychain>> {
    supplychain::table
        .select(supplychain::all_columns)
        .filter(supplychain::status.eq(status))
        .limit(limit)
        .offset(offset)
        .load::<Supplychain>(conn)
}

pub fn get_supplychain_count(conn: &PgConnection) -> QueryResult<i64> {
    supplychain::table.count().get_result(conn)
}

pub fn list_supplychains_with_paging(
    conn: &PgConnection,
    limit: i64,
    offset: i64,
) -> QueryResult<Vec<Supplychain>> {
    supplychain::table
        .select(supplychain::all_columns)
        .limit(limit)
        .offset(offset)
        .load::<Supplychain>(conn)
}

pub fn fetch_supplychain(conn: &PgConnection, circuit_id: &str) -> QueryResult<Option<Supplychain>> {
    supplychain::table
        .filter(supplychain::circuit_id.eq(circuit_id))
        .first(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn fetch_supplychain_by_alias(conn: &PgConnection, alias: &str) -> QueryResult<Option<Supplychain>> {
    supplychain::table
        .filter(supplychain::alias.eq(alias))
        .first(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn fetch_service_id_for_supplychain_service(
    conn: &PgConnection,
    circuit_id: &str,
    node_id: &str,
) -> QueryResult<Option<String>> {
    supplychain_service::table
        .filter(
            supplychain_service::circuit_id
                .eq(circuit_id)
                .and(supplychain_service::allowed_nodes.contains(vec![node_id])),
        )
        .first::<SupplychainService>(conn)
        .map(|service| Some(service.service_id))
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}
