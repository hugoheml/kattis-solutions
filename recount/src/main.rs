use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin read");

    let lines = input.lines();
    let mut final_result: HashMap<String, i32> = HashMap::new();

    for line in lines {
        if line == "***" {
            break;
        }

        let vote_result = line.parse::<String>().expect("String");

        if final_result.contains_key(&vote_result) {
            let count = final_result.get(&vote_result).unwrap();
            final_result.insert(vote_result.to_string(), count + 1);
        } else {
            final_result.insert(vote_result.to_string(), 1);
        }
    }

    let mut winner = String::new();
    let mut max_votes = 0;
    let mut amount_of_equality = 0;
    for (name, votes) in final_result {
        if votes > max_votes {
            max_votes = votes;
            winner = name;
            amount_of_equality = 1;
        } else if votes == max_votes {
            amount_of_equality += 1;
        }
    }

    if amount_of_equality > 1 {
        println!("Runoff!");
    } else {
        println!("{}", winner);
    }
}
