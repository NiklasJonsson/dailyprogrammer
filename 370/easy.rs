use std::io::BufReader;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

fn main() {
    let in_path = env::args().nth(1).unwrap();

    let file = File::open(&in_path).unwrap();
    let bfr = BufReader::new(file);
    let zeros = "00000000000";

    for line in bfr.lines() {
        let l = line.unwrap();

        let it = zeros.chars()
                      .take(max(11-l.len(), 0))
                      .chain(l.chars())
                      .take(11);

        let (even, odd) = it.enumerate()
                            .fold((0,0),
                                  |acc, (i, c)| {
                                      let n = c.to_digit(10).unwrap();
                                      return match i % 2 == 0 {
                                          true => (acc.0 + n, acc.1),
                                          false => (acc.0,  acc.1 + n),
                                      };
                                  });


        let M = (3 * even + odd) % 10;
        let final_code = match M {
            0 => 0,
            _ => 10 - M,
        };
        println!("{}", final_code);
    }
}
