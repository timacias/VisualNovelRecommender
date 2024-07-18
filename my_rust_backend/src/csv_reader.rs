use std::fmt::Debug;
use std::fs::File;
use std::ops::Index;
use csv::StringRecord;
use serde::de::Unexpected::Str;
use crate::test::Novel;

#[derive(serde::Deserialize)]
struct Row {
    id: u16,
    lang: String,
    official: bool,
    title: String,
    latin: String
}

pub fn reading_csv(/*novels: vec<Novel>*/) {
    // TODO: Make the vn object where we just initialize all the other parameters as nothing.
    // TODO: Make vector of vn objects so that we can implement vn searching.
    // GETTING VISUAL NOVEL TITLE "vn_titles"
    let vn_file = File::open("../database/db/vn_titles").unwrap();
    let mut vn_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(vn_file);

    let mut v_id : u16 = 1;
    let mut languages = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    let mut latin_titles: Vec<String> = Vec::new();
    let mut title_to_use = "";

    for vn in vn_reader.records() {
        if let record = vn.unwrap() {
            let curr_id = record
                .index(0)[1..]
                .parse()
                .unwrap();

            if v_id != curr_id {
                for i in 0..languages.len() {
                    if languages[i] == "en" {
                        title_to_use = &*titles[i];
                        break;
                    }
                    else if languages[i] == "ja" {
                        if &*latin_titles[i] == "\\N" {
                            title_to_use = &*titles[i];
                        }
                        else{
                            title_to_use = &*latin_titles[i];
                        }
                        break;
                    }

                }
                if title_to_use.is_empty() {
                    if &*latin_titles[0] == "\\N" {
                        title_to_use = &*titles[0];
                    }
                    else{
                        title_to_use = &*latin_titles[0];
                    }
                }
                // println!("ID: {}, Title: {}", v_id, title_to_use);

                v_id = curr_id;
                languages.clear();
                titles.clear();
                latin_titles.clear();
                title_to_use = "";
            }

            languages.push(record.index(1).to_string());
            titles.push(record.index(3).to_string());
            latin_titles.push(record.index(4).to_string());

        }
    }

    // GETTING GALL ALIAS TO SEARCH FOR STAFF AND SEIYUU "staff_alias"
    let alias_file = File::open("../database/db/staff_alias").unwrap();
    let mut alias_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(alias_file);

    let mut names: Vec<String> = Vec::new();
    for _i in 0..46643{
        names.push("N/A".to_string());
    }
    for alias in alias_reader.records(){
        if let record = alias.unwrap(){
            let index_str = record.index(1).to_string();
            let index_int = index_str.parse::<usize>().unwrap();
            let mut name;
            if record.index(3).to_string() != "\\N" {
                name = record.index(3).to_string();
            }
            else{
                name = record.index(2).to_string();
            }
            names[index_int] = name;
        }
    }

    for i in 0..names.len(){
        println!("{}: {}", i+1, names[i]);
    }

    // GETTING STAFF (EXCLUDING SEIYUU) NAMES FOR EACH VN "vn_staff"
    v_id = 1;
    let staff_file = File::open("../database/db/vn_staff").unwrap();
    let mut staff_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .from_reader(staff_file);

    let mut staff_names: Vec<String> = Vec::new();
    for name in staff_reader.records(){
        if let record = name.unwrap(){
            let curr_id = record
                .index(0)[1..]
                .parse()
                .unwrap();

            if v_id != curr_id {
                println!("ID: {}, Staff:", v_id);
                for name in &staff_names {
                    println!("{}", name);
                }
                println!();

                v_id = curr_id;
                staff_names.clear();
            }

            let index_str = record.index(1).to_string();
            let index_int = index_str.parse::<usize>().unwrap() - 1;
            let name = names[index_int].clone();

            let mut in_list = false;
            for title in &staff_names{
                if title == &name{
                    in_list = true;
                }
            }
            if !in_list && name != "N/A"{
                staff_names.push(name);
            }
        }
    }
    
    // GETTING SEIYUU NAMES "vn_seiyuu"
    v_id = 1;
    let seiyuu_file = File::open("../database/db/vn_seiyuu").unwrap();
    let mut seiyuu_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .from_reader(seiyuu_file);

    let mut seiyuu_names: Vec<String>  = Vec::new();
    for seiyuu_name in seiyuu_reader.records() {
        if let record = seiyuu_name.unwrap() {
            let curr_id = record
                .index(0)[1..]
                .parse()
                .unwrap();

            if v_id != curr_id {
                if seiyuu_names.is_empty(){
                    seiyuu_names.push("N/A".to_string());
                }
                println!("ID: {}, Voice Actors:", v_id);
                for name in &seiyuu_names {
                    println!("{}", name);
                }
                println!();

                v_id = curr_id;
                seiyuu_names.clear();
            }

            let index_str = record.index(1).to_string();
            let index_int = index_str.parse::<usize>().unwrap() - 1;
            let name = names[index_int].clone();

            let mut in_list = false;
            for title in &seiyuu_names {
                if title == &name {
                    in_list = true;
                }
            }
            if !in_list && name != "N/A" {
                seiyuu_names.push(name);
            }
        }
    }
}

