use std::env;
use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GolVal {
    DEAD,
    RED,
    BLUE
}

impl fmt::Display for GolVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = match *self {
            GolVal::DEAD => ".",
            GolVal::RED => "#",
            GolVal::BLUE => "*"
        };
        write!(f, "{}", sign)
    }
}

impl GolVal {
    fn from_char(c: char) -> GolVal {
        match c {
            '.' => GolVal::DEAD,
            '#' => GolVal::RED,
            '*' => GolVal::BLUE,
            _ => panic!("Encountered invalid char"),
        }
    }
}

struct Gol {
    width: usize,
    height: usize,
    board: Vec<GolVal>
}

impl Gol {
    fn get(&self, i: usize, j: usize) -> GolVal {
        self.board[i * self.width + j]
    }

    fn set(&mut self, i: usize, j: usize, v: GolVal) {
        self.board[i * self.width + j] = v
    }

    fn get_neighbours(&self, i: usize, j: usize) -> Vec<GolVal> {
        let mut result = vec![];
        for ii in -1..2 {
            for jj in -1..2 {
                if !(jj == 0 && ii == 0) {
                    let mut idx0 = i as i32 + ii;
                    if idx0 == -1 {
                        idx0 = self.height as i32 - 1;
                    } else if idx0 == self.height as i32 {
                        idx0 = 0;
                    }

                    let mut idx1 = j as i32 + ii;
                    if idx1 == -1 {
                        idx1 = self.width as i32 - 1;
                    } else if idx1 == self.width as i32 {
                        idx1 = 0;
                    }

                    result.push(self.get(idx0 as usize, idx1 as usize));
                }
            }
        }
        return result;
    }

    fn new(width: usize, height: usize, board: Vec<GolVal>) -> Gol {
        Gol {width: width, height: height, board: board}
    }

    fn count_colors(vals: Vec<GolVal>) -> (u32, u32) {
        vals.iter()
            .fold((0,0), |acc, &v| match v {
                GolVal::BLUE => (acc.0 + 1, acc.1),
                GolVal::RED => (acc.0, acc.1 + 1),
                GolVal::DEAD => acc
            }
            )
    }
}

impl fmt::Display for Gol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.width {
            for j in 0..self.height {
                try!(write!(f, "{}", format!("{}", self.get(i, j))));
            }
            try!(write!(f, "\n"));
        }
        write!(f, "")
    }
}

fn gol(width: usize, height: usize, n_iter: usize, print_each_iter: bool, mut game: Gol) {
    if print_each_iter {
        println!("{}", game);
    }

    for _ in 0..n_iter {
        for i in 0..width {
            for j in 0..height {
                let cur = game.get(i, j);
                let (n_blue, n_red) = Gol::count_colors(game.get_neighbours(i, j));
                let n_live = n_blue + n_red;
                if cur == GolVal::DEAD && n_live == 3 {
                    if n_blue > n_red {
                        game.set(i, j, GolVal::BLUE);
                    } else {
                        game.set(i, j, GolVal::RED);
                    }
                } else if (cur == GolVal::RED && n_blue == 0
                           || cur == GolVal::BLUE && n_red == 0
                           || cur == GolVal::RED && n_red + 1 > n_blue
                           || cur == GolVal::BLUE && n_blue + 1 > n_red)
                    && n_live != 3 {
                    game.set(i, j, GolVal::DEAD);
                } else if cur == GolVal::BLUE && n_red > n_blue {
                    game.set(i, j, GolVal::RED);
                } else if cur == GolVal::RED && n_blue > n_red {
                    game.set(i, j, GolVal::BLUE);
                }
            }
        }

        if print_each_iter {
            println!("{}", game);
        }
    }
    println!("{}", game);
}

fn main() {
    let in_path = env::args().nth(1).unwrap();
    let bfr = BufReader::new(File::open(&in_path).unwrap());
    let mut lines = bfr.lines();
    let width = lines.next().unwrap().unwrap().split_whitespace().collect::<Vec<&str>>()[1].parse().unwrap();
    let height = lines.next().unwrap().unwrap().split_whitespace().collect::<Vec<&str>>()[1].parse().unwrap();
    let n_iter = lines.next().unwrap().unwrap().split_whitespace().collect::<Vec<&str>>()[1].parse().unwrap();
    let mut board = Vec::with_capacity(width * height);
    
    for line in lines {
        let l = line.unwrap();
        for c in l.chars() {
            board.push(GolVal::from_char(c));
        }
    }
    gol(width, height, n_iter, true, Gol::new(width, height, board));
}

