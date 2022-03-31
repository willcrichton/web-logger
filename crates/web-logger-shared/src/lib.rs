//! Types shared between runtime and main logger crate.
//!
//! Has to be in a separate crate so we don't compile logger crate dependencies
//! in the WASM target.

use log::Level;
use serde::{Deserialize, Serialize};

pub mod components;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
  pub value: Option<serde_json::Value>,
  pub message: String,
  pub level: Level,
  pub file: Option<String>,
  pub module: Option<String>,
  pub line: Option<u32>,
}
