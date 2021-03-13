use std::{env::args, process::exit};

use itertools::*;

//* Create a programme which generates Fibonacci series til a number
//* 'n', where 'n' is entered by the user. Eg if the user enters 10
//* then the output would be: `1 1 2 3 5 8`

fn main() {
    let args = args().skip(1).collect_vec();

    match input_number(&args) {
        Ok(x) => println!("{}", format(fibonacci_until(x))),
        Err(str) => {
            eprintln!("{}", str);
            exit(1);
        }
    };
}

fn input_number(args: &[String]) -> Result<u32, &'static str> {
    let args = args.iter().map(|x| &x[..]).collect_vec();

    match &args[..] {
        [x] => x
            .parse()
            .map_err(|_| "You should pass just one numeric argument")
            .and_then(|x| {
                if x < 1 {
                    Err("The number should be greater than 1")
                } else {
                    Ok(x)
                }
            }),
        _ => Err("You should pass just one numeric argument"),
    }
}

fn fibonacci_until(x: u32) -> Vec<u32> {
    let mut numbers = vec![1, 1];
    let mut index = numbers.len() - 1;

    loop {
        let result = numbers[index - 1] + numbers[index];
        if result < x {
            index += 1;
            numbers.push(result);
        } else {
            break;
        }
    }

    numbers
}

fn format(numbers: Vec<u32>) -> String {
    numbers
        .into_iter()
        .fold(String::new(), |acc, x| format!("{} {}", acc, x))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fibonacci_works() {
        assert_eq!(fibonacci_until(10), vec![1, 1, 2, 3, 5, 8]);
        assert_eq!(fibonacci_until(15), vec![1, 1, 2, 3, 5, 8, 13]);
        assert_eq!(fibonacci_until(30), vec![1, 1, 2, 3, 5, 8, 13, 21]);
        assert_eq!(fibonacci_until(40), vec![1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}
