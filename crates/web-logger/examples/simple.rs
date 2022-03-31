use log::{as_serde, info};
use serde::{Deserialize, Serialize};
use web_logger::as_html;
use web_logger_shared::components::stdlib::{Chart, Circle};

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

  let c = Circle { radius: 5. };
  let c2 = Circle { radius: 10. };
  info!(s = as_html!(c); "");
  info!(s = as_html!(c2); "");

  info!("Generating data...");
  let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 4.0), (3.0, 3.0)];
  info!(s = as_html!(Chart { data }); "");
  info!("Plotted!");

  web_logger::flush();
}
