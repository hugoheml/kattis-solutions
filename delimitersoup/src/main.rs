use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin read");

    let mut lines = input.lines();
    lines.next();

    let mut open_characters: Vec<char> = Vec::new();

    let line = lines.next().expect("Line");

    let mut success = true;
    for (index, character) in line.char_indices() {
        match character {
            '(' => open_characters.push('('),
            '[' => open_characters.push('['),
            '{' => open_characters.push('{'),
            ']' => match open_characters.last() {
                Some(charact) => {
                    if *charact != '[' {
                        println!("{} {}", character, index);
                        success = false;
                        break;
                    } else {
                        open_characters.pop();
                    }
                }
                None => {
                    println!("{} {}", character, index);
                    success = false;
                    break;
                }
            },
            ')' => match open_characters.last() {
                Some(charact) => {
                    if *charact != '(' {
                        println!("{} {}", character, index);
                        success = false;
                        break;
                    } else {
                        open_characters.pop();
                    }
                }
                None => {
                    println!("{} {}", character, index);
                    success = false;
                    break;
                }
            },
            '}' => match open_characters.last() {
                Some(charact) => {
                    if *charact != '{' {
                        println!("{} {}", character, index);
                        success = false;
                        break;
                    } else {
                        open_characters.pop();
                    }
                }
                None => {
                    println!("{} {}", character, index);
                    success = false;
                    break;
                }
            },

            _ => {}
        }
    }

    if success {
        println!("ok so far");
    }
}
