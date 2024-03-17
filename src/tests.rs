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

use crate::domain::Node;
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
fn test_get_not_found() {
    let mut configer = ConfigerEnvironment::new();
    configer
        .set("io.github.photowey.configer.ok", "Rust".into())
        .unwrap();

    assert_eq!(
        configer.get("io.github.photowey.configer.not.found"),
        Err(ConfigerError::NotFound)
    );
}
