use std::env;
use std::io::BufReader;
use std::collections::VecDeque;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let in_path = env::args().nth(1).unwrap();
    let mut line_it = BufReader::new(File::open(&in_path).unwrap()).lines();

    while let Some(Ok(line)) = line_it.next() {
        let mut char_vec: VecDeque<char> = line.chars().collect();

        while let Some(c) = char_vec.pop_front() {
            char_vec.pop_front();
            if c == 'a' {
                char_vec.push_back('b');
                char_vec.push_back('c');
            } else if c == 'b' {
                char_vec.push_back('a');
            } else if c == 'c' {
                char_vec.push_back('a');
                char_vec.push_back('a');
                char_vec.push_back('a');
            }
            println!("{}", char_vec.iter().cloned().collect::<String>());
            if char_vec.len() <= 1 {
                break;
            }
        }
    }
}
