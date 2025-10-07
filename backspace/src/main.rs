use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin read");

    let mut lines = input.lines();
    let current_line = lines.next().expect("Line");

    let mut history: Vec<String> = Vec::new();

    for char in current_line.split("") {
        if char == "<" {
            if !history.is_empty() {
                history.pop();
            }
        } else {
            history.push(String::from(char));
        }
    }

    println!("{}", history.concat())
}
