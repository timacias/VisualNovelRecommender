// Simple graph data structure inspired by : https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

use serde::{Serialize, Deserialize};

// Novels are the data associated with nodes of the graph
#[derive(Serialize, Deserialize, Clone)]
pub struct Novel {
    pub v_id: u16,
    pub title: String,
    pub staff: Vec<String>,
    pub seiyuu: Vec<String>,
    pub tags: Vec<String>,
    pub nsfw: bool
}

// TODO: Implement graph data structure