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
    let file = File::open("../database/db/vn_titles").unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(file);

    let vn_titles_header = StringRecord::from(vec![
        "id", "lang", "official", "title", "latin"
    ]);

    let mut v_id : u16 = 1;
    let mut languages = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    let mut latin_titles: Vec<String> = Vec::new();
    let mut title_to_use = "";

    for vn in reader.records() {
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
                        if(&*latin_titles[i] == "\\N"){
                            title_to_use = &*titles[i];
                        }
                        else{
                            title_to_use = &*latin_titles[i];
                        }
                        break;
                    }

                }
                if title_to_use.is_empty() {
                    if(&*latin_titles[0] == "\\N"){
                        title_to_use = &*titles[0];
                    }
                    else{
                        title_to_use = &*latin_titles[0];
                    }
                }
                println!("ID: {}, Title: {}", v_id, title_to_use);

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
}

