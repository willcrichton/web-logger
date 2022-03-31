use log::{as_serde, info};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct Person {
  first: String,
  last: String
}

fn main() {
  web_logger::init();

  let p = Person { first: "Will".into(), last: "Crichton".into() };
  info!(s = as_serde!(&p); "");

  std::thread::sleep(Duration::from_secs(10));
}
