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

fn triangle_reversed(n: usize) {
    let mut string = "*".repeat(n);
    for _ in 0..n {
        println!("{}", string.bright_green());
        string.pop();
    }
}
