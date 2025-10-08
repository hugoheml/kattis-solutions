use std::collections::VecDeque;

use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin read");

    let mut lines = input.lines();

    let line = lines.next().expect("Line");
    let amount_tests = line.parse::<i32>().expect("Integer");

    for _ in 0..amount_tests {
        let action = lines.next().expect("Action(s)");

        lines.next();

        let items_input = lines
            .next()
            .expect("Number list")
            .trim_matches(|c| c == '[' || c == ']');

        let mut items_array: VecDeque<_> = if items_input.is_empty() {
            VecDeque::new()
        } else {
            items_input.split(',').collect()
        };

        let mut reverse = false;
        let mut skip_print = false;

        for action_name in action.chars() {
            match action_name {
                'R' => reverse = !reverse,
                'D' => {
                    if items_array.is_empty() {
                        println!("error");
                        skip_print = true;
                        break;
                    } else if reverse {
                        items_array.pop_back();
                    } else {
                        items_array.pop_front();
                    }
                }
                _ => {
                    println!("No action");
                    break;
                }
            }
        }

        if !skip_print {
            let result: Vec<_> = if reverse {
                items_array.into_iter().rev().collect()
            } else {
                items_array.into_iter().collect()
            };
            println!("[{}]", result.join(","));
        }
    }
}
