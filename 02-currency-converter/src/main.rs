use colored::*;
use std::{env::args, fs, process::exit};
use toml::*;

fn main() {
    let maybe_conf = read_config().and_then(parse_config);
    // first member of args is the name of the executable
    let maybe_expression = Expression::from(args().skip(1).collect());

    let result = maybe_conf.and_then(|conf| {
        maybe_expression.and_then(|expr| {
            let from_rate = conf[&expr.currency_from].as_float();
            let to_rate = conf[&expr.currency_to].as_float();
            let rates = from_rate.zip(to_rate);
            rates
                .map(|(from, to)| &expr.value * from / to)
                .ok_or("Unknown currencies")
        })
    });

    match result {
        Ok(c) => println!("{}", c),
        Err(s) => {
            eprintln!("{}", s.red().bold());
            exit(1)
        }
    }
}

#[derive(Debug, PartialEq)]
struct Expression {
    value: f64,
    currency_from: String,
    currency_to: String,
}

impl Expression {
    fn from(args: Vec<String>) -> Result<Expression, &'static str> {
        read_expression(args).and_then(parse_expression).map(
            |(value, currency_from, currency_to)| Expression {
                value,
                currency_from,
                currency_to,
            },
        )
    }
}

fn read_expression(args: Vec<String>) -> Result<(String, String, String), &'static str> {
    if args.len() == 4 && args[2] == "in" {
        Ok((args[0].to_owned(), args[1].to_owned(), args[3].to_owned()))
    } else {
        Err("The arguments should be four and in the form <value> <currency A> in <currency B>")
    }
}

fn parse_expression(
    expr_values: (String, String, String),
) -> Result<(f64, String, String), &'static str> {
    let (a, b, c) = expr_values;
    a.trim()
        .parse()
        .map_err(|_| "First argument should be a number")
        .map(|a| (a, b, c))
}

fn parse_config(string: String) -> Result<Value, &'static str> {
    string
        .parse()
        .map_err(|_| "Unable to parse the config as TOML")
}

fn read_config() -> Result<String, &'static str> {
    fs::read_to_string(format!(
        "{home}/.config/currencies.toml",
        home = env!("HOME")
    ))
    .map_err(|_| "Unable to read config file at ~/.config/currencies.toml")
}

#[cfg(test)]
mod expression_test {
    use crate::Expression;

    fn expr(vec: Vec<&str>) -> Result<Expression, &'static str> {
        Expression::from(vecs(vec))
    }

    fn vecs(vec: Vec<&str>) -> Vec<String> {
        let mut result = Vec::new();
        for i in vec {
            result.push(i.to_string())
        }
        result
    }

    #[test]
    fn from() {
        assert_eq!(
            Ok(Expression {
                value: 10.0,
                currency_from: format!("EUR"),
                currency_to: format!("USD")
            }),
            expr(vec!["10", "EUR", "in", "USD"])
        );

        assert_eq!(
            Err("First argument should be a number"),
            expr(vec!["foo", "EUR", "in", "USD"])
        );

        assert_eq!(
            Err(
                "The arguments should be four and in the form <value> <currency A> in <currency B>"
            ),
            expr(vec!["foo"])
        );

        // idk how PARTIAL quality for float works
        assert_ne!(
            Expression {
                value: 10.0,
                currency_from: format!("EUR"),
                currency_to: format!("USD")
            },
            Expression {
                value: 1.0,
                currency_from: format!("EUR"),
                currency_to: format!("USD")
            }
        )
    }
}
