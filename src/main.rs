use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    let mut all: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input");
            return;
        }
    };

    while all >= 1 {
        all -= 1;
        solved();
    }
}

fn solved() {}
