use serde::{Serialize, Deserialize};

// Add more attributes here as necessary
#[derive(Serialize, Deserialize, Clone)]
pub struct Novel {
    pub v_id: u16,
    pub title: String
}

// TODO: Implement graph data structure