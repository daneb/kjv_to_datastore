use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let filename = "kjv.txt";
    println!("In file {}", filename);

    let lines = read_lines("./kjv.txt");

    let lines_iter = lines.unwrap();
    lines_iter.for_each(|x| {
        let line = x.unwrap(); 
        if line.contains("Title:") {
            println!("{}", line);    
        }
    });

}
