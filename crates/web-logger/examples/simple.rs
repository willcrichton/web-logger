use log::{as_serde, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Education {
  highschool: Option<String>,
  college: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Person {
  first: String,
  last: String,
  education: Education,
}

fn main() {
  web_logger::init();

  info!("Building a person...");

  let p = Person {
    first: "Will".into(),
    last: "Crichton".into(),
    education: Education {
      highschool: Some("Valley High".into()),
      college: Some("CMU".into()),
    },
  };
  info!(s = as_serde!(p); "");

  info!("Person is built!");

  web_logger::flush();
}
