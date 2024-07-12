use std::fmt::Debug;
use std::fs::File;
use std::ops::Index;
use csv::StringRecord;
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



    /*if let Ok(result) = reader.records().next().unwrap() {
        assert_eq!(result, vec!["v1", "en", "t", "Let's Meow Meow!", "\\N"]);
        println!("{:?}", result);
    } else {
        println!("expected at least one record but got none")
    }*/

    // Make variables for currID and vectors of language, title, and romanji. Have currID set to 'v1' by default.
    // let mut curId =

    // Leave for example
    // language = {en, jp, de}
    // title = {"title in en", "title in jp", "title in de"};
    // romanji = {"\N", "Romanji", "\N"};

    let vn_titles_header = StringRecord::from(vec![
        "id", "lang", "official", "title", "latin"
    ]);

    //let novels = Vec::new();

    let mut v_id : u16 = 1;
    let mut languages = Vec::new();
    let mut titles = Vec::new();
    let mut latin_titles = Vec::new();
    let mut title_to_use = "";

    for vn in reader.records() {
        if let record = vn.unwrap() {
            let curr_id = record
                .index(0)[1..]
                .parse()
                .unwrap();

            if v_id == curr_id {
                languages.push(record.index(1).to_string());
                titles.push(record.index(3).to_string());
                latin_titles.push(record.index(4).to_string());
                continue;
            } else {
                for i in 0..languages.len() {
                    if !title_to_use.is_empty() {
                        break;
                    }
                    if languages[i] == "en" {
                        title_to_use = &*titles[i];
                        break;
                    }
                    if languages[i] == "ja" {
                        title_to_use = &*latin_titles[i];
                        break;
                    }
                }
                if title_to_use.is_empty() {
                    // This line panics at v_id 8, I need to find out why, kill me
                    title_to_use = titles.first().unwrap();
                }
                println!("{}", title_to_use);
                v_id = curr_id;
                languages.clear();
                titles.clear();
                latin_titles.clear();
                title_to_use = "";
            }
            // let row : Row = record.deserialize(Some(&vn_titles_header)).unwrap();
            // let id = record;
            // println!("{:?}", record); // Should print out a vector
        }

        // Check to see if the vID of the row matches currID.
        // If vID == currID
            // Append the language, title, and romanji of that row into their respective vectors.
        // Else If vID != currID
            // Making the object. DON'T FORGET TO TURN ID INTO I32
                // Check to see if 'en' is in the language vector
                    // If it is, find the index where it's at, n, and search for it via title[n] for the title of the VN.
                    // Ex: in the language vector, "en" is at index 0, so we find the title via title[0]
                // if else, check for 'jp'
                    // Same as with 'en' but we will use romanji[n] instead for the title.
                    // Ex: in the language vector, "jp" is at index 1, so we find the title vector via romanji[1]
                // Catch all
                    // Just use title[0] (first title) as the VN title.
            // Empty out all the variables, and change the currID to be the new vID
            // Append the language, title, and romanji of that row into their respective vectors.
    }
}

