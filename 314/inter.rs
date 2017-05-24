use std::io; 

fn rotate(a: &String, pos : usize) -> String {
    return format!("{}{}", a.chars().skip(pos).collect::<String>(), a.chars().take(pos).collect::<String>());
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed stdin");
    let word = String::from(line.trim());
    let mut pos = 0;
    let mut min = word.clone(); 
    for i in 0..word.len() {
        let t = rotate(&word, i);
        if t < min {
            min = t;
            pos = i;
        }
    }

    let start : String = word.chars().skip(pos).collect();
    let end : String = word.chars().take(pos).collect();
    println!("{} {}{}",pos, start, end);
}
