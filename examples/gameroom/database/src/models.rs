/*
 * Copyright 2018-2020 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

use super::schema::*;
use std::time::SystemTime;

#[derive(Insertable, Queryable)]
#[table_name = "supplychain_user"]
pub struct SupplychainUser {
    pub email: String,
    pub public_key: String,
    pub encrypted_private_key: String,
    pub hashed_password: String,
}

#[derive(Insertable, Queryable, Identifiable, PartialEq, Debug)]
#[table_name = "supplychain"]
#[primary_key(circuit_id)]
pub struct Supplychain {
    pub circuit_id: String,
    pub authorization_type: String,
    pub persistence: String,
    pub durability: String,
    pub routes: String,
    pub circuit_management_type: String,
    pub alias: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "supplychain_proposal"]
#[belongs_to(Supplychain, foreign_key = "circuit_id")]
pub struct SupplychainProposal {
    pub id: i64,
    pub proposal_type: String,
    pub circuit_id: String,
    pub circuit_hash: String,
    pub requester: String,
    pub requester_node_id: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "supplychain_proposal"]
pub struct NewSupplychainProposal {
    pub proposal_type: String,
    pub circuit_id: String,
    pub circuit_hash: String,
    pub requester: String,
    pub requester_node_id: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "proposal_vote_record"]
#[belongs_to(SupplychainProposal, foreign_key = "proposal_id")]
pub struct ProposalVoteRecord {
    pub id: i64,
    pub proposal_id: i64,
    pub voter_public_key: String,
    pub voter_node_id: String,
    pub vote: String,
    pub created_time: SystemTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "proposal_vote_record"]
pub struct NewProposalVoteRecord {
    pub proposal_id: i64,
    pub voter_public_key: String,
    pub voter_node_id: String,
    pub vote: String,
    pub created_time: SystemTime,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "supplychain_member"]
#[belongs_to(Supplychain, foreign_key = "circuit_id")]
pub struct SupplychainMember {
    pub id: i64,
    pub circuit_id: String,
    pub node_id: String,
    pub endpoints: Vec<String>,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "supplychain_member"]
pub struct NewSupplychainMember {
    pub circuit_id: String,
    pub node_id: String,
    pub endpoints: Vec<String>,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "supplychain_service"]
#[belongs_to(Supplychain, foreign_key = "circuit_id")]
pub struct SupplychainService {
    pub id: i64,
    pub circuit_id: String,
    pub service_id: String,
    pub service_type: String,
    pub allowed_nodes: Vec<String>,
    pub arguments: Vec<serde_json::Value>,
    pub status: String,
    pub last_event: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "supplychain_service"]
pub struct NewSupplychainService {
    pub circuit_id: String,
    pub service_id: String,
    pub service_type: String,
    pub allowed_nodes: Vec<String>,
    pub arguments: Vec<serde_json::Value>,
    pub status: String,
    pub last_event: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "supplychain_notification"]
pub struct SupplychainNotification {
    pub id: i64,
    pub notification_type: String,
    pub requester: String,
    pub requester_node_id: String,
    pub target: String,
    pub created_time: SystemTime,
    pub read: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "supplychain_notification"]
pub struct NewSupplychainNotification {
    pub notification_type: String,
    pub requester: String,
    pub requester_node_id: String,
    pub target: String,
    pub created_time: SystemTime,
    pub read: bool,
}

#[derive(Clone, Queryable, Identifiable, Associations, Insertable, AsChangeset)]
#[table_name = "xo_games"]
pub struct XoGame {
    pub id: i64,
    pub circuit_id: String,
    pub game_name: String,
    pub player_1: String,
    pub player_2: String,
    pub game_status: String,
    pub game_board: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Debug, Insertable)]
#[table_name = "xo_games"]
pub struct NewXoGame {
    pub circuit_id: String,
    pub game_name: String,
    pub player_1: String,
    pub player_2: String,
    pub game_status: String,
    pub game_board: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct ActiveSupplychain {
    pub circuit_id: String,
    pub service_id: String,
    pub status: String,
    pub last_event: String,
    pub requester: String,
    pub requester_node_id: String,
}
