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
pub mod converter;

// ----------------------------------------------------------------

pub type Table = HashMap<String, Node>;

/// @since 0.2.0

pub type Array = Vec<Node>;

// ----------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum Node {
    Nested(Table),
    /// @since 0.2.0
    Array(Array),
    DateTime(NaiveDateTime),
    String(String),
    /// @since 0.3.0
    Boolean(bool),
    /// @since 0.2.0
    IntU128(u128),
    IntU64(u64),
    IntU32(u32),
    /// @since 0.2.0
    Int128(i128),
    Int64(i64),
    Int32(i32),
    Float64(f64),
    Float32(f32),
    /// @since 0.2.0
    None,
}

impl Node {
    pub fn as_nested_mut(&mut self) -> Option<&mut Table> {
        match self {
            Node::Nested(ref mut nested) => Some(nested),
            _ => None,
        }
    }
}

// ----------------------------------------------------------------

impl Default for Node {
    fn default() -> Self {
        Self::None
    }
}

// ----------------------------------------------------------------

impl From<Table> for Node {
    fn from(value: Table) -> Self {
        Node::Nested(value)
    }
}

// ----------------------------------------------------------------

impl From<Array> for Node {
    fn from(value: Array) -> Self {
        Node::Array(value)
    }
}

// ----------------------------------------------------------------

impl From<NaiveDateTime> for Node {
    fn from(value: NaiveDateTime) -> Self {
        Node::DateTime(value)
    }
}

// ----------------------------------------------------------------

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

// ----------------------------------------------------------------

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        Node::Boolean(value)
    }
}

// ----------------------------------------------------------------

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

impl From<i128> for Node {
    fn from(value: i128) -> Self {
        Node::Int128(value)
    }
}

// ----------------------------------------------------------------

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

impl From<u128> for Node {
    fn from(value: u128) -> Self {
        Node::IntU128(value)
    }
}

// ----------------------------------------------------------------

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

// ----------------------------------------------------------------

impl<'a> From<&'a Node> for Option<&'a Table> {
    fn from(node: &'a Node) -> Option<&'a Table> {
        match *node {
            Node::Nested(ref table) => Some(table),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a Array> {
    fn from(node: &'a Node) -> Option<&'a Array> {
        match *node {
            Node::Array(ref array) => Some(array),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a NaiveDateTime> {
    fn from(node: &'a Node) -> Option<&'a NaiveDateTime> {
        match *node {
            Node::DateTime(ref time) => Some(time),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a String> {
    fn from(node: &'a Node) -> Option<&'a String> {
        match *node {
            Node::String(ref val) => Some(val),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a bool> {
    fn from(node: &'a Node) -> Option<&'a bool> {
        match *node {
            Node::Boolean(ref val) => Some(val),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a i128> {
    fn from(node: &'a Node) -> Option<&'a i128> {
        match *node {
            Node::Int128(ref val) => Some(val),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a u128> {
    fn from(node: &'a Node) -> Option<&'a u128> {
        match *node {
            Node::IntU128(ref val) => Some(val),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a i64> {
    fn from(node: &'a Node) -> Option<&'a i64> {
        match *node {
            Node::Int64(ref val) => Some(val),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a u64> {
    fn from(node: &'a Node) -> Option<&'a u64> {
        match *node {
            Node::IntU64(ref val) => Some(val),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a i32> {
    fn from(node: &'a Node) -> Option<&'a i32> {
        match *node {
            Node::Int32(ref val) => Some(val),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a u32> {
    fn from(node: &'a Node) -> Option<&'a u32> {
        match *node {
            Node::IntU32(ref val) => Some(val),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a f64> {
    fn from(node: &'a Node) -> Option<&'a f64> {
        match *node {
            Node::Float64(ref val) => Some(val),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a f32> {
    fn from(node: &'a Node) -> Option<&'a f32> {
        match *node {
            Node::Float32(ref val) => Some(val),
            _ => None,
        }
    }
}

impl<'a> From<&'a Node> for Option<&'a ()> {
    fn from(node: &'a Node) -> Option<&'a ()> {
        match *node {
            Node::None => Some(&()),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------- Into end
