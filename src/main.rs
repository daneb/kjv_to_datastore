use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

// Genesis = key
// Genesis -> k,v =  int, Vec<String>
// Genesis -> k,v =  1, ["In the beginning", "And so"]  # 0, 1

fn main() {

    let filename = String::from("./kjv.txt");
    parse_bible(filename);
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn insert_title_chapters(bible: &mut HashMap<String, Vec<Chapter>>, title: String, chapters: Vec<Chapter>)
{
    let trimmed_title = title.trim();
    bible.entry(trimmed_title.to_string()).or_insert(chapters);
}

// pub fn insert_chapter_verses(line: String, chapter: &mut String, verses: &mut Vec<String>) {

// }

#[derive(Clone, Eq, PartialEq)]
pub struct Chapter {
    number: String,
    verses: Vec<String>,
}

pub fn parse_bible(input: String) -> HashMap<String, Vec<Chapter>> {
    let mut title: String = String::from("");
    let mut chapter: String = String::from("");
    let _verse_no: String = String::from("");
    let _verse: String = String::from("");
    let mut chapters: Vec<Chapter> = Vec::new();

    let mut bible: HashMap<String, Vec<Chapter>> = HashMap::new();
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
                insert_title_chapters(&mut bible, title, chapters.clone());

                chapters.clear();
            }

            title = line.replace("Title:", "");
            
        } else if matched.is_match(&line) {

            let v: Vec<&str> = line.splitn(2, " ").collect();
            let x: Vec<&str> = v[0].split(":").collect(); // 1:1 => [1, 1] 
            let current_chapter = x[0].to_string();
            let current_verse = v[1].to_string();

            if current_chapter != *chapter {
                // Create the new Chapter and verses and add it to the Chapter List
                let collected_chapter = Chapter {
                    number: chapter.clone(),
                    verses: verses.clone()
                };
                chapters.push(collected_chapter);

                chapter = current_chapter; // we need to update the new chapter 5 -> 6
            }

            verses.push(current_verse);
            

        } else {
            // Verses without numbers but part of a verse
             if let Some(last) = verses.last_mut() {
                 *last += &line;
             }
        }
    }

    insert_title_chapters(&mut bible, title, chapters);

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
        let file_count = 99820;
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
    fn test_number_of_chapters() {
        let filename = String::from("./kjv.txt");
        let genesis_chapters = 50;
        let exodus_chapters = 40;
        let leviticus_chapters = 27;
        let numbers_chapters = 36;
        let matthew_chapters = 28;
        let one_john_chapters = 5;
        let two_john_chapters = 1;
        let three_john_chapters = 1;
        let jude_chapters = 1;
        let revelation_chapters = 22;
        let result = parse_bible(filename);
        assert_eq!(genesis_chapters, result.get("Genesis").unwrap().len()); 
        assert_eq!(exodus_chapters, result.get("Exodus").unwrap().len()); 
        assert_eq!(leviticus_chapters, result.get("Leviticus").unwrap().len()); 
        assert_eq!(numbers_chapters, result.get("Numbers").unwrap().len()); 
        assert_eq!(matthew_chapters, result.get("Matthew").unwrap().len());
        assert_eq!(one_john_chapters, result.get("1 John").unwrap().len());
        assert_eq!(two_john_chapters, result.get("2 John").unwrap().len());
        assert_eq!(three_john_chapters, result.get("3 John").unwrap().len());
        assert_eq!(jude_chapters, result.get("Jude").unwrap().len());
        assert_eq!(revelation_chapters, result.get("Revelation").unwrap().len());
    }



    // #[test]
    // fn test_keys_are_not_ordered() {
    //     let filename = String::from("./tests/sample_titles.txt");

    //     let book1 = String::from("Genesis");
    //     let book2 = String::from("Exodus");
    //     let book3 = String::from("Leviticuss");

    //     let result = parse_bible(filename);
    //     assert_ne!(&book1, result.keys().next().unwrap());
    //     assert_ne!(&book2, result.keys().next().unwrap());
    //     assert_ne!(&book3, result.keys().next().unwrap());
    // }

    #[test]
    fn test_adding_chapters_to_title() {

        let mut chapters: Vec<Chapter> = Vec::new();

        let title = String::from("Genesis");

        let chapter1 = String::from("Chapter 1");
        let verses1: Vec<String> = vec![
            String::from("In the beginning God created the heaven and the earth."),
            String::from(" In the beginning was the Word, and the Word was with God, and the Word was God.")];

        let chapter2 = String::from("Chapter 2");
        let verses2: Vec<String> = vec![
            String::from("Thus the heavens and the earth were finished, and all the host of them."),
            String::from("And on the seventh day God ended his work which he had made; and he rested on the seventh day from all his work which he had made.")
        ];
        
        let mut bible: HashMap<String, Vec<Chapter>> = HashMap::new();

        let first_chapter = Chapter {
            number: chapter1.clone(),
            verses: verses1.clone()
        };

        let second_chapter = Chapter {
            number: chapter2.clone(),
            verses: verses2.clone()
        };

        chapters.push(first_chapter);
        chapters.push(second_chapter);

        insert_title_chapters(&mut bible, title, chapters);

        for x in bible.get("Genesis").unwrap() {
           if x.number == "Chapter 1" {
               assert_eq!(x.verses, verses1.clone());
           } else {
               assert_eq!(x.verses, verses2.clone());
           }
           
           
        }


    }

    #[test]
    fn test_if_all_verses_are_seperate_lines() {
        // This is not the case.
        // Perhaps an integration test
    }

}
