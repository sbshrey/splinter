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

mod supplychain;
mod supplychain_user;
mod notification;
mod xo_games;

pub use supplychain::{
    fetch_active_supplychains, fetch_supplychain, fetch_supplychain_by_alias,
    fetch_supplychain_members_by_circuit_id_and_status, fetch_supplychain_proposal_with_status,
    fetch_proposal_by_id, fetch_service_id_for_supplychain_service, supplychain_service_is_active,
    get_supplychain_count, get_last_updated_proposal_time, get_proposal_count, insert_supplychain,
    insert_supplychain_members, insert_supplychain_proposal, insert_supplychain_services,
    insert_proposal_vote_record, list_supplychain_members_with_status, list_supplychains_with_paging,
    list_supplychains_with_paging_and_status, list_proposals_with_paging,
    update_supplychain_member_status, update_supplychain_proposal_status,
    update_supplychain_service_last_event, update_supplychain_service_status, update_supplychain_status,
};
pub use supplychain_user::{fetch_user_by_email, insert_user};
pub use notification::{
    create_new_notification, fetch_notification, fetch_notifications_by_time,
    get_unread_notification_count, insert_supplychain_notification,
    list_unread_notifications_with_paging, update_supplychain_notification,
};
pub use xo_games::{
    fetch_xo_game, get_xo_game_count, insert_xo_game, list_xo_games, update_xo_game,
};
