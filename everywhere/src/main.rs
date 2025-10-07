use std::{
    collections::HashSet,
    io::{self, prelude::*},
};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");

    let mut lines = input.lines();

    let line = lines.next().expect("Ligne");
    let amount_tests = line.parse::<i32>().expect("Entier");

    for _ in 0..amount_tests {
        let line = lines.next().expect("Ligne");
        let cities_amount = line.parse::<i32>().expect("Entier");

        let mut cities = HashSet::new();

        for _city in 0..cities_amount {
            let line = lines.next().expect("Ligne");
            let city = line.parse::<String>().expect("Entier");

            cities.insert(city);
        }

        println!("{}", cities.len());
    }
}
