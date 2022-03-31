use std::any::Any;

use log::Level;
use serde::{Deserialize, Serialize};
use yew::Component;

// pub trait IntoComponent {
//   type Output: Component<Message = (), Properties = ()>;
//   fn into_component(self) -> Self::Output;
// }

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
  pub value: Option<serde_json::Value>,
  pub message: String,
  pub level: Level,
  pub file: Option<String>,
  pub module: Option<String>,
  pub line: Option<u32>,
}
