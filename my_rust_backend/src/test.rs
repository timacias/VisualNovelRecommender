// Simple graph data structure inspired by : https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

use serde::{Serialize, Deserialize};

// Novels are the data associated with nodes of the graph
#[derive(Serialize, Deserialize, Clone)]
pub struct Novel {
    pub v_id: u16,
    pub title: String,
    pub staff: Vec<String>,
    pub seiyuu: Vec<String>,
    pub tag_cont: Vec<String>,
    pub tag_tech: Vec<String>,
    pub nsfw: bool
}

// TODO: Implement graph data structure
impl Novel{
    pub fn print_novel(&self){
        println!("Id: {} | Title: {}\nStaff: {:?}\nVoice Actors: {:?}\nContent Tags: {:?}\nTechnical Tags: {:?}\nNSFW?: {:?}\n",
                 self.v_id, self.title, self.staff, self.seiyuu, self.tag_cont, self.tag_tech, self.nsfw.to_string());
    }
}

impl Novel{
    // The closer to 1 is the more similarities there are
    pub fn comparing(&self, other_novel: &Novel) -> i32{
        let mut similarity_index = 126;
        /*  for n in 0..self.staff.len() {
            if other_novel.staff.contains(&self.staff[n]){
                similarity_index -= 2;
            }
        } */
        if self.seiyuu.len() < other_novel.seiyuu.len(){
            for seiyuu in &self.seiyuu{
                if other_novel.seiyuu.binary_search(&seiyuu).is_ok(){
                    similarity_index -= 1;
                }
            }
        }
        else{
            for seiyuu in &other_novel.seiyuu{
                if self.seiyuu.binary_search(&seiyuu).is_ok(){
                    similarity_index -= 1;
                }
            }
        }

        if self.tag_cont.len() < other_novel.tag_cont.len() {
            for tag in &self.tag_cont{
                if other_novel.tag_cont.binary_search(&tag).is_ok(){
                    similarity_index -= 1;
                }
            }
        }
        else{
            for tag in &other_novel.tag_cont{
                if self.tag_cont.binary_search(&tag).is_ok(){
                    similarity_index -= 1;
                }
            }
        }

        /*for n in 0..self.tag_tech.len() {
            if other_novel.tag_tech.contains(&self.tag_tech[n]){
                similarity_index -= 1;
            }
        }*/
        similarity_index
    }
}