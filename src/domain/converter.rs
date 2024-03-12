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

use crate::domain::{Array, Node, Table};
use crate::error::ConfigerError;

// ----------------------------------------------------------------

/// A struct responsible for converting between node types.
///
/// @sine 0.2.0
pub struct NodeConverter;

impl NodeConverter {
    pub fn try_datetime(rvt: Result<&Node, ConfigerError>) -> Option<&NaiveDateTime> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_string(rvt: Result<&Node, ConfigerError>) -> Option<&String> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_str(rvt: Result<&Node, ConfigerError>) -> Option<&str> {
        match rvt {
            Ok(Node::String(s)) => Some(s),
            _ => None,
        }
    }

    pub fn try_nested(rvt: Result<&Node, ConfigerError>) -> Option<&Table> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_array(rvt: Result<&Node, ConfigerError>) -> Option<&Array> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_int_u128(rvt: Result<&Node, ConfigerError>) -> Option<&u128> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_int_u64(rvt: Result<&Node, ConfigerError>) -> Option<&u64> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_int_u32(rvt: Result<&Node, ConfigerError>) -> Option<&u32> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_int_i128(rvt: Result<&Node, ConfigerError>) -> Option<&i128> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_int_i64(rvt: Result<&Node, ConfigerError>) -> Option<&i64> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_int_i32(rvt: Result<&Node, ConfigerError>) -> Option<&i32> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_float64(rvt: Result<&Node, ConfigerError>) -> Option<&f64> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_float32(rvt: Result<&Node, ConfigerError>) -> Option<&f32> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }

    pub fn try_none(rvt: Result<&Node, ConfigerError>) -> Option<&()> {
        match rvt {
            Ok(node) => node.into(),
            _ => None,
        }
    }
}
