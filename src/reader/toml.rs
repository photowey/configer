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

use chrono::NaiveDateTime;
use toml::de::Error;
use toml::Value;
use toml::value::{Date, Datetime, Time};

use crate::domain::{Node, Table};
use crate::error::FileError;
use crate::reader::ConfigReader;

// ----------------------------------------------------------------

pub const TOML: &str = "TOML";
const TOML_READER_NAME: &str = "toml";

// ----------------------------------------------------------------

pub struct TomlConfigReader {
    name: String,
    suffix: String,
}

impl TomlConfigReader {
    fn new() -> Self {
        Self {
            name: TOML_READER_NAME.to_string(),
            suffix: TOML_READER_NAME.to_string(),
        }
    }
}

// ----------------------------------------------------------------

impl Default for TomlConfigReader {
    fn default() -> Self {
        Self::new()
    }
}

// ----------------------------------------------------------------

impl ConfigReader for TomlConfigReader {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn supports(&self, suffix: &str) -> bool {
        self.suffix.eq(suffix)
    }

    fn read_from_str(&self, data: &str) -> Result<Table, FileError> {
        let mut ctx: Table = Table::new();
        let parsed_rvt: Result<Value, Error> = toml::from_str(&data);
        match parsed_rvt {
            Ok(value) => {
                if let Value::Table(table) = value {
                    for (key, value) in table {
                        ctx.insert(key, toml_value_to_node(value));
                    }

                    return Ok(ctx);
                }

                Err(FileError::IncorrectFormat(TOML.to_string()))
            }
            Err(err) => Err(FileError::ParseFailed(TOML.to_string(), err.message().to_string()))
        }
    }
}

// ----------------------------------------------------------------

pub fn toml_value_to_node(value: Value) -> Node {
    match value {
        Value::String(s) => Node::String(s),
        Value::Integer(i) => Node::Int64(i),
        Value::Float(f) => Node::Float64(f),
        Value::Boolean(b) => Node::Boolean(b),
        Value::Datetime(datetime) => {
            Node::DateTime(datetime_to_chrono_naive_date_time(datetime).unwrap())
        }
        Value::Array(arr) => Node::Array(arr.into_iter().map(toml_value_to_node).collect()),
        Value::Table(table) => {
            Node::Nested(table.into_iter().map(|(k, v)| (k, toml_value_to_node(v))).collect())
        }
    }
}

pub fn datetime_to_chrono_naive_date_time(datetime: Datetime) -> Option<NaiveDateTime> {
    match (datetime.date, datetime.time, datetime.offset) {
        (Some(date), Some(time), _) => Some(datetime_to_naive_time(date, time)),
        (Some(date), None, None) => Some(date_to_naive(date)),
        (None, Some(time), None) => Some(time_to_naive(time)),
        _ => None,
    }
}

pub fn date_to_naive(date: Date) -> NaiveDateTime {
    NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(date.year as i32, date.month as u32, date.day as u32).unwrap(),
        chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    )
}

pub fn datetime_to_naive_time(date: Date, time: Time) -> NaiveDateTime {
    NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(date.year as i32, date.month as u32, date.day as u32).unwrap(),
        chrono::NaiveTime::from_hms_opt(time.hour as u32, time.minute as u32, time.second as u32).unwrap(),
    )
}

pub fn time_to_naive(time: Time) -> NaiveDateTime {
    NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(0, 0, 0).unwrap(),
        chrono::NaiveTime::from_hms_opt(time.hour as u32, time.minute as u32, time.second as u32).unwrap(),
    )
}