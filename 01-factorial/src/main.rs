use std::io::stdin;

fn main() {
    println!("Gimme a number");
    let line = read_line();
    match line.parse::<u128>() {
        Ok(n) => println!("The factorial of {} is {}", n, factorial(n)),
        Err(_) => println!("{} is not a number!", line),
    }
}

fn read_line() -> String {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_owned()
}

fn factorial(n: u128) -> u128 {
    let mut value: u128 = 1;
    for x in 1..=n {
        value = value * x;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::factorial;
    #[test]
    pub fn factorial_should_work() {
        test_factorial(1, 1);
        test_factorial(2, 2);
        test_factorial(3, 6);
        test_factorial(4, 24);
        test_factorial(5, 120);
        test_factorial(6, 720)
    }

    fn test_factorial(n: u128, result: u128) {
        assert_eq!(
            factorial(n),
            result,
            "Factorial of {} was not {}",
            n,
            result
        )
    }
}
