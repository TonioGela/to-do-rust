use colored::*;
use std::env::args;
use std::process::exit;

fn main() {
    match first_arg().and_then(parse) {
        Ok(n) => triangle_reversed(n),
        Err(e) => {
            eprintln!("{}", e.red());
            exit(1);
        }
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
    ($a:expr, $b:expr) => {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeat_macro() {
        let result = "*".repeat(10);
        let mut test = String::new();
        repeat!(10, test = test + "*");
        assert_eq!(result, test)
    }

    #[test]
    fn parse_should_extract_numbers() {
        assert_eq!(
            parse(String::from("hello")),
            Err(String::from("Argument hello must be a positive number!"))
        );
        assert_eq!(
            parse(String::from("-10")),
            Err(String::from("Argument -10 must be a positive number!"))
        );
        assert_eq!(parse(String::from("100")), Ok(100));
    }
}
