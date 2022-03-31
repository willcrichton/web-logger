use std::{
  any::TypeId,
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

use linkme::distributed_slice;
use log::kv::{ToValue, Value as LogValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use yew::{
  self, function_component, html, use_state, virtual_dom::VComp, Callback, Component,
  Context, Html, Properties,
};

#[distributed_slice]
pub static COMPONENTS: [fn(&TypedJson) -> Option<VComp>] = [..];

macro_rules! into_component {
  ($ty:ty, $props:ty, $comp:ty, $id:ident, $id2:ident) => {
    #[distributed_slice($crate::components::COMPONENTS)]
    static $id: fn(&TypedJson) -> Option<yew::virtual_dom::VComp> = $id2;
    fn $id2(t: &TypedJson) -> Option<yew::virtual_dom::VComp> {
      let id = $crate::components::type_id::<$ty>();
      if id == t.tag {
        Some(yew::virtual_dom::VComp::new::<$comp>(
          std::rc::Rc::new(<$props>::default()),
          yew::NodeRef::default(),
          None,
        ))
      } else {
        None
      }
    }
  };
}


pub trait IntoComponent {
  fn into_component(self) -> VComp;
}

fn type_id<T: 'static>() -> u64 {
  let mut hasher = DefaultHasher::new();
  TypeId::of::<T>().hash(&mut hasher);
  hasher.finish()
}

#[derive(Serialize, Deserialize)]
pub struct TypedJson {
  tag: u64,
  value: serde_json::Value,
}

impl TypedJson {
  pub fn new<T: Serialize + 'static>(t: T) -> Self {
    TypedJson {
      tag: type_id::<T>(),
      value: serde_json::to_value(t).unwrap(),
    }
  }

  pub fn to_vcomp(&self) -> VComp {
    for f in COMPONENTS {
      if let Some(v) = f(self) {
        return v;
      }
    }    

    unimplemented!("{:?}", self.value)
  }
}

impl ToValue for TypedJson {
  fn to_value(&self) -> LogValue<'_> {
    LogValue::from_serde(self)
  }
}


#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct Circle {
  pub radius: f64,
}

into_component!(Circle, CircleProps, CircleView, MAKE_CIRCLE, make_circle);

#[derive(Properties, PartialEq, Default)]
pub struct CircleProps {
  circle: Circle,
}

#[function_component(CircleView)]
pub fn circle_view(props: &CircleProps) -> Html {
  let &Circle { radius } = &props.circle;
  let r = format!("{}", radius);
  html! {
    <svg viewBox={format!("0 0 {r} {r}", r=r)}>
      <circle cx={ r.clone() } cy={ r.clone() } r={ r.clone() } />
    </svg>
  }
}