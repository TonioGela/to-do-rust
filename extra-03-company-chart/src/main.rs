mod models;
use colored::*;
use itertools::Itertools;
use models::Company;
use savefile::prelude::*;
use std::{env::args, process::exit};

#[macro_use]
extern crate savefile_derive;

// TODO: Add tests

fn main() {
    let args: Vec<String> = args().dropping(1).collect_vec();
    let args_ref: Vec<&str> = args.iter().map(|x| x.as_ref()).collect_vec();

    let mut company: Company =
        load_file::<Company>("save.bin", 0).unwrap_or_else(|_| Company::new());

    let result: Result<models::Message, models::Error> = handle_commands(args_ref, &mut company);

    handle_result(result);

    save_file("save.bin", 0, &company).unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1)
    });
}

fn handle_commands(
    args_ref: Vec<&str>,
    company: &mut Company,
) -> Result<models::Message, models::Error> {
    match &args_ref[..] {
        ["add", "department", dep] => company.add((*dep).to_owned()),
        ["remove", "department", dep] => company.remove(*dep),
        ["add", person, "to", dep] => company.get(*dep)?.add(*person),
        ["remove", person, "from", dep] => company.get(*dep)?.remove(*person),
        ["show", "department", dep] => company.get(*dep)?.print(),
        ["show"] => company.print(),
        _ => Err(models::Error::ParsingError),
    }
}

fn handle_result(result: Result<models::Message, models::Error>) {
    match result {
        Ok(m) => println!("{}", format_message(m).green()),
        Err(e) => {
            eprintln!("{}", format_error(e).red());
            exit(1)
        }
    }
}

fn format_error(error: models::Error) -> String {
    match error {
        models::Error::DuplicatedUser { name, department } => {
            format!("{} already present in department {}", name.bold(), department.bold())
        }
        models::Error::DuplicatedDepartment { name } => {
            format!("Department {} already present", name.bold())
        }
        models::Error::UserNotFound { name, department } => {
            format!("{} not found in department {}", name.bold(), department.bold())
        }
        models::Error::DepartmentNotFound { name } => format!("Department {} not found", name.bold()),
        models::Error::NonEmptyDepartment { name } => format!("Department {} not empty", name.bold()),
        models::Error::ParsingError => format!("{}{}",
            "Unrecognised command, try one of these:\n  show,\n  add department <department>,\n  remove department <department>,",
            "\n  show department <department>,\n  add <person> to <department>,\n  remove <person> from <department>"
            )
    }
}

fn format_message(message: models::Message) -> String {
    match message {
        models::Message::AddedDepartment { name } => format!("Added department {}", name.bold()),
        models::Message::AddedUser { name, department } => {
            format!("Added {} to department {}", name.bold(), department.bold())
        }
        models::Message::RemovedUser { name, department } => {
            format!(
                "Removed {} from department {}",
                name.bold(),
                department.bold()
            )
        }
        models::Message::RemovedDepartment { name } => {
            format!("Removed department {}", name.bold())
        }
        models::Message::Print { message } => format!("{}", message),
    }
}

//

// #![deny(clippy::all)] or #[deny(clippy::all)]

// THIS? #[warn(
//     clippy::all,
//     clippy::restriction,
//     clippy::pedantic,
//     clippy::nursery,
//     clippy::cargo
// )] or all is enough?

// on top of main.rs or in clippy.toml

// cargo clippy or cargo clean && cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::restriction -W clippy::nursery -W clippy::cargo

// deny or warn?
