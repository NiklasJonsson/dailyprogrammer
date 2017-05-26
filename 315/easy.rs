use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn xor_mul(x: u32, y: u32) -> u32 {
    let mut pos: u32 = 0;
    let mut sum: u32 = 0;

    while (y >> pos) > 0 {
        if y & (1 << pos) > 0 {
            sum ^= (x & std::u32::MAX) << pos;
        };
        pos += 1;
    }
    return sum;
}

fn main() {
    let in_path = env::args().nth(1).unwrap();
    let bfr = BufReader::new(File::open(&in_path).unwrap());

    for line in bfr.lines() {
        let l = line.unwrap();
        let numbers: Vec<u32>= l.split_whitespace().map(|a| a.parse().unwrap()).collect();
        println!("{}@{}={}", numbers[0], numbers[1], xor_mul(numbers[0], numbers[1]));
    }
}

