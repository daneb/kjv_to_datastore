use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;



// Data we have
// <Book> -> <Chapter>
//           <Chapter>->[Verse]

// Genesis => 1 => "In the beginning"
//  Key       Value [Key] => [ "In the beginnning" ]                

// Data structure
// {k;v}
// 

fn main() {

    let filename = String::from("./kjv.txt");
    let result = parse_bible(filename);
    println!("{:?}", result.get("Genesis").unwrap().verses);

}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn add_new_entry(title: String, chapter: String, verses: Vec<String>, bible: &mut HashMap<String, Chapter>)
{
    let trimmed_title = title.trim();
    bible.entry(trimmed_title.to_string()).or_insert(Chapter {
            number: chapter.clone(),
            verses: verses.to_vec(),
    });
}

pub fn insert_title_chapter_verses(bible: &mut HashMap<String, Chapter>, title: String, chapter: String, verses: Vec<String>)
{
    let trimmed_title = title.trim();
    bible.entry(trimmed_title.to_string()).or_insert(Chapter {
            number: chapter.clone(),
            verses: verses.clone(),
    });
}

#[derive(Clone, Eq, PartialEq)]
pub struct Chapter {
    number: String,
    verses: Vec<String>,
}

pub fn parse_bible(input: String) -> std::collections::HashMap<String, Chapter> {
    let mut title: String = String::from("");
    let mut chapter: String = String::from("");
    let _verse_no: String = String::from("");
    let _verse: String = String::from("");

    let mut bible: HashMap<String, Chapter> = HashMap::new();
    let mut verses: Vec<String> = Vec::new();

    let matched = Regex::new(r"^\d{1,}:\d{1,}").unwrap(); // 1:1 | 2:22 | 15:23
    let lines = read_lines(input);
    let lines_iter = lines.unwrap();
    
    'outer: for x in lines_iter {

        let line = x.unwrap();

        // (1) Collect all the verse
        // (2) Build Chapter with verses
        // (3) Add Chapter to Title
        
        if line.is_empty() {
            continue 'outer;
        } else if line.contains("Title") {
            
            if !title.is_empty()
            {
                insert_title_chapter_verses(&mut bible, title, chapter, verses.clone());

                chapter = String::from("");
                verses.clear();
            }

            title = line.replace("Title:", "");
            
        } else if matched.is_match(&line) {

            // let v: Vec<&str> = line.splitn(2, " ").collect();
            // let x: Vec<&str> = v[0].split(":").collect(); // 1:1 => [1, 1] 
            // chapter = x[0].to_string();
            // verse = v[1].to_string();

            // verse.push_str(" ");
            // verses.push(verse);

        } else {
            // if let Some(last) = verses.last_mut() {
            //     *last += &line;
            // }
        }
    }

    insert_title_chapter_verses(&mut bible, title, chapter, verses.clone());

    return bible;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines_ok() {
        let result = read_lines("./kjv.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_reads_all_the_lines_ok() {
        let file_count = 99810;
        let result = read_lines("./kjv.txt");
        let line_count = result.unwrap().count();
        assert_eq!(file_count, line_count);
    }

    #[test]
    fn test_gets_all_titles()   {
        let filename = String::from("./tests/sample_titles.txt");
        let titles = 66;
        let result = parse_bible(filename);
        assert_eq!(titles, result.keys().count());
    }

    #[test]
    fn test_keys_are_not_ordered() {
        let filename = String::from("./tests/sample_titles.txt");

        let book1 = String::from("Genesis");
        let book2 = String::from("Exodus");
        let book3 = String::from("Leviticuss");

        let result = parse_bible(filename);
        assert_ne!(&book1, result.keys().next().unwrap());
        assert_ne!(&book2, result.keys().next().unwrap());
        assert_ne!(&book3, result.keys().next().unwrap());
    }

    #[test]
    fn test_adding_title_with_verses() {
        let title = String::from("Genesis");
        let chapter = String::from("1");
        let verses: Vec<String> = vec![
            String::from("In the beginning God created the heaven and the earth."),
            String::from(" In the beginning was the Word, and the Word was with God, and the Word was God.")];
        let mut bible: HashMap<String, Chapter> = HashMap::new();

        let expected_chapter = Chapter {
            number: chapter.clone(),
            verses: verses.clone()
        };

        add_new_entry(title, chapter, verses, &mut bible);

        assert_eq!(bible.contains_key("Genesis"), true);
        let result = bible.get_key_value("Genesis");
        assert!(expected_chapter == *result.unwrap().1);

    }

}
