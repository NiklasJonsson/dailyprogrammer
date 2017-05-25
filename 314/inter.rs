use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn rotate(a: &String, pos : usize) -> String {
    return format!("{}{}", a.chars().skip(pos).collect::<String>(), a.chars().take(pos).collect::<String>());
}

fn main() {
    let in_path = env::args().nth(1).unwrap();
    let file = File::open(&in_path).unwrap();
    let bfr = BufReader::new(file);

    for l in bfr.lines() {
        let line = l.unwrap();
        let word = String::from(line.trim());
        let mut pos = 0;
        let mut min = word.clone(); 
        for i in 0..word.len() {
            let t = rotate(&word, i);
            if t < min {
                min = t;
                pos = i;
            }
        }

        let start : String = word.chars().skip(pos).collect();
        let end : String = word.chars().take(pos).collect();
        println!("{} {}{}",pos, start, end);
    }

}
