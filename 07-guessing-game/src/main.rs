use colored::*;
use rand::{thread_rng, Rng};
use std::{
    cmp::Ordering,
    io::{stdin, stdout, Write},
};

fn main() {
    let secret: i32 = thread_rng().gen_range(1..=100);
    loop {
        match prompt_for_a_number() {
            Err(err) => {
                println!("{}", err.red().bold());
                continue;
            }
            Ok(guess) => match guess.cmp(&secret) {
                Ordering::Less => {
                    println!("{}", String::from("Too low!").red().bold());
                    continue;
                }
                Ordering::Greater => {
                    println!("{}", String::from("Too high!").red().bold());
                    continue;
                }
                Ordering::Equal => {
                    println!("{}", String::from("YOU WON").green().bold());
                    break;
                }
            },
        }
    }
}

fn prompt_for_a_number() -> Result<i32, &'static str> {
    let mut input: String = String::new();
    print!("Please input a number between 1 and 100: ");
    stdout()
        .flush()
        .map_err(|_| "Unable to write to stdout!")
        .and_then(|_| {
            stdin()
                .read_line(&mut input)
                .map_err(|_| "Unable to read from stdin!")?;
            let number = input
                .trim()
                .parse()
                .map_err(|_| "Input should be an integer number!")?;
            if number >= 1 && number <= 100 {
                Ok(number)
            } else {
                Err("Input should be in 1-100 range!")
            }
        })
}
