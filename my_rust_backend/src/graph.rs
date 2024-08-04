use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::fmt::{Display};
use serde::{Serialize, Deserialize};
use std::time::{Instant};

// Novels are the data associated with nodes of the graph
#[derive(Serialize, Deserialize, Clone)]
pub struct Novel {
    pub v_id: u16,
    pub title: String,
    pub seiyuu: HashSet<String>,
    pub staff: HashSet<String>,
    pub tag_cont: HashSet<String>,
    pub nsfw: bool
}

// Allow Novels to be displayed using println!
impl Display for Novel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id: {} | Title: {}\nVoice Actors: {:?}\nStaff: {:?}\nContent Tags: {:?}\nNSFW?: {:?}\n",
                 self.v_id, self.title, self.seiyuu, self.staff, self.tag_cont, self.nsfw.to_string())
    }
}

impl Novel {
    // The closer to 1 is the more similarities there are
    pub fn comparing(&self, other_novel: &Novel) -> u16 {
        // Finds the cardinality of the intersection of the same attribute set. Adds this to intersection_index
        let mut intersection_index: f32 = self.seiyuu.intersection(&other_novel.seiyuu).count() as f32;
        intersection_index += self.tag_cont.intersection(&other_novel.tag_cont).count() as f32;
        intersection_index += self.staff.intersection(&other_novel.staff).count() as f32;

        // In order to keep consistency between novels, we've decided to take the percentage
        // with respect with the smallest length of each attribute.
        let mut smallest_sizes: f32 = 0.0;
        if self.seiyuu.len() < other_novel.seiyuu.len() {
            smallest_sizes += self.seiyuu.len() as f32;
        }
        else{
            smallest_sizes += other_novel.seiyuu.len() as f32;
        }

        if self.staff.len() < other_novel.staff.len() {
            smallest_sizes += self.staff.len() as f32;
        }
        else{
            smallest_sizes += other_novel.staff.len() as f32;
        }

        if self.tag_cont.len() < other_novel.tag_cont.len() {
            smallest_sizes += self.tag_cont.len() as f32;
        }
        else{
            smallest_sizes += other_novel.tag_cont.len() as f32;
        }

        if smallest_sizes == 0.0 {
            return 0;
        }

        // To get the amount of work required for shortest path
        // because we want novels that are similar to each other:
        // 100 - (similarity percentage)
        (100.0 - ((intersection_index/smallest_sizes) * 100.0)) as u16
    }
}

pub trait FindNovel {
    fn find_novel(&self, vid: &u16) -> usize;
}

// Binary Search to find index of novel dependent on vector.
impl FindNovel for Vec<Novel> {
    // Uses binary search to find the index of novel with id vid in a vector Novels
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
    fn dijkstra(&self, source : &u16, terminal : &u16, novels : Vec<Novel>) -> (Vec<u16>, f64);
    fn bellman_ford(&self, source : &u16, terminal : &u16, novels : Vec<Novel>) -> (Vec<u16>, f64);
}

