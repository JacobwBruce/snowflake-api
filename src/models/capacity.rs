use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Capacity {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub num_of_vendors_needed: i32,
    pub tsa_needed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewCapacity {
    pub name: String,
    pub location: String,
    pub num_of_vendors_needed: i32,
    pub tsa_needed: bool,
}
