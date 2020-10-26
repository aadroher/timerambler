// mod phase;

use super::phase::Phase;
use std::fs;

pub fn load_phase() -> Phase {
  let file_path = "../../test/fixtures/";
  let result = fs::read_to_string(file_path);
  match result {
    Ok(file_contents) => println!("{}", file_contents),
    Err(e) => println!("{:?}", e),
  };

  Phase {
    name: "Batxillerat".to_string(),
    code: "bat".to_string(),
  }
}
