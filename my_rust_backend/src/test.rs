use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Novel {
    pub name: String,
    pub age: u32,
    pub favourite_food: Option<String>,
}