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

//! Provides functionality to set the `metadata` field of a `CreateCircuitBuilder`.

use super::super::{yaml_parser::v1, CircuitTemplateError};
use super::{get_argument_value, is_arg_value, RuleArgument, Value};

/// Data structure wrapping the `Metadata` object to be used to fill in the `metadata` field of the
/// `CreateCircuitBuilder`.
pub(super) struct SetMetadata {
    metadata: Metadata,
}

impl SetMetadata {
    pub fn apply_rule(
        &self,
        template_arguments: &[RuleArgument],
    ) -> Result<Vec<u8>, CircuitTemplateError> {
        match &self.metadata {
            Metadata::Json { metadata } => {
                let metadata = metadata
                    .iter()
                    .map(|metadata| match &metadata.value {
                        Value::Single(value) => {
                            let value = if is_arg_value(&value) {
                                get_argument_value(&value, &template_arguments)?
                            } else {
                                value.to_string()
                            };
                            Ok(format!("\"{}\":\"{}\"", metadata.key, value))
                        }
                        Value::List(values) => {
                            let processed_values = values
                                .iter()
                                .map(|value| {
                                    let value = if is_arg_value(&value) {
                                        get_argument_value(&value, &template_arguments)?
                                    } else {
                                        value.to_string()
                                    };
                                    Ok(format!("\"{}\"", value))
                                })
                                .collect::<Result<Vec<String>, CircuitTemplateError>>()?;

                            Ok(format!(
                                "\"{}\":[{}]",
                                metadata.key,
                                processed_values.join(",")
                            ))
                        }
                    })
                    .collect::<Result<Vec<String>, CircuitTemplateError>>()?;

                let json_metadata = format!("{{{}}}", metadata.join(","));

                Ok(json_metadata.as_bytes().to_vec())
            }
        }
    }
}

impl From<v1::SetMetadata> for SetMetadata {
    fn from(set_metadata: v1::SetMetadata) -> Self {
        SetMetadata {
            metadata: Metadata::from(set_metadata.metadata().clone()),
        }
    }
}

/// Enum of the possible types of `Metadata` representations.
pub(super) enum Metadata {
    Json { metadata: Vec<JsonMetadata> },
}

impl From<v1::Metadata> for Metadata {
    fn from(metadata: v1::Metadata) -> Self {
        match metadata {
            v1::Metadata::Json { metadata } => Metadata::Json {
                metadata: metadata.into_iter().map(JsonMetadata::from).collect(),
            },
        }
    }
}

/// Data structure of the JSON representation of `metadata`.
pub(super) struct JsonMetadata {
    key: String,
    value: Value,
}

impl From<v1::JsonMetadata> for JsonMetadata {
    fn from(metadata: v1::JsonMetadata) -> Self {
        JsonMetadata {
            key: metadata.key().to_string(),
            value: Value::from(metadata.value().clone()),
        }
    }
}
