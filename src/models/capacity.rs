use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Capacity {
    pub capacity: u32,
}
