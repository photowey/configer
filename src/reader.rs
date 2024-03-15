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
    fn register<T>(&mut self, reader: T) where T: ConfigReader;
    fn try_acquire(&self, suffix: &str) -> Option<Box<dyn ConfigReader>>;
}

