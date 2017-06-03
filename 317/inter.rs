use std::env;
use std::io::BufReader;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct CharStream {
    index: usize,
    vec: Vec<char>
}

impl CharStream {
    fn new(s: &String) -> CharStream {
        CharStream {index: 0, vec: s.chars().collect::<Vec<char>>()}
    }

    fn is_digit(&self) -> bool {
        self.index < self.vec.len() && self.vec[self.index].is_digit(10)
    }

    fn read(&mut self) -> Option<char> {
        if self.index < self.vec.len() {
            self.index += 1;
            return Some(self.vec[self.index - 1]);
        }
        return None;
    }

    fn read_lowercase(&mut self) -> Option<char> {
        if self.index < self.vec.len() && self.vec[self.index].is_lowercase() {
            self.index += 1;
            return Some(self.vec[self.index - 1]);
        }
        return None;
    }

    fn read_digit(&mut self) -> Option<u32> {
        if self.index < self.vec.len() {
            if let Some(n) = self.vec[self.index].to_digit(10) {
                self.index += 1;
                return Some(n);
            }
        }
        return None;
    }

    fn incr(&mut self) {
        self.index += 1;
    }
}

fn add_cnt(map: &mut BTreeMap<String, u32>, key: String, incr_val: u32)  {
    *map.entry(key).or_insert(0) += incr_val;
}

fn eat_number(stream: &mut CharStream) -> u32 {
    let mut n = 0;
    while let Some(x) = stream.read_digit() {
            n = n * 10 + x;
    }
    return n;
}

fn count_elem(mut stream: &mut CharStream) -> BTreeMap<String, u32> {
    let mut counter: BTreeMap<String, u32> = BTreeMap::new();

    while let Some(c) = stream.read() {
        if c.is_uppercase() {
            let mut s = String::new();
            s.push(c);

            if let Some(c2) = stream.read_lowercase() {
                s.push(c2);
            }

            if stream.is_digit() {
                let val = eat_number(&mut stream);
                add_cnt(&mut counter, s, val);
            } else {
                add_cnt(&mut counter, s, 1);
            }
        } else if c == '(' {
            let nested = count_elem(&mut stream);
            let mut mult = 1;

            if stream.is_digit() {
                mult = eat_number(&mut stream);
            }

            for (s, val) in nested {
                add_cnt(&mut counter, s, mult * val);
            }
        } else if c == ')' {
            break;
        }
    }
    return counter;
}


fn main() {
    let in_path = env::args().nth(1).unwrap();
    let mut line_it = BufReader::new(File::open(&in_path).unwrap()).lines();

    while let Some(Ok(line)) = line_it.next() {
        println!("{}", line);
        let mut cs = CharStream::new(&line);
        let cnt = count_elem(&mut cs);
        for k in cnt.keys() {
            println!("{} : {}", k, cnt[k]);
        }
    }
}
