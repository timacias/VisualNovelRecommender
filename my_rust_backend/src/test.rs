// Ord implementation (required for usage in BTreeMap) inspired by https://stackoverflow.com/questions/29884402#29884582

use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Pointer};
use serde::{Serialize, Deserialize};

// Novels are the data associated with nodes of the graph
#[derive(Serialize, Deserialize, Clone)]
pub struct Novel {
    pub v_id: u16,
    pub title: String,
    pub seiyuu: HashSet<String>,
    pub tag_cont: HashSet<String>,
    pub nsfw: bool
}

impl Display for Novel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id: {} | Title: {}\nVoice Actors: {:?}\nContent Tags: {:?}\nNSFW?: {:?}\n",
                 self.v_id, self.title, self.seiyuu, self.tag_cont, self.nsfw.to_string())
    }
}

impl Novel {
    // The closer to 1 is the more similarities there are
    pub fn comparing(&self, other_novel: &Novel) -> u8 {
        let similarity_index = 126;

        let mut intersection_index = self.seiyuu.intersection(&other_novel.seiyuu).count();
        intersection_index += self.tag_cont.intersection(&other_novel.tag_cont).count();

        let return_val = similarity_index - intersection_index;
        if return_val <= 0 {
            panic!("Edge weight is zero or less!")
        }

        return_val as u8
    }
}

pub trait FindNovel {
    fn find_novel(&self, vid: &u16) -> usize;
}

// Binary Search to find index of novel dependent on vector.
impl FindNovel for Vec<Novel> {
    fn find_novel(&self, vid: &u16) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low <= high {
            let mid = low + (high - low) / 2;
            if self[mid].v_id == *vid {
                return mid;
            }
            if self[mid].v_id < *vid {
                low = mid + 1;
            }
            else{
                high = mid - 1;
            }
        }
        99999 // This instead of -1 for id not found.
    }
}

pub trait Graph {
    // TODO: Implement Dijkstra's and Bellman-Ford algorithms
    fn dijkstra(&self, source : &u16, terminal : &u16, novels : Vec<Novel>) -> Vec<u16>;
    fn bellman_ford(&self, source : &u16, terminal : &u16, novels : Vec<Novel>) -> Vec<u16>;
}

// TreeMap of <v_id, Vec<v_id, weight>>
impl Graph for BTreeMap<u16, Vec<(u16, u8)>> {
    fn dijkstra(&self, source : &u16, terminal : &u16, novels : Vec<Novel>) -> Vec<u16> { // Self refers to the map
        let path = Vec::new();
        let mut s: HashSet<u16> = HashSet::new(); // Computed id's

        let mut distance: Vec<i32> = vec![99999; self.len()]; // Distance
        let mut previous: Vec<u16> = vec![0; self.len()]; // Previous id's. Default set to 0 because no V0 exists
        distance[novels.find_novel(source)] = 0;

        let mut current_id = source;
        while !s.contains(terminal) {
            let neighbors: Vec<(u16, u8)> = self[current_id].clone();
            for node in neighbors {
                if !s.contains(&node.0) &&
                    distance[novels.find_novel(&node.0)] > (distance[novels.find_novel(&current_id)] + i32::from(node.1)){

                    distance[novels.find_novel(&node.0)] = distance[novels.find_novel(&current_id)] + i32::from(node.1);
                    previous[novels.find_novel(&node.0)] = *current_id;
                }
            }
            let mut smallest_weight = 99999;
            for i in 0..distance.len() {
                if smallest_weight > distance[i] && !s.contains(&novels[i].v_id){
                    smallest_weight = distance[i];
                    current_id = &novels[i].v_id;
                }
            }
            s.insert(current_id.clone());
        }
        path
    }

    fn bellman_ford(&self, source : &u16, terminal : &u16, novels : Vec<Novel>) -> Vec<u16> {
        todo!();
    }

}
