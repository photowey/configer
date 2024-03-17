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

use std::{env, fs};
use std::collections::HashMap;

use toml::Value;

use crate::{domain, reader};
use crate::domain::{Node, Table};
use crate::domain::converter::NodeConverter;
use crate::env::Environment;
use crate::env::standard::ConfigerEnvironment;
use crate::error::ConfigerError;
use crate::reader::{ConfigReader, ConfigReaderRegistry, ReaderRegistry};
use crate::reader::toml::TomlConfigReader;

// ----------------------------------------------------------------

#[test]
fn test_toml_read() {
    let path = "resources/testdata/configer-dev.toml";

    let toml_str = fs::read_to_string(path)
        .expect("Failed to read configer-dev.toml file");

    let parsed_toml: Value = toml::from_str(&toml_str)
        .expect("Failed to parse configer-dev.toml file");

    traverse_toml(&parsed_toml)
}

#[test]
fn test_toml_value_to_node() {
    let path = "resources/testdata/configer-dev.toml";

    let toml_str = fs::read_to_string(path)
        .expect("Failed to read configer-dev.toml file");

    let parsed_toml: Value = toml::from_str(&toml_str)
        .expect("Failed to parse configer-dev.toml file");

    let mut hashmap: HashMap<String, Node> = HashMap::new();

    match parsed_toml {
        Value::Table(table) => {
            for (key, value) in table {
                hashmap.insert(key, reader::toml::toml_value_to_node(value));
            }

            return ();
        }
        _ => panic!("Incorrect TOML format: Missing table data.")
    }
}

// ----------------------------------------------------------------

#[test]
fn test_toml_reader_read_from_path() {
    let toml_reader = TomlConfigReader::default();

    let path = "resources/testdata/configer-dev.toml";
    let toml_from_path_rvt = toml_reader.read_from_path(path);

    if let Ok(table) = toml_from_path_rvt {
        return assert_table(table);
    }

    panic!("Failed to read config file")
}

#[test]
fn test_toml_reader_read_from_str() {
    let toml_reader = TomlConfigReader::default();

    let path = "resources/testdata/configer-dev.toml";
    let content = fs::read_to_string(path).expect("Failed to read config file");
    let toml_from_content_rvt = toml_reader.read_from_str(&content);

    if let Ok(table) = toml_from_content_rvt {
        return assert_table(table);
    }

    panic!("Failed to read config file")
}

fn assert_table(table: Table) {
    assert!(table.contains_key("string_value"));
    assert!(table.contains_key("floats"));
    assert!(table.contains_key("table"));
    assert!(table.contains_key("database"));
}

// ----------------------------------------------------------------

#[test]
fn test_build_configer_by_table() {
    let path = "resources/testdata/configer-dev.toml";

    let toml_reader = TomlConfigReader::default();
    let toml_rvt = toml_reader.read_from_path(path);

    if let Ok(table) = toml_rvt {
        let configer = ConfigerEnvironment::table(table);
        let rvt_database_servers = configer.get("database.servers");

        return assert_configer_array(rvt_database_servers, "database.servers");
    }

    panic!("Failed to read configer-dev.toml file")
}

// ----------------------------------------------------------------

#[test]
#[allow(deprecated)]
fn test_build_configer_by_register_table() {
    let path = "resources/testdata/configer-dev.toml";

    let mut configer = ConfigerEnvironment::new();
    if let Some(reader) = configer.try_acquire("toml") {
        let toml_from_path_rvt = reader.read_from_path(path);
        if let Ok(table) = toml_from_path_rvt {
            configer.register_table(table);

            let rvt_string_value = configer.get("table.table_s");
            match NodeConverter::try_string(rvt_string_value) {
                Some(v) => {
                    assert_eq!(*v, "value1");
                }
                _ => panic!("Get key:[table.table_s] failed")
            }

            return ();
        }
    }

    panic!("Failed to read configer-dev.toml file")
}

// ----------------------------------------------------------------

#[test]
fn test_build_configer_builder_with_table() {
    let path = "resources/testdata/configer-dev.toml";

    let toml_reader = TomlConfigReader::default();
    let toml_rvt = toml_reader.read_from_path(path);

    if let Ok(table) = toml_rvt {
        let builder_rvt = ConfigerEnvironment::builder()
            .with_table(table)
            .build();

        if let Ok(configer) = builder_rvt {
            let rvt_database_servers = configer.get("database.servers");

            return assert_configer_array(rvt_database_servers, "database.servers");
        }

        panic!("Failed to build ConfigerEnvironment")
    }

    panic!("Failed to read configer-dev.toml file")
}

#[test]
fn test_build_configer_builder_with_registry_and_path() {
    let path = "resources/testdata/configer-dev.toml";

    let toml_reader = TomlConfigReader::default();
    let mut registry = ConfigReaderRegistry::default();
    registry.register(Box::new(toml_reader));

    let builder_rvt = ConfigerEnvironment::builder()
        .with_registry(Box::new(registry))
        .with_path(path.to_string())
        .build();

    if let Ok(configer) = builder_rvt {
        let rvt_database_servers = configer.get("database.servers");
        return assert_configer_array(rvt_database_servers, "database.servers");
    }

    panic!("Failed to read configer-dev.toml file")
}

