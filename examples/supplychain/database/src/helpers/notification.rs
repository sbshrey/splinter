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

use crate::models::{SupplychainNotification, NewSupplychainNotification};
use crate::schema::supplychain_notification;
use diesel::{
    dsl::insert_into, pg::PgConnection, prelude::*, result::Error::NotFound, QueryResult,
};

pub fn fetch_notification(
    conn: &PgConnection,
    notification_id: i64,
) -> QueryResult<Option<SupplychainNotification>> {
    supplychain_notification::table
        .filter(supplychain_notification::id.eq(notification_id))
        .first::<SupplychainNotification>(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn list_unread_notifications_with_paging(
    conn: &PgConnection,
    limit: i64,
    offset: i64,
) -> QueryResult<Vec<SupplychainNotification>> {
    supplychain_notification::table
        .select(supplychain_notification::all_columns)
        .filter(supplychain_notification::read.eq(false))
        .limit(limit)
        .offset(offset)
        .load::<SupplychainNotification>(conn)
}

pub fn update_supplychain_notification(
    conn: &PgConnection,
    notification_id: i64,
) -> QueryResult<Option<SupplychainNotification>> {
    diesel::update(supplychain_notification::table.find(notification_id))
        .set(supplychain_notification::read.eq(true))
        .get_result(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn insert_supplychain_notification(
    conn: &PgConnection,
    notifications: &[NewSupplychainNotification],
) -> QueryResult<()> {
    insert_into(supplychain_notification::table)
        .values(notifications)
        .execute(conn)
        .map(|_| ())
}

pub fn create_new_notification(
    notification_type: &str,
    requester: &str,
    requester_node_id: &str,
    target: &str,
) -> NewSupplychainNotification {
    NewSupplychainNotification {
        notification_type: notification_type.to_string(),
        requester: requester.to_string(),
        requester_node_id: requester_node_id.to_string(),
        target: target.to_string(),
        created_time: SystemTime::now(),
        read: false,
    }
}

pub fn get_unread_notification_count(conn: &PgConnection) -> QueryResult<i64> {
    supplychain_notification::table
        .filter(supplychain_notification::read.eq(false))
        .count()
        .get_result(conn)
}

pub fn fetch_notifications_by_time(
    conn: &PgConnection,
    current_check_time: SystemTime,
    previous_check_time: SystemTime,
) -> QueryResult<Vec<SupplychainNotification>> {
    supplychain_notification::table
        .filter(
            supplychain_notification::created_time
                .ge(previous_check_time)
                .and(supplychain_notification::created_time.le(current_check_time)),
        )
        .load::<SupplychainNotification>(conn)
}
