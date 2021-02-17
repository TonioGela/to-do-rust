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

//TODO add tests for factorial!
