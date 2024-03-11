/*
 * Copyright © 2024 the original author or authors.
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
 */

// ----------------------------------------------------------------

use std::collections::HashMap;

use crate::domain::Table;
use crate::env::{DynamicEnvironment, Environment, Node};
use crate::error::ConfigerError;

// ----------------------------------------------------------------

const DOT: char = '.';

// ----------------------------------------------------------------

pub struct ConfigerEnvironment {
    ctx: Table,
}

impl ConfigerEnvironment {
    pub fn new() -> Self {
        Self {
            ctx: HashMap::new(),
        }
    }

    fn set_nested_recursive(
        node_ref: &mut Table,
        keys: Vec<&str>,
        value: Node,
    ) -> Result<(), ConfigerError> {
        if let Some(sentinel) = keys.first() {
            let key = (*sentinel).to_string();

            if keys.len() > 1 {
                let nested = node_ref
                    .entry(key.clone())
                    .or_insert(Node::Nested(HashMap::new()));
                return if let Node::Nested(nested_ref) = nested {
                    Self::set_nested_recursive(nested_ref, keys[1..].to_vec(), value)
                } else {
                    Err(ConfigerError::NonNested)
                };
            }

            node_ref.insert(key, value);
        }

        Ok(())
    }
}

// ----------------------------------------------------------------

impl ConfigerEnvironment {
    fn set_nested(&mut self, keys: Vec<&str>, value: Node) -> Result<(), ConfigerError> {
        if keys.is_empty() {
            return Err(ConfigerError::EmptyKey);
        }

        Self::set_nested_recursive(&mut self.ctx, keys, value)?;

        Ok(())
    }

    fn get_nested(&self, keys: Vec<&str>) -> Result<&Node, ConfigerError> {
        if keys.is_empty() {
            return Err(ConfigerError::EmptyKey);
        }

        let mut node_ref = &self.ctx;

        for key in keys {
            node_ref = match node_ref.get(key) {
                Some(Node::Nested(ref nested)) => nested,
                Some(node) => return Ok(node),
                None => return Err(ConfigerError::NotFound),
            };
        }

        Err(ConfigerError::NotFound)
    }
}

// ----------------------------------------------------------------

impl Default for ConfigerEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

// ----------------------------------------------------------------

impl Environment for ConfigerEnvironment {
    fn set(&mut self, key: &str, value: Node) -> Result<(), ConfigerError> {
        if key.is_empty() {
            return Err(ConfigerError::EmptyKey);
        }

        let keys: Vec<&str> = key.split(DOT).collect();
        self.set_nested(keys, value)
    }

    fn get(&self, key: &str) -> Result<&Node, ConfigerError> {
        let keys: Vec<&str> = key.split(DOT).collect();
        self.get_nested(keys)
    }
}

// ----------------------------------------------------------------

impl DynamicEnvironment for ConfigerEnvironment {}
