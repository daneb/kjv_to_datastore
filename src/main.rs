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
pub struct Chapter {
    number: String,
    verses: Vec<String>,
}

// Data we have
// <Book> -> <Chapter>
//           <Chapter>->[Verse]

// Genesis => 1 => "In the beginning"
//  Key       Value [Key] => [ "In the beginnning" ]                

// Data structure
// {k;v}
// 

fn main() {

    let mut title: String = String::from("");
    let mut chapter: String = String::from("");
    let mut verse_no: String = String::from("");
    let mut verse: String = String::from("");

    let mut bible: HashMap<String, Chapter> = HashMap::new();
    let mut verses: Vec<String> = Vec::new();

    let filename = "kjv.txt";
    println!("In file {}", filename);

    let matched = Regex::new(r"^\d{1,}:\d{1,}").unwrap(); // 1:1 | 2:22 | 15:23
    let lines = read_lines("./kjv_abridged.txt");
    let lines_iter = lines.unwrap();
    
    'outer: for x in lines_iter {

        let line = x.unwrap();

        // (1) Collect all the verse
        // (2) Build Chapter with verses
        // (3) Add Chapter to Title
        
        if line.is_empty() {
            continue 'outer;
        } else if line.contains("Title") {
            title = line.replace("Title:", "");
        } else if matched.is_match(&line) {

            let v: Vec<&str> = line.splitn(2, " ").collect();
            let x: Vec<&str> = v[0].split(":").collect(); // 1:1 => [1, 1] 
            chapter = x[0].to_string();
            verse = v[1].to_string();

            verse.push_str(" ");
            verses.push(verse);

        } else {
            if let Some(last) = verses.last_mut() {
                *last += &line;
            }
        }
    }
    
    let trimmed_title = title.trim();
    bible.entry(trimmed_title.to_string()).or_insert(Chapter {
            number: chapter,
            verses: verses,
    });

    println!("{:?}", bible.get("Genesis").unwrap().verses);

}
