use std::io::BufReader;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn cmp_str_lrg(a : &str, b : &str) -> std::cmp::Ordering{
    let mut ab = String::from(a);
    ab.push_str(b);
    let mut ba = String::from(b);
    ba.push_str(a);
    let x:i32 = ab.parse().unwrap();
    let y:i32 = ba.parse().unwrap();
    return y.cmp(&x);
}

fn cmp_str_sml(a : &str, b : &str) -> std::cmp::Ordering{
    let mut ab = String::from(a);
    ab.push_str(b);
    let mut ba = String::from(b);
    ba.push_str(a);
    let x:i32 = ab.parse().unwrap();
    let y:i32 = ba.parse().unwrap();
    return x.cmp(&y);
}

fn main() {
    let in_path = env::args().nth(1).unwrap();

    let file = File::open(&in_path).unwrap();
    let bfr = BufReader::new(file);

    for line in bfr.lines() {
        let l = line.unwrap();
        let mut numbers: Vec<&str> = l.split_whitespace().collect();
        numbers.sort_by(|a, b| cmp_str_lrg(a,b));
        let largest = numbers.join("");
        numbers.sort_by(|a, b| cmp_str_sml(a,b));
        let smallest = numbers.join("");
        println!("{} {}", smallest, largest);
    }
}
