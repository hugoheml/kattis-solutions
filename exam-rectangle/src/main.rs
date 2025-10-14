use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(& mut input)
        .expect("Lecture de stdin");

    let mut lines = input.lines();
    let line = lines.next().expect("Ligne");

    let mut args = line.split_whitespace();
    
    let height = args.next().expect("Height").parse::<i32>().expect("number");
    let width = args.next().expect("Width").parse::<i32>().expect("number");

    // Method 1
    let mut str = String::new();
    for _ in 0..height {
        for _ in 0..width {
            str = str + "x";
        }
        str = str + "\n";
    }
    println!("{}", str);
    
    // Method 2
    // for _ in 0..height {
    //     println!("{}", "X".repeat(width as usize));
    // }
}
