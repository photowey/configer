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
use std::fs;

use toml::Value;

use crate::{domain, reader};
use crate::domain::{Node, Table};
use crate::domain::converter::NodeConverter;
use crate::env::Environment;
use crate::env::standard::ConfigerEnvironment;
use crate::reader::ConfigReader;
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
        let configer = ConfigerEnvironment::build(table);
        let rvt_database_servers = configer.get("database.servers");

        match NodeConverter::try_array(rvt_database_servers) {
            Some(servers) => {
                let mut array = domain::Array::new();
                array.push(Node::String("192.168.1.1".to_string()));
                array.push(Node::String("192.168.1.2".to_string()));
                array.push(Node::String("192.168.1.3".to_string()));

                assert!(assert_node_array_equals(servers, &array));
            }
            _ => panic!("Get key:[database.servers] failed")
        }

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

fn assert_node_array_equals(array: &domain::Array, vec: &domain::Array) -> bool {
    array.iter().zip(vec.iter()).all(|(a, b)| a == b)
}