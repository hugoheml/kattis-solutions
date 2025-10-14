use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(& mut input)
        .expect("Lecture de stdin");

    let lines = input.lines();

    for line in lines {
        let mut sum = 0;
        for arg in line.split_whitespace() {
            let arg_result = arg.parse::<i32>();

            match arg_result {
                Ok(arg_int) => {
                    sum = sum + arg_int;
                }
                Err(e) => {
                    // nothing
                }
            }
        }
        println!("{}", sum);
    }
}
