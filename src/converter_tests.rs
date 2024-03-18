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

use std::f32::consts::PI;

use chrono::NaiveDateTime;
use chronounit::formatter::pattern::DateTimePattern;
use snowflaker::snowflake_dynamic;

use crate::domain;
use crate::domain::{Node, Table};
use crate::domain::converter::NodeConverter;
use crate::env::Environment;
use crate::env::standard::ConfigerEnvironment;

#[test]
#[rustfmt::skip]
fn test_get_converter_nested() {
    let mut configer = ConfigerEnvironment::new();

    let mut nested = Table::new();
    nested.insert("Hello".to_string(), Node::String("Rust".to_string()));

    configer.set("io.github.photowey.nested", Node::Nested(nested)).unwrap();

    let rvt_nested = configer.get("io.github.photowey.nested");

    if let Some(into_value) = NodeConverter::try_nested(rvt_nested) {
        match into_value.get("Hello") {
            Some(node) => {
                assert_eq!(*node, Node::String("Rust".to_string()));
            }
            _ => {}
        }
    } else {
        panic!("failed to convert the value to Table")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_array() {
    let mut configer = ConfigerEnvironment::new();
    let now = 1710265983u32;
    let mut array = domain::Array::new();
    array.push(Node::String("Rust".to_string()));
    array.push(Node::IntU32(now));

    configer.set("io.github.photowey.array", Node::Array(array)).unwrap();

    let rvt_array = configer.get("io.github.photowey.array");

    let mut image = domain::Array::new();
    image.push(Node::String("Rust".to_string()));
    image.push(Node::IntU32(now));

    if let Some(into_value) = NodeConverter::try_array(rvt_array) {
        assert!(assert_array_equals(into_value, &image));
    } else {
        panic!("failed to convert the value to Table")
    }
}

fn assert_array_equals(array: &domain::Array, vec: &domain::Array) -> bool {
    array.iter().zip(vec.iter()).all(|(a, b)| a == b)
}

#[test]
#[rustfmt::skip]
fn test_get_converter_date_time() {
    let mut configer = ConfigerEnvironment::new();

    let now = NaiveDateTime::parse_from_str("2024-03-11 22:50:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS).unwrap();
    configer.set("io.github.photowey.configer.Time", now.into()).unwrap();

    let rvt_time = configer.get("io.github.photowey.configer.Time");

    // match
    match rvt_time {
        Ok(node) => {
            match node {
                Node::DateTime(ref time) => {
                    assert_eq!(*time, now);
                }
                _ => {}
            }
        }
        _ => {}
    }

    // converter
    if let Some(into_value) = NodeConverter::try_datetime(rvt_time) {
        assert_eq!(*into_value, now);
    } else {
        panic!("failed to convert the value to NaiveDateTime")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_string() {
    let mut configer = ConfigerEnvironment::new();

    configer.set("io.github.photowey.str", String::from("Rust").into()).unwrap();
    let rvt_string = configer.get("io.github.photowey.str");

    if let Some(into_value) = NodeConverter::try_string(rvt_string) {
        assert_eq!(*into_value, String::from("Rust"));
    } else {
        panic!("failed to convert the value to String")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_str() {
    let mut configer = ConfigerEnvironment::new();

    configer.set("io.github.photowey.str", "Rust".into()).unwrap();
    let rvt_str = configer.get("io.github.photowey.str");

    if let Some(into_value) = NodeConverter::try_str(rvt_str) {
        assert_eq!(into_value, "Rust");
    } else {
        panic!("failed to convert the value to &str")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_bool() {
    let mut configer = ConfigerEnvironment::new();

    configer.set("io.github.photowey.bool", false.into()).unwrap();
    let rvt_bool = configer.get("io.github.photowey.bool");

    if let Some(into_value) = NodeConverter::try_bool(rvt_bool) {
        assert_eq!(*into_value, false);
    } else {
        panic!("failed to convert the value to false")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_u128() {
    let mut configer = ConfigerEnvironment::new();

    let rvt = snowflake_dynamic!().unwrap() as u128;
    configer.set("io.github.photowey.u128", rvt.into()).unwrap();
    let rvt_u128 = configer.get("io.github.photowey.u128");

    if let Some(into_value) = NodeConverter::try_int_u128(rvt_u128) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to u128")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_u64() {
    let mut configer = ConfigerEnvironment::new();

    let rvt = snowflake_dynamic!().unwrap();
    configer.set("io.github.photowey.u64", rvt.into()).unwrap();
    let rvt_u64 = configer.get("io.github.photowey.u64");

    if let Some(into_value) = NodeConverter::try_int_u64(rvt_u64) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to u64")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_u32() {
    let mut configer = ConfigerEnvironment::new();

    let rvt = 1710265983u32;
    configer.set("io.github.photowey.u32", rvt.into()).unwrap();
    let rvt_u32 = configer.get("io.github.photowey.u32");

    if let Some(into_value) = NodeConverter::try_int_u32(rvt_u32) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to u32")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_i128() {
    let mut configer = ConfigerEnvironment::new();

    let rvt = snowflake_dynamic!().unwrap() as i128;
    configer.set("io.github.photowey.i128", rvt.into()).unwrap();
    let rvt_i128 = configer.get("io.github.photowey.i128");

    if let Some(into_value) = NodeConverter::try_int_i128(rvt_i128) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to i128")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_i64() {
    let mut configer = ConfigerEnvironment::new();

    let rvt = snowflake_dynamic!().unwrap() as i64;
    configer.set("io.github.photowey.i64", rvt.into()).unwrap();
    let rvt_i64 = configer.get("io.github.photowey.i64");

    if let Some(into_value) = NodeConverter::try_int_i64(rvt_i64) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to i64")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_i32() {
    let mut configer = ConfigerEnvironment::new();

    let rvt = 1710265983i32;
    configer.set("io.github.photowey.i32", rvt.into()).unwrap();
    let rvt_i32 = configer.get("io.github.photowey.i32");

    if let Some(into_value) = NodeConverter::try_int_i32(rvt_i32) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to i32")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_f64() {
    let mut configer = ConfigerEnvironment::new();

    let rvt = PI as f64;
    configer.set("io.github.photowey.f64", rvt.into()).unwrap();
    let rvt_f64 = configer.get("io.github.photowey.f64");

    if let Some(into_value) = NodeConverter::try_float64(rvt_f64) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to f64")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_f32() {
    let mut configer = ConfigerEnvironment::new();

    let rvt = PI;
    configer.set("io.github.photowey.f32", rvt.into()).unwrap();
    let rvt_f32 = configer.get("io.github.photowey.f32");

    if let Some(into_value) = NodeConverter::try_float32(rvt_f32) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to f32")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_none() {
    let mut configer = ConfigerEnvironment::new();

    let none = Node::None;
    configer.set("io.github.photowey.none", none).unwrap();
    let rvt_none = configer.get("io.github.photowey.none");

    if let Some(into_value) = NodeConverter::try_none(rvt_none) {
        assert_eq!(*into_value, ());
    } else {
        panic!("failed to convert the value to none")
    }
}
