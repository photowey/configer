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
use std::path::Path;

use crate::domain::{merge_tables, Table};
use crate::env::{DynamicEnvironment, Environment, Node, try_load_env_variables};
use crate::error::{ConfigerError, FileError};
use crate::reader::{ConfigReader, ConfigReaderRegistry, ReaderRegistry};
#[cfg(feature = "usetoml")]
use crate::reader::toml::TomlConfigReader;

// ----------------------------------------------------------------

const DOT: char = '.';

// ----------------------------------------------------------------

pub struct ConfigerEnvironment {
    ctx: Table,
    registry: Option<Box<dyn ReaderRegistry>>,
}


// ----------------------------------------------------------------

impl ConfigerEnvironment {
    #[cfg(not(feature = "usetoml"))]
    pub fn new() -> Self {
        let env_table = try_load_env_variables();
        Self::mixed(Some(env_table), Some(Box::<ConfigReaderRegistry>::default()))
    }

    #[cfg(feature = "usetoml")]
    pub fn new() -> Self {
        let env_table = try_load_env_variables();
        let mut configer = Self::mixed(Some(env_table), Some(Box::<ConfigReaderRegistry>::default()));

        configer.register_toml_reader();
        configer
    }

    pub fn mixed(table_opt: Option<Table>, registry: Option<Box<dyn ReaderRegistry>>) -> Self {
        if let Some(table) = table_opt {
            return Self {
                ctx: table,
                registry,
            };
        }

        let env_table = try_load_env_variables();
        Self {
            ctx: env_table,
            registry,
        }
    }

    pub fn mixed_with_env_variables(table_opt: Option<Table>, registry: Option<Box<dyn ReaderRegistry>>) -> Self {
        if let Some(table) = table_opt {
            let env_table = try_load_env_variables();
            let merged_table = merge_tables(table, env_table);
            return Self {
                ctx: merged_table,
                registry,
            };
        }

        let env_table = try_load_env_variables();
        Self {
            ctx: env_table,
            registry,
        }
    }

    /// @since 0.3.0
    #[deprecated(since = "0.4.0", note = "use `table()` instead")]
    pub fn build(table: Table) -> Self {
        Self::table(table)
    }

    /// @since 0.4.0
    #[cfg(not(feature = "usetoml"))]
    pub fn table(table: Table) -> Self {
        Self::mixed_with_env_variables(Some(table), Some(Box::<ConfigReaderRegistry>::default()))
    }

    /// @since 0.4.0
    #[cfg(feature = "usetoml")]
    pub fn table(table: Table) -> Self {
        let mut configer = Self::mixed_with_env_variables(Some(table), Some(Box::<ConfigReaderRegistry>::default()));

        configer.register_toml_reader();
        configer
    }

    /// @since 0.4.0
    #[deprecated(since = "0.4.0", note = "use `ConfigerEnvironmentBuilder` instead")]
    pub fn register_table(&mut self, table: Table) {
        self.ctx = table;
    }

    /// @since 0.5.0
    pub fn register_table_with_env_variables(&mut self, table: Table) {
        let env_table = try_load_env_variables();
        let merged_table = merge_tables(table, env_table);

        self.ctx = merged_table;
    }

    /// @since 0.4.0
    pub fn merge_table(&mut self, table: Table) {
        self.ctx = merge_tables(self.ctx.clone(), table)
    }

    /// @since 0.4.0
    #[cfg(feature = "usetoml")]
    fn register_toml_reader(&mut self) {
        if let Some(ref mut registry) = self.registry {
            registry.register(Box::<TomlConfigReader>::default())
        }
    }
}

impl ConfigerEnvironment {
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

        for (index, sentinel) in keys.iter().enumerate() {
            let key = (*sentinel).to_string();
            match node_ref.get(&key) {
                Some(next_node) => {
                    if index == keys.len() - 1 {
                        return Ok(next_node);
                    }

                    match next_node {
                        Node::Nested(nested) => {
                            node_ref = nested;
                        }
                        _ => return Err(ConfigerError::NonNested),
                    }
                }
                None => return Err(ConfigerError::NotFound),
            }
        }

