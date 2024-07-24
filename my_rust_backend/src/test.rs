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
    // The closer to 0 is the more similarities there are
    pub fn comparing(&self, other_novel: &Novel) -> i32{
        let mut similarity_index = 250;

        for n in 0..self.staff.len() {
            for m in 0..other_novel.staff.len(){
                if self.staff[n] == other_novel.staff[m]{
                    similarity_index -= 2;
                    break;
                }
            }
        }

        for n in 0..self.seiyuu.len() {
            for m in 0..other_novel.seiyuu.len(){
                if self.seiyuu[n] == other_novel.seiyuu[m]{
                    similarity_index -= 2;
                    break;
                }
            }
        }

        for n in 0..self.tag_cont.len() {
            for m in 0..other_novel.tag_cont.len(){
                if self.tag_cont[n] == other_novel.tag_cont[m]{
                    similarity_index -= 2;
                    break;
                }
            }
        }

        for n in 0..self.tag_tech.len() {
            for m in 0..other_novel.tag_tech.len(){
                if self.tag_tech[n] == other_novel.tag_tech[m]{
                    similarity_index -= 1;
                    break;
                }
            }
        }
        similarity_index
    }
}