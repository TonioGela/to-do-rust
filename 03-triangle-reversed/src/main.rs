use colored::*;
use std::env::args;

fn main() {
    match first_arg().and_then(parse) {
        Ok(n) => triangle_reversed(n),
        Err(e) => println!("{}", e.red()),
    }
}

fn first_arg() -> Result<String, String> {
    args()
        .nth(1)
        .ok_or("You must provide at least 1 numeric argument".to_owned())
}

fn parse(s: String) -> Result<usize, String> {
    s.trim()
        .parse::<usize>()
        .map_err(|_| format!("Argument {} must be a positive number!", s))
}

macro_rules! repeat {
    ($a:expr, $b:block) => {
        for _ in 0..$a {
            $b
        }
    };
}

fn triangle_reversed(n: usize) {
    let mut string = "*".repeat(n);
    repeat!(n, {
        println!("{}", string.bright_green());
        string.pop();
    });
}
