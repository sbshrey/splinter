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

use crate::models::SupplychainUser;
use crate::schema::supplychain_user;

use diesel::{
    dsl::insert_into, pg::PgConnection, prelude::*, result::Error::NotFound, QueryResult,
};

pub fn fetch_user_by_email(conn: &PgConnection, email: &str) -> QueryResult<Option<SupplychainUser>> {
    supplychain_user::table
        .find(email)
        .first::<SupplychainUser>(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn insert_user(conn: &PgConnection, user: SupplychainUser) -> QueryResult<()> {
    insert_into(supplychain_user::table)
        .values(&vec![user])
        .execute(conn)
        .map(|_| ())
}
