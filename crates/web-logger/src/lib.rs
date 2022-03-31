use std::{
  net::{TcpListener, TcpStream},
  sync::{Arc, Mutex},
  thread,
};

use log::{kv::Key, Log};
use tungstenite::{accept, Message as WsMessage, WebSocket};
use web_logger_shared::Message;

#[derive(Default)]
struct WebLoggerInner {
  clients: Vec<WebSocket<TcpStream>>,
  buffer: Vec<WsMessage>,
}

struct WebLogger {
  inner: Arc<Mutex<WebLoggerInner>>,
  _handle: thread::JoinHandle<()>,
}

impl WebLogger {
  fn new() -> Self {
    let server = TcpListener::bind("localhost:1234").unwrap();
    let inner = Arc::new(Mutex::new(WebLoggerInner::default()));
    let inner_ref = inner.clone();

    let _handle = thread::spawn(move || {
      for stream in server.incoming().filter_map(Result::ok) {
        let mut websocket = accept(stream).unwrap();
        let mut inner = inner_ref.lock().unwrap();
        for msg in inner.buffer.drain(..) {
          websocket.write_message(msg).unwrap();
        }
        inner.clients.push(websocket);
      }
    });

    WebLogger { inner, _handle }
  }
}

impl Log for WebLogger {
  fn enabled(&self, _metadata: &log::Metadata) -> bool {
    true
  }

  fn log(&self, record: &log::Record) {
    let kvs = record.key_values();
    let v = kvs.get(Key::from_str("s")).unwrap();
    let value = serde_json::value::to_value(&v).unwrap();
    let message = Message {
      value: Some(value),
      message: "".into(),
      level: record.level(),
      file: record.file().map(|s| s.to_owned()),
      module: record.module_path().map(|s| s.to_owned()),
      line: record.line(),
    };
    let message_str = serde_json::to_string(&message).unwrap();
    let ws_message = WsMessage::Text(message_str);

    let mut inner = self.inner.lock().unwrap();
    if inner.clients.is_empty() {
      inner.buffer.push(ws_message);
    } else {
      for client in inner.clients.iter_mut() {
        client.write_message(ws_message.clone()).unwrap();
      }
    }
  }

  fn flush(&self) {
    // no-op
  }
}

pub fn init() {
  log::set_boxed_logger(Box::new(WebLogger::new())).unwrap();
  log::set_max_level(log::LevelFilter::Info);
}
