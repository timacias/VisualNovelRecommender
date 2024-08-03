// 'tags_vn' has been sorted based on vid

use std::collections::{HashMap, HashSet};
// use std::fmt::Debug;
use std::fs::File;
use std::mem::swap;
use std::ops::Index;
use crate::test;
use test::Novel;

pub fn reading_csv() -> (Vec<Novel>, HashMap<String, u16>) {
    // Make vector of vn objects so that we can implement vn searching.
    let mut novels: Vec<Novel> = Vec::new();
    // Make a map of vn titles to v_ids for faster v_id lookups by title
    let mut titles_to_ids : HashMap<String, u16> = HashMap::new();

    // GET VISUAL NOVEL V_ID AND TITLE from "vn_titles" ////////////////////////////////////////////
    let vn_file = File::open("../database/db/vn_titles").unwrap();
    let mut vn_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(vn_file);

    let mut v_id : u16 = 1;
    let mut languages = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    let mut latin_titles: Vec<String> = Vec::new();
    let mut title_to_use = "".to_string();

    for vn in vn_reader.records() {
        let record = vn.unwrap();
        let curr_id = record
            .index(0)[1..]
            .parse()
            .unwrap();

        if v_id != curr_id {
            for i in 0..languages.len() {
                if languages[i] == "en" {
                    title_to_use = titles[i].clone();
                    break;
                }
                else if languages[i] == "ja" {
                    if &*latin_titles[i] == "\\N" {
                        title_to_use = titles[i].clone();
                    }
                    else{
                        title_to_use = latin_titles[i].clone();
                    }
                    break;
                }

            }
            if title_to_use.is_empty() {
                if &*latin_titles[0] == "\\N" {
                    title_to_use = titles[0].clone();
                }
                else{
                    title_to_use = latin_titles[0].clone();
                }
            }
            // Create the Novel struct and add it to the vector
            novels.push(Novel {
                v_id,
                title : title_to_use.clone(),
                seiyuu: HashSet::new(),
                staff: HashSet::new(),
                tag_cont: HashSet::new(),
                nsfw : false
            });
            // Add a title, v_id pair to the map
            titles_to_ids.insert(title_to_use, v_id);

            v_id = curr_id;
            languages.clear();
            titles.clear();
            latin_titles.clear();
            title_to_use = "".to_string();
        }

        languages.push(record.index(1).to_string());
        titles.push(record.index(3).to_string());
        latin_titles.push(record.index(4).to_string());
    }


    // GET ALL ALIASES TO SEARCH FOR STAFF AND SEIYUU from "staff_alias" ///////////////////////////
    let alias_file = File::open("../database/db/staff_alias").unwrap();
    let mut alias_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(alias_file);

    let mut names: Vec<String> = vec!["N/A".to_string(); 46643];
    for alias in alias_reader.records() {
        let record = alias.unwrap();
        let index_str = record.index(1).to_string();
        let index_int = index_str.parse::<usize>().unwrap();
        let name;
        if record.index(3).to_string() != "\\N" {
            name = record.index(3).to_string();
        } else {
            name = record.index(2).to_string();
        }
        names[index_int] = name;
    }

    // GET SEIYUU NAMES from "vn_seiyuu" ///////////////////////////////////////////////////////////
    v_id = 1;
    let mut novels_index = 0usize;
    let seiyuu_file = File::open("../database/db/vn_seiyuu").unwrap();
    let mut seiyuu_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .from_reader(seiyuu_file);

    let mut seiyuu_names = HashSet::new();
    for name in seiyuu_reader.records() {
        let record = name.unwrap();
        let curr_id = record
            .index(0)[1..]
            .parse()
            .unwrap();

        if v_id != curr_id {
            // v_id does not exactly correlate to the indices of novels
            // This must be accounted for
            while novels[novels_index].v_id != v_id {
                novels_index += 1;
            }
            
            swap(&mut novels[novels_index].seiyuu, &mut seiyuu_names);
            v_id = curr_id;
        }

        let index_str = record.index(1).to_string();  // `parse()` works with `&str` and `String`!
        let index_int = index_str.parse::<usize>().unwrap();
        let name = names[index_int].clone();

        if !seiyuu_names.contains(&name) && name != "N/A"{
            seiyuu_names.insert(name);
        }
    }
    // Same issue as above (fence post problem w/ for-loop)
    swap(&mut novels[novels_index + 2].seiyuu, &mut seiyuu_names);

    // GET STAFF NAMES from "vn_staff" ///////////////////////////////////////////////////////////
    v_id = 1;
    let mut novels_index = 0usize;
    let staff_file = File::open("../database/db/vn_staff").unwrap();
    let mut staff_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .from_reader(staff_file);

    let mut staff_names = HashSet::new();
    for name in staff_reader.records() {
        let record = name.unwrap();
        let curr_id = record
            .index(0)[1..]
            .parse()
            .unwrap();

        if v_id != curr_id {
            // v_id does not exactly correlate to the indices of novels
            // This must be accounted for
            while novels[novels_index].v_id != v_id {
                novels_index += 1;
            }

            swap(&mut novels[novels_index].staff, &mut staff_names);
            v_id = curr_id;
        }

        let index_str = record.index(1).to_string();  // `parse()` works with `&str` and `String`!
        let index_int = index_str.parse::<usize>().unwrap();
        let name = names[index_int].clone();

        if !staff_names.contains(&name) && name != "N/A"{
            staff_names.insert(name);
        }
    }
    // Same issue as above (fence post problem w/ for-loop)
    swap(&mut novels[novels_index + 2].staff, &mut staff_names);

    // GET LIST OF TAGS from "tags" ////////////////////////////////////////////////////////////////
    let tags_file = File::open("../database/db/tags").unwrap();
    let mut tags_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(tags_file);

    let mut tag_names: Vec<String> = vec!["N/A".to_string(); 3918];
    let mut tag_category: Vec<String> = vec!["N/A".to_string(); 3918];
    for tag in tags_reader.records() {
        let record = tag.unwrap();

        let index_int = record
            .index(0)[1..]
            .parse::<usize>()
            .unwrap();
        tag_names[index_int] = record.index(5).to_string();
        tag_category[index_int] = record.index(1).to_string();
    }

    // MAP TAGS TO NOVELS using "tags_vn" //////////////////////////////////////////////////////////
    v_id = 1;
    let mut v_is_ero: bool = false;
    novels_index = 0usize;
    let tags_vn_file = File::open("../database/db/tags_vn").unwrap();
    let mut tags_vn_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(tags_vn_file);

    let mut tags_vn_cont_names = HashSet::new();
    for name in tags_vn_reader.records() {
        let record = name.unwrap();
        let curr_id = record
            .index(2)
            .parse()
            .unwrap();

        if v_id != curr_id {
            // v_id does not exactly correlate to the indices of novels
            // This must be accounted for
            while novels[novels_index].v_id != v_id {
                novels_index += 1;
            }

            swap(&mut novels[novels_index].tag_cont, &mut tags_vn_cont_names);
            swap(&mut novels[novels_index].nsfw, &mut v_is_ero);
            v_id = curr_id;
            v_is_ero = false;
        }

        let index_int = record
            .index(1)[1..]
            .parse::<usize>()
            .unwrap();
        let name = tag_names[index_int].clone();
        let category = tag_category[index_int].clone();

        if !tags_vn_cont_names.contains(&name) && name != "N/A" && record.index(4).parse::<i8>().unwrap() > 1 {
            if category != "ero" {
                if category == "cont" {
                    tags_vn_cont_names.insert(name);
                }
            }
            else {
                v_is_ero = true;
            }
        }
    }
    // Same issue as with seiyuu
    swap(&mut novels[novels_index + 2].tag_cont, &mut tags_vn_cont_names);
    swap(&mut novels[novels_index + 2].nsfw, &mut v_is_ero);


    // Move any sfw Novels into a new vector
    let mut sfw_novels = Vec::new();
    for novel in novels {
        if !novel.nsfw {
            sfw_novels.push(novel);
        }
        // Check to see if any sus novels are making it through
        else if !novel.nsfw && novel.title.to_lowercase().contains("sex") {
            println!("NSFW title found! {}\n\n\n\n", novel);
            //panic!("NSFW title found!")
        }
    }

    /*for novel in &sfw_novels {
        println!("{}", novel);
    }*/
    // After dealing with the horrors of vndb, return the lovely vector of Novels
    (sfw_novels, titles_to_ids)
}
