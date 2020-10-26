mod models;

use models::{load_phase, Phase};

fn main() {
  let batxillerat = Phase {
    name: "Batxillerat".to_string(),
    code: "bat".to_string(),
  };

  let s = serde_yaml::to_string(&batxillerat);

  println!("Hello, Timerambler!");
  // println!("{:?}", batxillerat);
  // println!("{:?}", s);

  load_phase();
}
