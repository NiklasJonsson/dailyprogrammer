use std::env;
use std::collections::HashSet;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn dbg_print(v: &Vec<char>, index: usize) {
    for (i,c) in v.iter().enumerate() {
        if i == index {
            print!("{}", '[');
            print!("{}", c);
            print!("{}", ']');
        } else {
            print!("{}", c);
        }
    }

    if index >= v.len() {
        print!("[]");
    }
    println!("");
}

fn only_wc_left(rest: &[char]) -> bool {
    for c in rest.iter() {
        if *c != '*' {
            return false;
        }
    }
    return true;
}

fn has_overlap(v0: &Vec<char>, v1: &Vec<char>, i: usize, j: usize, visited: &mut HashSet<(usize, usize)>) -> bool {
   /* 
    dbg_print(v0, i);
    dbg_print(v1, j);
    println!("");
    */

    if !visited.insert((i, j)) {
        return false;
    }

    if i == v0.len() {
        return only_wc_left(&v1[j..]);
    } else if j == v1.len() {
        return only_wc_left(&v0[i..]);
    }

    let c0 = v0[i];
    let c1 = v1[j];

    if c0 != '*' && c1 != '*' {
        return c0 == c1 && has_overlap(v0, v1, i + 1, j + 1, visited);
    } else if c0 == '*' && c1 != '*' {
        return has_overlap(v0, v1, i + 1, j, visited) || has_overlap(v0, v1, i + 1, j + 1, visited);
    } else if c0 != '*' && c1 == '*' {
        return has_overlap(v0, v1, i, j + 1, visited) || has_overlap(v0, v1, i + 1, j + 1, visited);
    } else if c0 == '*' && c1 == '*' {
        return has_overlap(v0, v1, i + 1, j + 1, visited) || has_overlap(v0, v1, i, j + 1, visited) || has_overlap(v0, v1, i + 1, j, visited);
    } else {
        return false;
    }
}

fn expand_wc(s: &str) -> Vec<char> {
    let mut res: Vec<char> = Vec::new();
    s.chars().fold(&mut res,
                   |acc, c| {
                       if c == '*' {
                           acc.push(c);
                           acc.push(c); 
                           acc.push(c); 
                           acc.push(c); 
                       } else {
                           acc.push(c);
                       }
                       return acc;
                   }
                  );
    return res;
}

fn main() {
    let in_path = env::args().nth(1).unwrap();
    let mut line_it = BufReader::new(File::open(&in_path).unwrap()).lines();

    while let Some(Ok(line)) = line_it.next() {
        println!("{}", line);
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let v0 = expand_wc(words[0]);
        let v1 = expand_wc(words[1]);
        println!("{}", has_overlap(&v0, &v1, 0, 0, &mut HashSet::new()));
    }
}
