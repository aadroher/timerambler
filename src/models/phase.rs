use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Phase {
  pub name: String,
  pub code: String,
}
