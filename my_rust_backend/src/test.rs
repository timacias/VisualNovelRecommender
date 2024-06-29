use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub favourite_food: Option<String>,
}