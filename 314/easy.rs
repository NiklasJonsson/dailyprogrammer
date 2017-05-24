use std::io; 

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed stdin");
    let mut numbers: Vec<&str> = line.split_whitespace().collect();
    numbers.sort_by(|a, b| b.cmp(a));
    let largest = numbers.join("");
    numbers.sort();
    let smallest = numbers.join("");
    println!("{} {}", smallest, largest);
}
