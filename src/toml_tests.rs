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

use crate::domain::Node;
use crate::reader;
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
fn test_toml_reader() {
    let path = "resources/testdata/configer-dev.toml";

    let toml_reader = TomlConfigReader::default();

    let toml_rvt = toml_reader.read_from_path(path);

    if let Ok(table) = toml_rvt {
        assert!(table.contains_key("string_value"));
        assert!(table.contains_key("floats"));
        assert!(table.contains_key("table"));
        assert!(table.contains_key("database"));

        return ();
    }

    panic!("TOML reader read config-dev.toml failed.")
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