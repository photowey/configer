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

use std::error::Error;
use std::fmt;

// ----------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum ConfigerError {
    EmptyKey,
    NonNested,
    NotFound,
}

impl fmt::Display for ConfigerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigerError::EmptyKey => write!(f, "Key can't be empty"),
            ConfigerError::NonNested => {
                write!(
                    f,
                    "Attempted to set/get a nested value on a non-nested node"
                )
            }
            ConfigerError::NotFound => write!(f, "Not found"),
        }
    }
}

impl Error for ConfigerError {}

// ----------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum FileError {
    InvalidPath(String),
    InvalidFile(String),
    ReaderNotFound(String),
    ReadFailed(String),
    IncorrectFormat(String),
    ParseFailed(String, String),
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::InvalidPath(path) => write!(f, "Invalid path:[{}]", path),
            FileError::InvalidFile(file) => write!(f, "Invalid file type:[{}]", file),
            FileError::ReaderNotFound(suffix) => write!(f, "Reader not found, suffix: {}", suffix),
            FileError::ReadFailed(path) => write!(f, "Failed to read config file, path:[{}]", path),
            FileError::IncorrectFormat(reader_type) => write!(f, "Incorrect {} format: Missing table data.", reader_type),
            FileError::ParseFailed(reader_type, message) => write!(f, "Failed to parse {} file, message: {}", reader_type, message),
        }
    }
}

impl Error for FileError {}