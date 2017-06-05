use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn get_index_of_after(word: &VecDeque<char>, new_char: char, idx: usize) -> Option<usize> {
    for (i,c) in word.iter().enumerate() {
        if *c == new_char && i >= idx {
            return Some(i);
        }
    }
    return None;
}

fn main() {
    let in_path = env::args().nth(1).unwrap();
    let mut line_it = BufReader::new(File::open(&in_path).unwrap()).lines();
    let mut queue = VecDeque::new();

    while let Some(Ok(word)) = line_it.next() {
        word.trim();
        let mut index = 0;
        for c in word.chars() {
         //   println!("{:?}, {}", queue, c);
            if let Some(i) = get_index_of_after(&queue, c, index) {
                index = i + 1;
            } else {
                queue.insert(index, c);
                index += 1;
            }
        }
        index = 0;

    }

    for c in queue.iter() {
        print!("{}", c);
    }
    println!(" {}", queue.len());

}
