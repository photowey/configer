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
use std::path::PathBuf;

use crate::domain::Table;
use crate::error::FileError;

// ----------------------------------------------------------------

#[cfg(feature = "usetoml")]
pub mod toml;

// ----------------------------------------------------------------

pub trait ConfigReader {
    fn name(&self) -> String;
    fn suffix(&self) -> String;
    fn supports(&self, suffix: &str) -> bool;

    fn read_from_str(&self, data: &str) -> Result<Table, FileError>;

    fn read_from_path(&self, path: &str) -> Result<Table, FileError> {
        let canon = PathBuf::from(path).canonicalize().map_err(|_| FileError::InvalidPath(path.to_string()))?;
        let content = fs::read_to_string(canon).map_err(|_| FileError::ReadFailed(path.to_string()))?;
        self.read_from_str(&content)
    }
}

// ----------------------------------------------------------------

pub trait ReaderRegistry {
    fn register(&mut self, reader: Box<dyn ConfigReader>);
    fn try_acquire(&self, suffix: &str) -> Option<&dyn ConfigReader>;
    fn try_acquires(&self) -> Vec<&dyn ConfigReader>;
}

pub struct ConfigReaderRegistry {
    readers: HashMap</*suffix*/String, Box<dyn ConfigReader>>,
}

impl ConfigReaderRegistry {
    pub fn new() -> Self {
        Self {
            readers: HashMap::new(),
        }
    }
}

impl Default for ConfigReaderRegistry {
    fn default() -> Self {
        ConfigReaderRegistry::new()
    }
}

// ----------------------------------------------------------------

impl ReaderRegistry for ConfigReaderRegistry {
    fn register(&mut self, reader: Box<dyn ConfigReader>) {
        self.readers.insert(reader.suffix(), reader);
    }

    fn try_acquire(&self, suffix: &str) -> Option<&dyn ConfigReader> {
        self.readers.get(suffix).map(|r| r.as_ref())
    }

    fn try_acquires(&self) -> Vec<&dyn ConfigReader> {
        self.readers.values().map(|r| r.as_ref() as &dyn ConfigReader).collect()
    }
}