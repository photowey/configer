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

/// @since 0.1.0
pub mod domain;
/// @since 0.1.0
pub mod env;
/// @since 0.1.0
pub mod error;
/// @since 0.3.0
pub mod reader;

// ----------------------------------------------------------------

#[cfg(test)]
mod tests;
#[cfg(test)]
mod converter_tests;
#[cfg(test)]
#[cfg(feature = "usetoml")]
mod toml_tests;
