use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


fn main() {

    let mut title_hash : u64 = 0;
    // let mut bible_store = HashMap::new();
    let mut verses: Vec<String> = Vec::new();

    let filename = "kjv.txt";
    println!("In file {}", filename);

    let lines = read_lines("./kjv.txt");
    let lines_iter = lines.unwrap();

    lines_iter.for_each(|x| {
        let line = x.unwrap(); 
        if line.contains("Title:") {
            title_hash = calculate_hash(&line);
        } else {
            verses.push(line);
        }
    });

    println!("{:?}", verses);

}
