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

use std::error::Error;
use std::fmt;

use sawtooth_sdk::signing::Error as KeyGenError;

use crate::authorization_handler::AppAuthHandlerError;
use crate::rest_api::RestApiServerError;
use supplychain_database::DatabaseError;

#[derive(Debug)]
pub enum SupplychainDaemonError {
    LoggingInitializationError(flexi_logger::FlexiLoggerError),
    ConfigurationError(Box<ConfigurationError>),
    DatabaseError(Box<DatabaseError>),
    RestApiError(RestApiServerError),
    AppAuthHandlerError(AppAuthHandlerError),
    KeyGenError(KeyGenError),
    GetNodeError(GetNodeError),
}

impl Error for SupplychainDaemonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SupplychainDaemonError::LoggingInitializationError(err) => Some(err),
            SupplychainDaemonError::ConfigurationError(err) => Some(err),
            SupplychainDaemonError::DatabaseError(err) => Some(&**err),
            SupplychainDaemonError::RestApiError(err) => Some(err),
            SupplychainDaemonError::AppAuthHandlerError(err) => Some(err),
            SupplychainDaemonError::KeyGenError(err) => Some(err),
            SupplychainDaemonError::GetNodeError(err) => Some(err),
        }
    }
}

impl fmt::Display for SupplychainDaemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SupplychainDaemonError::LoggingInitializationError(e) => {
                write!(f, "Logging initialization error: {}", e)
            }
            SupplychainDaemonError::ConfigurationError(e) => write!(f, "Coniguration error: {}", e),
            SupplychainDaemonError::DatabaseError(e) => write!(f, "Database error: {}", e),
            SupplychainDaemonError::RestApiError(e) => write!(f, "Rest API error: {}", e),
            SupplychainDaemonError::AppAuthHandlerError(e) => write!(
                f,
                "The application authorization handler returned an error: {}",
                e
            ),
            SupplychainDaemonError::KeyGenError(e) => write!(
                f,
                "an error occurred while generating a new key pair: {}",
                e
            ),
            SupplychainDaemonError::GetNodeError(e) => write!(
                f,
                "an error occurred while getting splinterd node information: {}",
                e
            ),
        }
    }
}

impl From<flexi_logger::FlexiLoggerError> for SupplychainDaemonError {
    fn from(err: flexi_logger::FlexiLoggerError) -> SupplychainDaemonError {
        SupplychainDaemonError::LoggingInitializationError(err)
    }
}

impl From<DatabaseError> for SupplychainDaemonError {
    fn from(err: DatabaseError) -> SupplychainDaemonError {
        SupplychainDaemonError::DatabaseError(Box::new(err))
    }
}

impl From<RestApiServerError> for SupplychainDaemonError {
    fn from(err: RestApiServerError) -> SupplychainDaemonError {
        SupplychainDaemonError::RestApiError(err)
    }
}

impl From<AppAuthHandlerError> for SupplychainDaemonError {
    fn from(err: AppAuthHandlerError) -> SupplychainDaemonError {
        SupplychainDaemonError::AppAuthHandlerError(err)
    }
}

impl From<KeyGenError> for SupplychainDaemonError {
    fn from(err: KeyGenError) -> SupplychainDaemonError {
        SupplychainDaemonError::KeyGenError(err)
    }
}

#[derive(Debug, PartialEq)]
pub enum ConfigurationError {
    MissingValue(String),
}

impl Error for ConfigurationError {}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigurationError::MissingValue(config_field_name) => {
                write!(f, "Missing configuration for {}", config_field_name)
            }
        }
    }
}

impl From<ConfigurationError> for SupplychainDaemonError {
    fn from(err: ConfigurationError) -> Self {
        SupplychainDaemonError::ConfigurationError(Box::new(err))
    }
}

#[derive(Debug, PartialEq)]
pub struct GetNodeError(pub String);

impl Error for GetNodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for GetNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<GetNodeError> for SupplychainDaemonError {
    fn from(err: GetNodeError) -> Self {
        SupplychainDaemonError::GetNodeError(err)
    }
}
