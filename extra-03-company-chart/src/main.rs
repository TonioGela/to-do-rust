use itertools::Itertools;
use std::env::args;
fn main() {
    let args: Vec<String> = args().dropping(1).collect_vec();
    let args_ref: Vec<&str> = args.iter().map(|x| x.as_ref()).collect_vec();
    let mut company = Company::new();
    company.add(String::from("Stronzi"));
    company.add(String::from("Marketing"));
    match &args_ref[..] {
        ["add", person, "to", department] => match company.get(*department) {
            Some(dep) => {
                dep.add(*person);
                println!("Company has now a person: {:?}", company)
            }
            None => println!("There is no department named {}", department),
        },
        ["print", department] => {
            match company.get(*department) {
                Some(dep) => println!("Say hello to department {:?}", dep),
                None => println!("There is no department named {}", department),
            };
            println!("foo")
        }
        _ => println!("ARGS --> {:?} <--", args),
    }
}

#[derive(Debug)]
struct Company {
    departments: Vec<Department>,
}

impl Company {
    fn new() -> Company {
        Company {
            departments: Vec::new(),
        }
    }

    fn add(&mut self, dep_name: String) {
        self.departments.push(Department::new(dep_name))
    }

    fn get(&mut self, dep_name: &str) -> Option<&mut Department> {
        self.departments.iter_mut().find(|dep| dep.name == dep_name)
    }

    fn remove(&mut self, dep_name: &str) {
        self.departments = self
            .departments
            .drain(..)
            .filter(|d| d.name != dep_name)
            .collect();
    }
}

#[derive(Debug)]
struct Department {
    name: String,
    persons: Vec<String>,
}

impl Department {
    fn new(name: String) -> Department {
        Department {
            name,
            persons: Vec::new(),
        }
    }

    fn add(&mut self, person: &str) {
        self.persons.push(person.to_owned())
    }

    fn remove(&mut self, person: &str) {
        self.persons = self.persons.drain(..).filter(|p| p != person).collect();
    }
}

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
