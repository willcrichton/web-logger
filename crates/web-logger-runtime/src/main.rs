use std::rc::Rc;

use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message as WsMessage, WebSocketError};
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use web_logger_shared::Message;
use yew::{
  self, function_component, html, use_state, Callback, Component, Context, Html,
  Properties,
};

fn read_socket(ctx: &Context<Logger>) {
  let callback = ctx.link().callback(|m: Message| m);
  spawn_local(async move {
    loop {
      let ws = WebSocket::open("ws://localhost:1234").unwrap();
      let (_, mut read) = ws.split();
      while let Some(msg) = read.next().await {
        match msg {
          Ok(WsMessage::Text(data)) => {
            let message: Message = serde_json::from_str(&data).unwrap();
            callback.emit(message);
          }
          Err(WebSocketError::ConnectionError) => {
            gloo_timers::future::TimeoutFuture::new(1_000).await;
            break;
          }
          _ => {}
        }
      }

      // TODO: this is a memory leak, but we get a panic if we don't do this.
      std::mem::forget(read);
    }
  });
}

#[derive(Properties, PartialEq)]
struct JsonProps {
  json: Value,
}

#[function_component(JsonView)]
fn json_view(props: &JsonProps) -> Html {
  match &props.json {
    Value::Null => html! { { "null" } },
    Value::Number(n) => html! { { n } },
    Value::Bool(b) => html! { { b } },
    Value::String(s) => html! { { s } },
    Value::Object(obj) => {
      let expanded = use_state(|| false);
      let on_click = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
      };

      let left = html! { { "{" } };
      let inner = if !*expanded {
        html! {
          <span onclick={on_click}>{
            obj.iter().map(|(k, _)| html! { <span class="obj-key">{k}</span> }).collect::<Html>()
          }</span>
        }
      } else {
        obj
          .iter()
          .map(|(k, v)| {
            let expanded = use_state(|| false);
            let on_click = {
              let expanded = expanded.clone();
              Callback::from(move |_| expanded.set(!*expanded))
            };

            html! {
              <div>
                { k }
                <span class="arrow" onclick={on_click.clone()}>
                  if *expanded {
                    {"▼"}
                  } else {
                    {"▶"}
                  }
                </span>
                if *expanded {
                  <div class={"nested"}>
                    <JsonView json={ v.clone() } />
                  </div>
                }
              </div>
            }
          })
          .collect::<Html>()
      };
      let right = html! { { "}" } };

      [left, inner, right].into_iter().collect::<Html>()
    }
    Value::Array(_) => todo!(),
  }
}

#[derive(Properties, PartialEq)]
struct MessageProps {
  message: Rc<Message>,
}

#[function_component(MessageView)]
fn message_view(props: &MessageProps) -> Html {
  html! { <JsonView json={props.message.value.as_ref().unwrap().clone()} /> }
}

pub struct Logger {
  logs: Vec<Rc<Message>>,
}

impl Component for Logger {
  type Message = Message;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    read_socket(ctx);
    Self { logs: Vec::new() }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div>
        { self.logs.iter().map(|log| {
          html! {
            <MessageView message={ log.clone() } />
          }
        }).collect::<Html>() }
      </div>
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    self.logs.push(Rc::new(msg));
    true
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<Logger>();
}