// TreeMap of <v_id, Vec<v_id, weight>>
impl Graph for BTreeMap<u16, Vec<(u16, u16)>> { // TODO: Figure out when to stop because the two are part to two different graphs
    fn dijkstra(&self, source : &u16, terminal : &u16, novels : Vec<Novel>) -> (Vec<u16>, f64) { // Self refers to the map
        let start_time = Instant::now();
        let mut path: Vec<u16> = Vec::new();
        if self[source].is_empty() || self[terminal].is_empty(){
            println!("EITHER SOURCE OR TERMINAL NOVEL HAS NO EDGE");
            return (path, start_time.elapsed().as_secs_f64()); // Returns an empty if the node is isolated.
        }

        let mut s: HashSet<u16> = HashSet::new(); // Computed id's
        let mut distance: Vec<i32> = vec![99999; novels.len()]; // Distance
        let mut previous: Vec<u16> = vec![0; novels.len()]; // Previous id's. Default set to 0 because no V0 exists
        distance[novels.find_novel(source)] = 0;

        let mut current_id = source;
        s.insert(*current_id);
        while !s.contains(terminal) {
            let neighbors: Vec<(u16, u16)> = self[current_id].clone();
            for node in neighbors {
                // If the node hasn't been visited yet and the weight of the new path is cheaper than originally, relax that vertex
                if !s.contains(&node.0) &&
                    distance[novels.find_novel(&node.0)] > (distance[novels.find_novel(&current_id)] + i32::from(node.1)){

                    distance[novels.find_novel(&node.0)] = distance[novels.find_novel(&current_id)] + i32::from(node.1);
                    previous[novels.find_novel(&node.0)] = *current_id;
                }
            }

            // Determine next vertex to visit by which has the smallest value in d[v]
            let mut smallest_weight = 99999;
            for i in 0..distance.len() {
                if smallest_weight > distance[i] && !s.contains(&novels[i].v_id){
                    smallest_weight = distance[i];
                    current_id = &novels[i].v_id;
                }
            }

            // If no change has occurred, the terminal vertex is disconnected from source and hence no path exists.
            if s.contains(current_id){
                break;
            }
            s.insert(current_id.clone());
        }

        // Find the path from terminal to source
        current_id = terminal;
        path.push(*terminal);
        println!("Distance Needed: {}", distance[novels.find_novel(current_id)]);
        if s.contains(terminal) && s.contains(source){
            while current_id != source {
                current_id = &previous[novels.find_novel(current_id)];
                path.push(*current_id);
            }
        }
        path.reverse();
        (path, start_time.elapsed().as_secs_f64())
    }

    fn bellman_ford(&self, source : &u16, terminal : &u16, novels : Vec<Novel>) -> (Vec<u16>, f64) {
        let start_time = Instant::now();
        let mut path: Vec<u16> = Vec::new();
        if self[source].is_empty() || self[terminal].is_empty(){
            println!("EITHER SOURCE OR TERMINAL NOVEL HAS NO EDGE");
            return (path, start_time.elapsed().as_secs_f64()); // Returns an empty if the node is isolated.
        }
        let mut distance: Vec<i32> = vec![99999; novels.len()]; // Distance
        let mut previous: Vec<u16> = vec![0; novels.len()]; // Previous id's. Default set to 0 because no V0 exists

        distance[novels.find_novel(source)] = 0;
        for _iterations_needed in 0..novels.len(){ // Number of iterations needed for Bellman-Ford to guarantee the shortest path
            let mut changed_made = false;
            for node in 0..novels.len(){ // Each iteration request every node in a graph to be visited.
                let edge_list = self[&novels[node].v_id].clone();
                for edge in edge_list{ // For every node, every edge in an edge list much be checked
                    if edge.0 != *source &&
                        distance[novels.find_novel(&edge.0)] > (distance[node] + i32::from(edge.1)){
                        // As long as the edge it's direct to isn't the source and the weight of the new path is cheaper than originally, relax that vertex

                        distance[novels.find_novel(&edge.0)] = distance[node] + i32::from(edge.1);
                        previous[novels.find_novel(&edge.0)] = novels[node].v_id.clone();
                        changed_made = true;
                    }
                }
            }
            // If no changes have been made by the end of the iteration, Bellman-Ford is said to be completed
            // earlier than usual
            if !changed_made{
                break;
            }
        }

        // Finds the path starting from terminal to source
        let mut current_id = terminal;
        path.push(*terminal);
        println!("Distance Needed: {}", distance[novels.find_novel(current_id)]);
        if distance[novels.find_novel(terminal)] != 99999 && distance[novels.find_novel(source)] != 99999{
            while current_id != source  && distance[novels.find_novel(current_id)] != 99999{
                current_id = &previous[novels.find_novel(current_id)];
                path.push(*current_id);
            }
        }
        path.reverse();
        (path, start_time.elapsed().as_secs_f64())
    }
}