        Err(ConfigerError::NotFound)
    }
}

// ----------------------------------------------------------------

impl ConfigerEnvironment {
    pub fn builder() -> ConfigerEnvironmentBuilder {
        ConfigerEnvironmentBuilder::default()
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

    fn try_acquire(&self, name: &str) -> Option<&dyn ConfigReader> {
        if let Some(ref registry) = self.registry {
            registry.try_acquire(name)
        } else {
            None
        }
    }

    fn try_acquires(&self) -> Vec<&dyn ConfigReader> {
        if let Some(ref registry) = self.registry {
            return registry.try_acquires();
        }

        Vec::new()
    }
}

// ----------------------------------------------------------------

impl DynamicEnvironment for ConfigerEnvironment {}

// ----------------------------------------------------------------

pub struct ConfigerEnvironmentBuilder {
    table: Option<Table>,
    registry: Option<Box<dyn ReaderRegistry>>,
    path: Option<String>,
}

impl ConfigerEnvironmentBuilder {
    pub fn new() -> Self {
        Self {
            table: None,
            registry: None,
            path: None,
        }
    }

    pub fn with_table(mut self, table: Table) -> Self {
        self.table = Some(table);
        self
    }

    pub fn with_registry(mut self, registry: Box<dyn ReaderRegistry>) -> Self {
        self.registry = Some(registry);
        self
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    pub fn build(self) -> Result<ConfigerEnvironment, FileError> {
        match (self.table, self.registry, self.path) {
            // @since 0.5.0
            (Some(table_outer), Some(registry), Some(path)) => {
                let file_path = Path::new(&path);

                if let Some(extension) = file_path.to_str().and_then(|name| Path::new(name).extension()) {
                    let suffix = extension.to_string_lossy();

                    if let Some(reader) = registry.try_acquire(suffix.as_ref()) {
                        let rvt = reader.read_from_path(&path[..]);

                        if let Ok(table) = rvt {
                            let merged_table = merge_tables(table, table_outer);
                            return Ok(ConfigerEnvironment::mixed_with_env_variables(Some(merged_table), Some(registry)));
                        }

                        return Err(FileError::ReaderNotFound(suffix.to_string()));
                    } // end of registry.try_acquire

                    return Err(FileError::ReaderNotFound(suffix.to_string()));
                } // end of and_then

                Err(FileError::InvalidFile(path))
            }
            (None, Some(registry), Some(path)) => {
                let file_path = Path::new(&path);

                if let Some(extension) = file_path.to_str().and_then(|name| Path::new(name).extension()) {
                    let suffix = extension.to_string_lossy();

                    if let Some(reader) = registry.try_acquire(suffix.as_ref()) {
                        let rvt = reader.read_from_path(&path[..]);

                        if let Ok(table) = rvt {
                            return Ok(ConfigerEnvironment::mixed_with_env_variables(Some(table), Some(registry)));
                        }

                        return Err(FileError::ReaderNotFound(suffix.to_string()));
                    } // end of registry.try_acquire

                    return Err(FileError::ReaderNotFound(suffix.to_string()));
                } // end of and_then

                Err(FileError::InvalidFile(path))
            }
            (Some(table), Some(registry), None) => {
                Ok(ConfigerEnvironment::mixed(Some(table), Some(registry)))
            }
            (Some(table), None, None) => {
                Ok(ConfigerEnvironment::mixed(Some(table), None))
            }
            _ => {
                Ok(ConfigerEnvironment::new())
            }
        }
    }
}

// ----------------------------------------------------------------
impl Default for ConfigerEnvironmentBuilder {
    fn default() -> Self {
        ConfigerEnvironmentBuilder::new()
    }
}
