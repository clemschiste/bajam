use serde::{Deserialize, Serialize};
// SQL struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Escapade {
    pub id: i64,
    pub name: String,
}
