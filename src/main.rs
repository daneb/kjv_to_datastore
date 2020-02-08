use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone)]
pub struct Verse {
    number: String,
    text: String
}

// Data we have
// <Book> -> <Chapter>
//           <Chapter>-><Verse>
//                      <Number=> Text>

// Data structure
// {k;v}
// 

fn main() {

    let mut title: String = String::from("");
    let mut chapter: String = String::from("");
    let mut verse_no: String = String::from("");
    let mut verse: String = String::from("");

    // let mut bible_store = HashMap::new();
    let mut bible: HashMap<String, HashMap<String, Verse>> = HashMap::new();
    let mut chapterHash: HashMap<String, Verse> = HashMap::new();

    let filename = "kjv.txt";
    println!("In file {}", filename);

    let matched = Regex::new(r"^\d{1,}:\d{1,}").unwrap();
    let lines = read_lines("./kjv_abridged.txt");
    let lines_iter = lines.unwrap();
    
    'outer: for x in lines_iter {

        let line = x.unwrap();
        
        if line.is_empty() {
            continue 'outer;
        } else if line.contains("Title") {
            title = line.replace("Title:", "");
        } else if matched.is_match(&line) {

            if !verse.is_empty() {
                chapterHash.insert(chapter, Verse {
                    number: verse_no,
                    text: verse,
                });
            }

            let v: Vec<&str> = line.splitn(2, " ").collect();
            let x: Vec<&str> = v[0].split(":").collect(); // 1:1 => [1, 1] 
            chapter = x[0].to_string();
            verse_no = x[1].to_string();
            verse = v[1].to_string();

        } else {
            

        }
    }

}
