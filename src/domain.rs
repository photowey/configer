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

use chrono::NaiveDateTime;

// ----------------------------------------------------------------

pub type Table = HashMap<String, Node>;

#[derive(Debug, PartialEq)]
pub enum Node {
    Nested(Table),
    DateTime(NaiveDateTime),
    String(String),
    IntU64(u64),
    IntU32(u32),
    Int64(i64),
    Int32(i32),
    Float64(f64),
    Float32(f32),
}

impl Node {
    pub fn as_nested_mut(&mut self) -> Option<&mut Table> {
        match self {
            Node::Nested(ref mut nested) => Some(nested),
            _ => None,
        }
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::String(value)
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::String(value.to_string())
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node::Int32(value)
    }
}

impl From<i64> for Node {
    fn from(value: i64) -> Self {
        Node::Int64(value)
    }
}

impl From<u32> for Node {
    fn from(value: u32) -> Self {
        Node::IntU32(value)
    }
}

impl From<u64> for Node {
    fn from(value: u64) -> Self {
        Node::IntU64(value)
    }
}

impl From<f64> for Node {
    fn from(value: f64) -> Self {
        Node::Float64(value)
    }
}

impl From<f32> for Node {
    fn from(value: f32) -> Self {
        Node::Float32(value)
    }
}

impl From<NaiveDateTime> for Node {
    fn from(value: NaiveDateTime) -> Self {
        Node::DateTime(value)
    }
}
