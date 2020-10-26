// mod phase;

use super::phase::Phase;
use std::fs;

const FILE_PATH: &str = "test/fixtures/phases.yml";

fn parse_phases(s: &str) -> Result<Vec<Phase>, serde_yaml::Error> {
  let parsed_phases: Vec<Phase> = serde_yaml::from_str(&s)?;
  Ok(parsed_phases)
}

pub fn load_phase() -> Phase {
  let result = fs::read_to_string(FILE_PATH);
  match result {
    Ok(file_contents) => {
      let phases = parse_phases(&file_contents);
      println!("{:?}", phases);
    }
    Err(e) => {
      println!("{:?}", e);
    }
  };

  Phase {
    name: "Batxillerat".to_string(),
    code: "bat".to_string(),
  }
}
