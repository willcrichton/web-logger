use log::kv::{ToValue, Value as LogValue};
use serde::{Deserialize, Serialize};
use type_uuid::TypeUuid;
use yew::{self, virtual_dom::VComp};

use super::stdlib::COMPONENTS;

#[macro_export]
macro_rules! into_component {
  ($ty:ty, $props:ty, $comp:ty, $id:ident) => {
    fn $id(t: &Value) -> yew::virtual_dom::VComp {
      let t: $ty = serde_json::from_value(t.clone()).unwrap();
      yew::virtual_dom::VComp::new::<$comp>(
        std::rc::Rc::new(<$props>::new(t)),
        yew::NodeRef::default(),
        None,
      )
    }
  };
}

pub trait IntoComponent {
  fn into_component(self) -> VComp;
}

#[derive(Serialize, Deserialize)]
pub struct TypedJson {
  tag: type_uuid::Bytes,
  value: serde_json::Value,
}

impl TypedJson {
  pub fn new<T: Serialize + TypeUuid + 'static>(t: T) -> Self {
    TypedJson {
      tag: <T as TypeUuid>::UUID,
      value: serde_json::to_value(t).unwrap(),
    }
  }

  pub fn to_vcomp(&self) -> VComp {
    COMPONENTS.with(|components| {
      let f = components.get(&self.tag).unwrap();
      f(&self.value)
    })
  }
}

impl ToValue for TypedJson {
  fn to_value(&self) -> LogValue<'_> {
    LogValue::from_serde(self)
  }
}
