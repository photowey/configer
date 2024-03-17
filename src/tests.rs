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

use std::f32::consts::PI;

use chrono::NaiveDateTime;
use chronounit::formatter::pattern::DateTimePattern;
use snowflaker::snowflake_dynamic;

use crate::domain::{merge_tables, Node, Table};
use crate::env::{DynamicEnvironment, Environment};
use crate::env::standard::ConfigerEnvironment;
use crate::error::ConfigerError;

// ----------------------------------------------------------------

#[test]
#[rustfmt::skip]
fn test_set() {
    let mut configer = ConfigerEnvironment::new();

    configer.set("io.github.photowey.string", String::from("Hello, Configer!").into()).unwrap();
    configer.set("io.github.photowey.str", "Rust".into()).unwrap();

    let rvt = snowflake_dynamic!().unwrap() as i64;
    configer.set("io.github.photowey.i32", 123_i32.into()).unwrap();
    configer.set("io.github.photowey.i64", rvt.into()).unwrap();

    let pi = PI as f64;
    configer.set("io.github.photowey.configer.f32", 9527.8848_f32.into()).unwrap();
    configer.set("io.github.photowey.configer.f64", pi.into()).unwrap();

    let now =
        NaiveDateTime::parse_from_str("2024-03-11 22:50:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS)
            .unwrap();
    configer
        .set("io.github.photowey.configer.Time", now.into())
        .unwrap();
}

/// @since 0.4.2
#[test]
#[rustfmt::skip]
fn test_set_t() {
    let mut configer = ConfigerEnvironment::new();
    configer.set_t("io.github.photowey.string", String::from("Hello, Configer!")).unwrap();
    configer.set_t("io.github.photowey.str", "Rust").unwrap();

    let rvt = snowflake_dynamic!().unwrap() as i64;
    configer.set_t("io.github.photowey.i32", 123_i32).unwrap();
    configer.set_t("io.github.photowey.i64", rvt).unwrap();

    let pi = PI as f64;
    configer.set_t("io.github.photowey.configer.f32", 9527.8848_f32).unwrap();
    configer.set_t("io.github.photowey.configer.f64", pi).unwrap();

    let now = NaiveDateTime::parse_from_str("2024-03-11 22:50:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS).unwrap();
    configer.set_t("io.github.photowey.configer.Time", now).unwrap();
}

#[test]
#[rustfmt::skip]
fn test_set_empty_key() {
    let mut configer = ConfigerEnvironment::new();

    let now = NaiveDateTime::parse_from_str("2024-03-11 22:50:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS).unwrap();
    let empty_rvt = configer.set("", now.into());
    assert_eq!(empty_rvt, Err(ConfigerError::EmptyKey));

    let empty_rvt = configer.set("", "Rust".into());
    assert_eq!(empty_rvt, Err(ConfigerError::EmptyKey));
}

#[test]
#[rustfmt::skip]
fn test_get() {
    let mut configer = ConfigerEnvironment::new();

    configer.set("io.github.photowey.string", String::from("Hello, Configer!").into()).unwrap();
    configer.set("io.github.photowey.str", "Rust".into()).unwrap();

    let rvt = snowflake_dynamic!().unwrap() as i64;
    configer.set("io.github.photowey.i32", 123_i32.into()).unwrap();
    configer.set("io.github.photowey.i64", rvt.into()).unwrap();

    let pi = PI as f64;
    configer.set("io.github.photowey.configer.f32", 9527.8848_f32.into()).unwrap();
    configer.set("io.github.photowey.configer.f64", pi.into()).unwrap();

    let now = NaiveDateTime::parse_from_str("2024-03-11 22:50:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS).unwrap();
    configer.set("io.github.photowey.configer.Time", now.into()).unwrap();

    assert_eq!(
        configer.get("io.github.photowey.string"),
        Ok(&Node::String(String::from("Hello, Configer!").into()))
    );
    assert_eq!(
        configer.get("io.github.photowey.str"),
        Ok(&Node::String(String::from("Rust").into()))
    );
    assert_eq!(
        configer.get("io.github.photowey.i32"),
        Ok(&Node::Int32(123_i32))
    );
    assert_eq!(
        configer.get("io.github.photowey.i64"),
        Ok(&Node::Int64(rvt))
    );
    assert_eq!(
        configer.get("io.github.photowey.configer.f32"),
        Ok(&Node::Float32(9527.8848_f32))
    );
    assert_eq!(
        configer.get("io.github.photowey.configer.f64"),
        Ok(&Node::Float64(pi))
    );
    assert_eq!(
        configer.get("io.github.photowey.configer.Time"),
        Ok(&Node::DateTime(now))
    );
}

/// @since 0.4.2
#[test]
#[rustfmt::skip]
fn test_get_by_set_t() {
    let mut configer = ConfigerEnvironment::new();

    configer.set_t("io.github.photowey.string", String::from("Hello, Configer!")).unwrap();
    configer.set_t("io.github.photowey.str", "Rust").unwrap();

    let rvt = snowflake_dynamic!().unwrap() as i64;
    configer.set_t("io.github.photowey.i32", 123_i32).unwrap();
    configer.set_t("io.github.photowey.i64", rvt).unwrap();

    let pi = PI as f64;
    configer.set_t("io.github.photowey.configer.f32", 9527.8848_f32).unwrap();
    configer.set_t("io.github.photowey.configer.f64", pi).unwrap();

    let now = NaiveDateTime::parse_from_str("2024-03-11 22:50:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS).unwrap();
    configer.set_t("io.github.photowey.configer.Time", now).unwrap();

    assert_eq!(
        configer.get("io.github.photowey.string"),
        Ok(&Node::String(String::from("Hello, Configer!").into()))
    );
    assert_eq!(
        configer.get("io.github.photowey.str"),
        Ok(&Node::String(String::from("Rust").into()))
    );
    assert_eq!(
        configer.get("io.github.photowey.i32"),
        Ok(&Node::Int32(123_i32))
    );
    assert_eq!(
        configer.get("io.github.photowey.i64"),
        Ok(&Node::Int64(rvt))
    );
    assert_eq!(
        configer.get("io.github.photowey.configer.f32"),
        Ok(&Node::Float32(9527.8848_f32))
    );
    assert_eq!(
        configer.get("io.github.photowey.configer.f64"),
        Ok(&Node::Float64(pi))
    );
    assert_eq!(
        configer.get("io.github.photowey.configer.Time"),
        Ok(&Node::DateTime(now))
    );
}

#[test]
#[rustfmt::skip]
fn test_get_not_found() {
    let mut configer = ConfigerEnvironment::new();
    configer.set("io.github.photowey.configer.ok", "Rust".into()).unwrap();

    assert_eq!(
        configer.get("io.github.photowey.configer.not.found"),
        Err(ConfigerError::NotFound)
    );
}

// ----------------------------------------------------------------

/// @since 0.5.0
#[test]
fn test_merge_tables() {
    let mut table_a: Table = Table::new();
    table_a.insert("io".to_string(), Node::Nested({
        let mut inner_table = Table::new();
        inner_table.insert("github".to_string(), Node::Array(vec![Node::Int32(1), Node::Int32(3)]));
        inner_table
    }));
    table_a.insert("replaced".to_string(), Node::Float64(PI as f64));

    let mut table_b: Table = Table::new();

    // 1
    table_b.insert("io".to_string(), Node::Nested({
        let mut inner_table = Table::new();
        inner_table.insert("github".to_string(), Node::Array(vec![Node::Int32(2), Node::Int32(4)]));
        inner_table
    }));
    // 2
    table_b.insert("hello".to_string(), Node::Nested({
        let mut inner_table = Table::new();
        inner_table.insert("world".to_string(), Node::Array(vec![Node::Int32(2), Node::Int32(4)]));
        inner_table
    }));

    // 3
    let seed = snowflake_dynamic!().unwrap();
    table_b.insert("replaced".to_string(), Node::IntU64(seed));

    // 4: merge
    let merged_table = merge_tables(table_a, table_b);

    let mut table_sentinel: Table = Table::new();
    table_sentinel.insert("io".to_string(), Node::Nested({
        let mut inner_table = Table::new();
        inner_table.insert("github".to_string(), Node::Array(vec![Node::Int32(1), Node::Int32(3), Node::Int32(2), Node::Int32(4)]));
        inner_table
    }));
    table_sentinel.insert("hello".to_string(), Node::Nested({
        let mut inner_table = Table::new();
        inner_table.insert("world".to_string(), Node::Array(vec![Node::Int32(2), Node::Int32(4)]));
        inner_table
    }));
    // replaced
    table_sentinel.insert("replaced".to_string(), Node::IntU64(seed));

    assert_eq!(merged_table, table_sentinel);
}