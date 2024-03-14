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

use std::collections::HashMap;
use std::fs;
use std::path::MAIN_SEPARATOR;

use chrono::NaiveDateTime;
use toml::Value;
use toml::value::{Date, Datetime, Time};

use crate::domain::Node;

#[test]
fn test_toml_read() {
    let path = format!("resources{}testdata{}configer-dev.toml", MAIN_SEPARATOR, MAIN_SEPARATOR);

    let toml_str = fs::read_to_string(path)
        .expect("Failed to read configer-dev.toml file");

    let parsed_toml: Value = toml::from_str(&toml_str)
        .expect("Failed to parse configer-dev.toml file");

    traverse_toml(&parsed_toml)
}

#[test]
fn test_toml_value_to_node() {
    let path = format!("resources{}testdata{}configer-dev.toml", MAIN_SEPARATOR, MAIN_SEPARATOR);

    let toml_str = fs::read_to_string(path)
        .expect("Failed to read configer-dev.toml file");

    let parsed_toml: Value = toml::from_str(&toml_str)
        .expect("Failed to parse configer-dev.toml file");

    let mut hashmap: HashMap<String, Node> = HashMap::new();
    if let Value::Table(table) = parsed_toml {
        for (key, value) in table {
            hashmap.insert(key, value_to_node(value));
        }
    }

    for (key, value) in &hashmap {
        println!("Key: {:?}, Value: {:?}", key, value);
    }
}

fn value_to_node(value: Value) -> Node {
    match value {
        Value::String(s) => Node::String(s),
        Value::Integer(i) => Node::Int64(i),
        Value::Float(f) => Node::Float64(f),
        Value::Boolean(b) => Node::Boolean(b),
        Value::Datetime(datetime) => {
            Node::DateTime(datetime_to_chrono_naive_date_time(datetime).unwrap())
        }
        Value::Array(arr) => Node::Array(arr.into_iter().map(value_to_node).collect()),
        Value::Table(table) => {
            Node::Nested(table.into_iter().map(|(k, v)| (k, value_to_node(v))).collect())
        }
    }
}

fn traverse_toml(value: &Value) {
    match value {
        Value::String(s) => println!("String: {}", s),
        Value::Integer(i) => println!("Integer: {}", i),
        Value::Float(f) => println!("Float: {}", f),
        Value::Boolean(b) => println!("Boolean: {}", b),
        Value::Array(arr) => {
            println!("Array:--------------------------------");
            for v in arr {
                traverse_toml(v);
            }
        }
        Value::Table(table) => {
            for (key, value) in table.iter() {
                println!("Table-key: {}", key);
                traverse_toml(value);
            }
        }
        _ => println!("Unknown type"),
    }
}

fn datetime_to_chrono_naive_date_time(datetime: Datetime) -> Option<NaiveDateTime> {
    match (datetime.date, datetime.time, datetime.offset) {
        (Some(date), Some(time), _) => Some(datetime_to_naive_time(date, time)),
        (Some(date), None, None) => Some(date_to_naive(date)),
        (None, Some(time), None) => Some(time_to_naive(time)),
        _ => None,
    }
}

fn date_to_naive(date: Date) -> NaiveDateTime {
    NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(date.year as i32, date.month as u32, date.day as u32).unwrap(),
        chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    )
}

fn datetime_to_naive_time(date: Date, time: Time) -> NaiveDateTime {
    NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(date.year as i32, date.month as u32, date.day as u32).unwrap(),
        chrono::NaiveTime::from_hms_opt(time.hour as u32, time.minute as u32, time.second as u32).unwrap(),
    )
}

fn time_to_naive(time: Time) -> NaiveDateTime {
    NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(0, 0, 0).unwrap(),
        chrono::NaiveTime::from_hms_opt(time.hour as u32, time.minute as u32, time.second as u32).unwrap(),
    )
}