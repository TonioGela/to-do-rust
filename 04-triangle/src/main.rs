use std::env::args;

fn main() -> Result<(), &'static str> {
    //* See https://pzol.github.io/getting_rusty/posts/20140417_destructuring_in_rust/
    match args().collect::<Vec<String>>().as_slice() {
        [_, x] => x
            .parse::<i64>()
            .map_err(|_| "Cannot parse the argument as int")
            .map(|n| {
                let mut str = String::new();
                for _ in 0..=n {
                    str.push('*');
                    println!("{}", str)
                }
            }),
        _ => Err("You should pass one argument!"),
    }
}
