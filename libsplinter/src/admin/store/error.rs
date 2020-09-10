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

//! Types for errors that can be raised while using an admin service store, as well as errors
//! raised by the builders

use std::error::Error;
use std::fmt;

/// Represents AdminServiceStore errors
#[derive(Debug)]
pub enum AdminServiceStoreError {
    /// Represents CRUD operations failures
    OperationError {
        context: String,
        source: Option<Box<dyn Error>>,
    },
    /// Represents store query failures
    QueryError {
        context: String,
        source: Box<dyn Error>,
    },
    /// Represents general failures in the store
    StorageError {
        context: String,
        source: Option<Box<dyn Error>>,
    },
    /// Represents an issue connecting to the store
    ConnectionError(Box<dyn Error>),
    NotFoundError(String),
}

impl Error for AdminServiceStoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AdminServiceStoreError::OperationError {
                source: Some(source),
                ..
            } => Some(&**source),
            AdminServiceStoreError::OperationError { source: None, .. } => None,
            AdminServiceStoreError::QueryError { source, .. } => Some(&**source),
            AdminServiceStoreError::StorageError {
                source: Some(source),
                ..
            } => Some(&**source),
            AdminServiceStoreError::StorageError { source: None, .. } => None,
            AdminServiceStoreError::ConnectionError(err) => Some(&**err),
            AdminServiceStoreError::NotFoundError(_) => None,
        }
    }
}

impl fmt::Display for AdminServiceStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdminServiceStoreError::OperationError {
                context,
                source: Some(source),
            } => write!(f, "failed to perform operation: {}: {}", context, source),
            AdminServiceStoreError::OperationError {
                context,
                source: None,
            } => write!(f, "failed to perform operation: {}", context),
            AdminServiceStoreError::QueryError { context, source } => {
                write!(f, "failed query: {}: {}", context, source)
            }
            AdminServiceStoreError::StorageError {
                context,
                source: Some(source),
            } => write!(
                f,
                "the underlying storage returned an error: {}: {}",
                context, source
            ),
            AdminServiceStoreError::StorageError {
                context,
                source: None,
            } => write!(f, "the underlying storage returned an error: {}", context),
            AdminServiceStoreError::ConnectionError(err) => {
                write!(f, "failed to connect to underlying storage: {}", err)
            }
            AdminServiceStoreError::NotFoundError(ref s) => write!(f, "Not found: {}", s),
        }
    }
}

/// Represents errors raised while building
#[derive(Debug)]
pub enum BuilderError {
    InvalidField(String),
    MissingField(String),
}

impl Error for BuilderError {}

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            BuilderError::InvalidField(ref s) => write!(f, "unable to build, invalid field: {}", s),
            BuilderError::MissingField(ref s) => write!(f, "unable to build, missing field: {}", s),
        }
    }
}
