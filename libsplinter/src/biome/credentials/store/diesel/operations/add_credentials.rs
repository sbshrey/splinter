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

use super::CredentialsStoreOperations;
use crate::biome::credentials::store::diesel::{
    schema::user_credentials, Credentials, CredentialsStoreError,
};
use crate::biome::credentials::store::{CredentialsModel, NewCredentialsModel};
use diesel::{dsl::insert_into, prelude::*, result::Error::NotFound};

pub(in crate::biome::credentials) trait CredentialsStoreAddCredentialsOperation {
    fn add_credentials(&self, credentials: Credentials) -> Result<(), CredentialsStoreError>;
}

#[cfg(feature = "postgres")]
impl<'a> CredentialsStoreAddCredentialsOperation
    for CredentialsStoreOperations<'a, diesel::pg::PgConnection>
{
    fn add_credentials(&self, credentials: Credentials) -> Result<(), CredentialsStoreError> {
        let duplicate_credentials = user_credentials::table
            .filter(user_credentials::username.eq(&credentials.username))
            .first::<CredentialsModel>(self.conn)
            .map(Some)
            .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
            .map_err(|err| CredentialsStoreError::QueryError {
                context: "Failed check for existing username".to_string(),
                source: Box::new(err),
            })?;
        if duplicate_credentials.is_some() {
            return Err(CredentialsStoreError::DuplicateError(format!(
                "Username already in use: {}",
                &credentials.username
            )));
        }

        let new_credentials: NewCredentialsModel = credentials.into();

        insert_into(user_credentials::table)
            .values(new_credentials)
            .execute(self.conn)
            .map(|_| ())
            .map_err(|err| CredentialsStoreError::OperationError {
                context: "Failed to add credentials".to_string(),
                source: Box::new(err),
            })?;
        Ok(())
    }
}

#[cfg(feature = "sqlite")]
impl<'a> CredentialsStoreAddCredentialsOperation
    for CredentialsStoreOperations<'a, diesel::sqlite::SqliteConnection>
{
    fn add_credentials(&self, credentials: Credentials) -> Result<(), CredentialsStoreError> {
        let duplicate_credentials = user_credentials::table
            .filter(user_credentials::username.eq(&credentials.username))
            .first::<CredentialsModel>(self.conn)
            .map(Some)
            .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
            .map_err(|err| CredentialsStoreError::QueryError {
                context: "Failed check for existing username".to_string(),
                source: Box::new(err),
            })?;
        if duplicate_credentials.is_some() {
            return Err(CredentialsStoreError::DuplicateError(format!(
                "Username already in use: {}",
                &credentials.username
            )));
        }

        let new_credentials: NewCredentialsModel = credentials.into();

        insert_into(user_credentials::table)
            .values(new_credentials)
            .execute(self.conn)
            .map(|_| ())
            .map_err(|err| CredentialsStoreError::OperationError {
                context: "Failed to add credentials".to_string(),
                source: Box::new(err),
            })?;
        Ok(())
    }
}
