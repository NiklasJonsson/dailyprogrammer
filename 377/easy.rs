use std::io::BufReader;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;
use std::str::FromStr;

fn fit1(crate_x: usize, crate_y: usize, box_x: usize, box_y: usize) -> usize {
    let x = crate_x / box_x;
    let y = crate_y / box_y;

    y * x
}

fn fit2(crate_x: usize, crate_y: usize, box_x: usize, box_y: usize) -> usize {
    max(fit1(crate_x, crate_y, box_x, box_y), fit1(crate_x, crate_y, box_y, box_x))
}

fn fit3(crate_x: usize, crate_y: usize, crate_z: usize,
        box_x: usize, box_y: usize, box_z: usize) -> usize {
    let arr = [(crate_x / box_x) * (crate_y / box_y) * (crate_z / box_z),
        (crate_x / box_x) * (crate_y / box_z) * (crate_z / box_y),

        (crate_x / box_y) * (crate_y / box_z) * (crate_z / box_x),
        (crate_x / box_y) * (crate_y / box_x) * (crate_z / box_z),

        (crate_x / box_z) * (crate_y / box_x) * (crate_z / box_y),
        (crate_x / box_z) * (crate_y / box_y) * (crate_z / box_x)];

    *arr.into_iter().max().expect("No elements!")
}

fn main() {
    let in_path = env::args().nth(1).expect("Can't read file arg");
    let file = File::open(&in_path).expect("Can't open file");
    let bfr = BufReader::new(file);

    for line in bfr.lines() {
        let line = line.expect("No line!");

        let (fit_func, rest) = line.split_at(4);
        let rest = rest
            .trim_matches(|c| c == ')' || c == '(')
            .replace("[", "")
            .replace("]", "");
        let splitted = rest
            .split(',')
            .map(|s| usize::from_str(s.trim()).expect("Could not read usize"))
            .collect::<Vec<_>>();
        assert_eq!(splitted.len() % 2, 0);

        if fit_func == "fit1" {
            println!("{} => {}", line, fit1(splitted[0],
                                      splitted[1],
                                      splitted[2],
                                      splitted[3]));
        } else if fit_func == "fit2" {
            println!("{} => {}", line, fit2(splitted[0],
                                      splitted[1],
                                      splitted[2],
                                      splitted[3]));
        } else if fit_func == "fit3" {

            println!("{} => {}", line, fit3(splitted[0],
                                      splitted[1],
                                      splitted[2],
                                      splitted[3],
                                      splitted[4],
                                      splitted[5]));
        } else {
            assert_eq!(fit_func, "fitn");
        }
    }
}
