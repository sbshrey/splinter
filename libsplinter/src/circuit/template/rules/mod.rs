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

//! Data structures to hold necessary information for setting the values in the builders generated
//! from a circuit template. Also provides the general functionality to apply the circuit template
//! `rules`.

mod create_services;
mod set_management_type;
mod set_metadata;

use std::convert::TryFrom;

use super::{yaml_parser::v1, CircuitTemplateError, CreateCircuitBuilder};

use create_services::CreateServices;
use set_management_type::CircuitManagement;
use set_metadata::SetMetadata;

/// Available `rules` used to create the value for entries of a builder, based on the circuit
/// template arguments that have been set.
pub struct Rules {
    set_management_type: Option<CircuitManagement>,
    create_services: Option<CreateServices>,
    set_metadata: Option<SetMetadata>,
}

impl Rules {
    /// Applies all available `Rules` for the circuit template. This updates all builders,
    /// including the `SplinterServiceBuilder` objects and `CreateCircuitBuilder`.
    pub fn apply_rules(
        &self,
        mut circuit_builder: CreateCircuitBuilder,
        template_arguments: &[RuleArgument],
    ) -> Result<CreateCircuitBuilder, CircuitTemplateError> {
        if let Some(circuit_management) = &self.set_management_type {
            circuit_builder =
                circuit_builder.with_circuit_management_type(&circuit_management.apply_rule()?);
        }

        if let Some(create_services) = &self.create_services {
            let service_builders = create_services.apply_rule(template_arguments)?;
            let mut services = vec![];
            for service_builder in service_builders {
                match service_builder.build() {
                    Ok(service) => services.push(service),
                    Err(err) => {
                        return Err(CircuitTemplateError::new_with_source(
                            "Failed to build SplinterService: {}",
                            Box::new(err),
                        ));
                    }
                }
            }
            circuit_builder = circuit_builder.with_roster(&services);
        }

        if let Some(set_metadata) = &self.set_metadata {
            circuit_builder = circuit_builder
                .with_application_metadata(&set_metadata.apply_rule(template_arguments)?);
        }

        Ok(circuit_builder)
    }
}

impl From<v1::Rules> for Rules {
    fn from(rules: v1::Rules) -> Self {
        Rules {
            set_management_type: rules
                .set_management_type()
                .map(|val| CircuitManagement::from(val.clone())),
            create_services: rules
                .create_services()
                .map(|val| CreateServices::from(val.clone())),
            set_metadata: rules
                .set_metadata()
                .map(|val| SetMetadata::from(val.clone())),
        }
    }
}

/// Data structure to hold an argument used by a rule.
#[derive(Clone)]
pub struct RuleArgument {
    name: String,
    /// Represents whether the argument itself is required.
    required: bool,
    default_value: Option<String>,
    description: Option<String>,
    /// Value specified by the user.
    user_value: Option<String>,
}

impl RuleArgument {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn default_value(&self) -> Option<&String> {
        self.default_value.as_ref()
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn user_value(&self) -> Option<&String> {
        self.user_value.as_ref()
    }

    pub fn set_user_value(&mut self, value: &str) {
        self.user_value = Some(value.to_string())
    }
}

impl TryFrom<v1::RuleArgument> for RuleArgument {
    type Error = CircuitTemplateError;
    fn try_from(arguments: v1::RuleArgument) -> Result<Self, Self::Error> {
        Ok(RuleArgument {
            name: arguments.name().to_lowercase(),
            required: arguments.required(),
            default_value: arguments.default_value().map(String::from),
            description: arguments.description().map(String::from),
            user_value: None,
        })
    }
}

fn is_arg_value(key: &str) -> bool {
    key.starts_with("$(")
}

fn strip_arg_marker(key: &str) -> String {
    if key.starts_with("$(") && key.ends_with(')') {
        let mut key = key.to_string();
        key.pop();
        key.trim_start_matches("$(").to_string().to_lowercase()
    } else {
        key.to_string().to_lowercase()
    }
}

#[derive(Debug)]
enum Value {
    Single(String),
    List(Vec<String>),
}

impl From<v1::Value> for Value {
    fn from(value: v1::Value) -> Self {
        match value {
            v1::Value::Single(value) => Self::Single(value),
            v1::Value::List(values) => Self::List(values),
        }
    }
}

fn get_argument_value(
    key: &str,
    template_arguments: &[RuleArgument],
) -> Result<String, CircuitTemplateError> {
    let key = strip_arg_marker(key);
    let value = match template_arguments.iter().find(|arg| arg.name == key) {
        Some(arg) => match arg.user_value() {
            Some(val) => val.to_string(),
            None => {
                if arg.required {
                    return Err(CircuitTemplateError::new(&format!(
                        "Argument \"{}\" is required but was not provided",
                        key
                    )));
                } else {
                    let default_value = arg.default_value.to_owned().ok_or_else(|| {
                        CircuitTemplateError::new(&format!(
                            "Argument \"{}\" was not provided and no default value is set",
                            key
                        ))
                    })?;
                    if is_arg_value(&default_value) {
                        get_argument_value(&default_value, template_arguments)?
                    } else {
                        default_value
                    }
                }
            }
        },
        None => {
            return Err(CircuitTemplateError::new(&format!(
                "Invalid template. Argument \"{}\" was expected but not provided",
                key
            )));
        }
    };

    Ok(value)
}
