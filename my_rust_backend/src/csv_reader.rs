// 'tags_vn' has been sorted based on vid

// use std::fmt::Debug;
use std::fs::File;
use std::mem::swap;
use std::ops::Index;
use crate::test;
use test::Novel;

pub fn reading_csv() -> Vec<Novel> {
    // Make vector of vn objects so that we can implement vn searching.
    let mut novels: Vec<Novel> = Vec::new();

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


    // TODO: Every for-loop that iterates through a csv file skips the final novel, fix needed
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
                title : title_to_use,
                staff: vec![],
                seiyuu: vec![],
                tags: vec![],
                nsfw : false
            });

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

    // for i in 0..names.len(){
    //     println!("{}: {}", i+1, names[i]);
    // }

    // GET STAFF (EXCLUDING SEIYUU) NAMES FOR EACH VN from "vn_staff" //////////////////////////////
    v_id = 1;
    let mut novels_index = 0usize;
    let staff_file = File::open("../database/db/vn_staff").unwrap();
    let mut staff_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .from_reader(staff_file);

    let mut staff_names = Vec::new();
    for name in staff_reader.records() {
        let record = name.unwrap();
        let curr_id = record
            .index(0)[1..]
            .parse()
            .unwrap();

        if v_id != curr_id {
            // println!("ID: {}, Staff:", v_id);
            // for name in &staff_names {
            //     println!("{}", name);
            // }
            // println!();

            // v_id does not exactly correlate to the indices of novels
            // This must be accounted for
            while novels[novels_index].v_id != v_id {
                novels_index += 1;
            }

            //println!("I: {}, vid: {}", novels_index, v_id);
            swap(&mut novels[novels_index].staff, &mut staff_names);
            v_id = curr_id;

            // No need to clear staff_names since it is being swapped with an empty vector
            // staff_names.clear();
        }

        let index_str = record.index(1).to_string();  // `parse()` works with `&str` and `String`!
        let index_int = index_str.parse::<usize>().unwrap();
        let name = names[index_int].clone();

        let mut in_list = false;
        for title in &staff_names {
            if title == &name {
                in_list = true;
            }
        }
        if !in_list && name != "N/A"{
            staff_names.push(name);
        }
    }

    swap(&mut novels[novels_index + 11].staff, &mut staff_names);
    
    // GET SEIYUU NAMES from "vn_seiyuu" ///////////////////////////////////////////////////////////
    v_id = 1;
    novels_index = 0usize;
    let seiyuu_file = File::open("../database/db/vn_seiyuu").unwrap();
    let mut seiyuu_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .from_reader(seiyuu_file);

    let mut seiyuu_names = Vec::new();
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

        let mut in_list = false;
        for title in &seiyuu_names {
            if title == &name {
                in_list = true;
            }
        }
        if !in_list && name != "N/A"{
            seiyuu_names.push(name);
        }
    }
    // Same issue as above (with vn_staff)
    swap(&mut novels[novels_index + 3].seiyuu, &mut seiyuu_names);

    // GET LIST OF TAGS from "tags" ////////////////////////////////////////////////////////////////
    let tags_file = File::open("../database/db/tags").unwrap();
    let mut tags_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(tags_file);

    let mut tag_names: Vec<String> = vec!["N/A".to_string(); 3918];
    for tag in tags_reader.records() {
        let record = tag.unwrap();

        let index_int = record
            .index(0)[1..]
            .parse::<usize>()
            .unwrap();
        tag_names[index_int] = record.index(5).to_string();
    }

    // MAP TAGS TO NOVELS using "tags_vn" //////////////////////////////////////////////////////////
    // TODO: Implement a check for tag votes?
    // TODO: Check for ero tags
    v_id = 1;
    novels_index = 0usize;
    let tags_vn_file = File::open("../database/db/tags_vn").unwrap();
    let mut tags_vn_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(tags_vn_file);

    let mut tags_vn_names: Vec<String> = Vec::new();
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

            swap(&mut novels[novels_index].tags, &mut tags_vn_names);
            v_id = curr_id;
        }

        let index_int = record
            .index(1)[1..]
            .parse::<usize>()
            .unwrap();
        let name = tag_names[index_int].clone();

        let mut in_list = false;
        for tag in &tags_vn_names {
            if tag == &name {
                in_list = true;
            }
        }
        if !in_list && name != "N/A"{
            tags_vn_names.push(name);
        }
    }
    // Same issue as with staff and seiyuu
    swap(&mut novels[novels_index + 2].tags, &mut tags_vn_names);

    ////////////////////////////////////////////////////////////////////////////////////////////////

    // Print out the contents of novels
    for vn in &novels {
        println!("Id: {} | Title: {}\nStaff: {:?}\nSeiyuu: {:?}\nTags: {:?}\n",
                 vn.v_id, vn.title, vn.staff, vn.seiyuu, vn.tags);
    }

    // After dealing with the horrors of vndb, return the lovely vector of Novels
    novels
}