// ----------------------------------------------------------------

/// @since 0.5.0
#[test]
fn test_load_environment_variables() {
    env::set_var("CONFIGER_TEST_VAR", "rust.configer");
    let configer_var = env::var("CONFIGER_TEST_VAR").unwrap();
    assert_eq!(configer_var, "rust.configer");

    let table = crate::env::try_load_env_variables();
    let var_rvt = table.get("CONFIGER_TEST_VAR");

    assert_eq!(var_rvt, Some(&Node::String(String::from("rust.configer"))));
}

// ----------------------------------------------------------------

/// @since 0.5.0
#[test]
fn test_build_configer_builder_with_table_registry_and_path() {
    env::set_var("CONFIGER_TEST_VAR", "rust.configer");

    let path = "resources/testdata/configer-dev.toml";

    let toml_reader = TomlConfigReader::default();
    let mut registry = ConfigReaderRegistry::default();
    registry.register(Box::new(toml_reader));

    let table = crate::env::try_load_env_variables();

    let builder_rvt = ConfigerEnvironment::builder()
        .with_table(table)
        .with_registry(Box::new(registry))
        .with_path(path.to_string())
        .build();

    if let Ok(configer) = builder_rvt {
        let rvt_database_servers = configer.get("database.servers");
        assert_configer_array(rvt_database_servers, "database.servers");

        let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
        assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));

        return ();
    }

    panic!("Failed to read configer-dev.toml file")
}

/// @since 0.5.0
#[test]
fn test_build_configer_builder_without_table_with_registry_and_path() {
    env::set_var("CONFIGER_TEST_VAR", "rust.configer");

    let path = "resources/testdata/configer-dev.toml";

    let toml_reader = TomlConfigReader::default();
    let mut registry = ConfigReaderRegistry::default();
    registry.register(Box::new(toml_reader));

    let builder_rvt = ConfigerEnvironment::builder()
        .with_registry(Box::new(registry))
        .with_path(path.to_string())
        .build(); // load environment variables auto.

    if let Ok(configer) = builder_rvt {
        let rvt_database_servers = configer.get("database.servers");
        assert_configer_array(rvt_database_servers, "database.servers");

        let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
        assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));

        return ();
    }

    panic!("Failed to read configer-dev.toml file")
}

/// @since 0.5.0
#[test]
fn test_default_configer_with_enn_variables() {
    env::set_var("CONFIGER_TEST_VAR", "rust.configer");

    let configer = ConfigerEnvironment::default();
    let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
    assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));
}

/// @since 0.5.0
#[test]
fn test_new_configer_with_enn_variables() {
    env::set_var("CONFIGER_TEST_VAR", "rust.configer");

    let configer = ConfigerEnvironment::new();
    let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
    assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));
}

/// @since 0.5.0
#[test]
fn test_mixed_configer_with_enn_variables() {
    env::set_var("CONFIGER_TEST_VAR", "rust.configer");

    let configer = ConfigerEnvironment::mixed_with_env_variables(None, None);
    let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
    assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));
}

/// @since 0.5.0
#[test]
fn test_table_configer_with_env_variables() {
    env::set_var("CONFIGER_TEST_VAR", "rust.configer");

    let path = "resources/testdata/configer-dev.toml";

    let toml_reader = TomlConfigReader::default();
    let toml_rvt = toml_reader.read_from_path(path);

    if let Ok(table) = toml_rvt {
        let configer = ConfigerEnvironment::table(table);

        let rvt_database_servers = configer.get("database.servers");
        assert_configer_array(rvt_database_servers, "database.servers");

        let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
        assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));

        return ();
    }

    panic!("Failed to read configer-dev.toml file")
}

// ----------------------------------------------------------------

fn traverse_toml(value: &Value) {
    match value {
        Value::String(_) => { /*println!("String: {}", s)*/ }
        Value::Integer(_) => { /*println!("Integer: {}", i)*/ }
        Value::Float(_) => { /*println!("Float: {}", f)*/ }
        Value::Boolean(_) => { /*println!("Boolean: {}", b)*/ }
        Value::Array(arr) => {
            for v in arr {
                traverse_toml(v);
            }
        }
        Value::Table(table) => {
            for (_, value) in table.iter() {
                traverse_toml(value);
            }
        }
        _ => println!("Unknown type"),
    }
}

// ----------------------------------------------------------------

fn assert_configer_array(rvt_database_servers: Result<&Node, ConfigerError>, key: &str) {
    match NodeConverter::try_array(rvt_database_servers) {
        Some(servers) => {
            let mut array = domain::Array::new();
            array.push(Node::String("192.168.1.1".to_string()));
            array.push(Node::String("192.168.1.2".to_string()));
            array.push(Node::String("192.168.1.3".to_string()));

            assert!(assert_node_array_equals(servers, &array));
        }
        _ => panic!("Failed to get key:[{}]", key)
    }
}

fn assert_node_array_equals(array: &domain::Array, vec: &domain::Array) -> bool {
    array.iter().zip(vec.iter()).all(|(a, b)| a == b)
}