use std::time::Duration;

use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message as WsMessage, WebSocketError};
use serde_json::Value;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::spawn_local;
use yew::{self, html, Component, Context, Html};
use web_logger_shared::Message;

pub struct Logger;

impl Component for Logger {
  type Message = Value;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let callback = ctx.link().callback(|m: Value| m);

    spawn_local(async move {
      loop {
        let mut ws = WebSocket::open("ws://localhost:1234").unwrap();
        let (_, mut read) = ws.split();
        while let Some(msg) = read.next().await {
          match msg {
            Ok(WsMessage::Text(data)) => {
              let message: Message = serde_json::from_str(&data).unwrap();
              let value = serde_json::from_str(&message.value).unwrap();
              callback.emit(value);
            }
            Err(WebSocketError::ConnectionError) => {
              gloo_timers::future::TimeoutFuture::new(1_000).await;
              break;
            }
            _ => {}
          }
        }

        std::mem::forget(read);
      }
    });

    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div>{ "Hey world" }</div>
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    log::info!("{msg:?}");
    false
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<Logger>();
}
