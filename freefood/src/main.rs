use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin read");

    let mut lines = input.lines();

    let line = lines.next().expect("Line");
    let amount_free_food = line.parse::<i32>().expect("Integer");

    let mut days_with_free_food = vec![];
    for _ in 0..amount_free_food {
        let line = lines.next().expect("Line");
        let free_food = line.parse::<String>().expect("String");
        let args = free_food.split(' ').collect::<Vec<&str>>();

        let (x, y) = (
            args[0].parse::<i32>().expect("Integer"),
            args[1].parse::<i32>().expect("Integer"),
        );

        for day in x..=y {
            if !days_with_free_food.contains(&day) {
                days_with_free_food.push(day);
            }
        }
    }

    println!("{}", days_with_free_food.len());
}
