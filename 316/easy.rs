use std::io::BufReader;
use std::cmp::Ordering;
use std::collections::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive (PartialEq, Eq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x == other.x {
            return self.y.cmp(&other.y);
        }
        return self.x.cmp(&other.x);
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

static MOVES: [[i32; 2]; 8]  = [[-1,-2], [ 1,-2], [-1, 2], [ 1, 2], [-2,-1], [ 2,-1], [-2, 1], [ 2, 1]];
const N_MOVS: usize = 8;

fn move_to(to : &Point) -> i32 {
    let start = Point{x: 0, y: 0};
    let mut visited = BTreeSet::new();
    let mut dist = BTreeMap::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back(start);
    dist.insert(start, 0);

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();

        if cur == *to {
            return *dist.get(&cur).unwrap();
        }

        for i in 0..N_MOVS {
            let next = Point{x: cur.x + MOVES[i][0], y: cur.y + MOVES[i][1]};

            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back(next);
                let d = dist.get(&cur).unwrap() + 1;
                dist.insert(next, d);
            }
        }
    }
    return -1;
}

fn main() {
    let in_path = env::args().nth(1).unwrap();

    let file = File::open(&in_path).unwrap();
    let bfr = BufReader::new(file);

    for line in bfr.lines() {
        let l = line.unwrap();
        let strs = l.split_whitespace();
        let numbers : Vec<i32> = strs.map(|a| a.parse().unwrap()).collect();
        assert_eq!(numbers.len(), 2);
        let end = Point{x: numbers[0], y: numbers[1]};
        let n_movs = move_to(&end);
        assert!(n_movs != -1);
        println!("{}", n_movs);
    }
}
