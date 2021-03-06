use crate::models::Company;
use itertools::Itertools;
use savefile::prelude::*;
use std::{env::args, process::exit};

#[macro_use]
extern crate savefile_derive;

// ! FIXME: it's actually possible to remove a non empty department
// TODO: refactor it using -> Result<(), Custom Error ADT> for (&mut self) methods
// TODO: and simply handle Result in main (should be more testable)

fn main() {
    let args: Vec<String> = args().dropping(1).collect_vec();
    let args_ref: Vec<&str> = args.iter().map(|x| x.as_ref()).collect_vec();

    let mut company = match load_file::<Company>("save.bin", 0) {
        Err(_) => Company::new(),
        Ok(company) => company,
    };

    match &args_ref[..] {
        ["add", "department", dep] => {
            if company.has(*dep) {
                log_and_fail(format!("Department {} already exists", dep))
            } else {
                company.add(String::from(*dep));
                println!("Successfully added department {} to company", dep);
            }
        }
        ["remove", "department", dep] => {
            if company.has(dep) {
                company.remove(dep);
                println!("Successfully removed {} department", dep);
            } else {
                log_and_fail(format!("There is no {} department", dep))
            }
        }
        ["add", person, "to", dep] => match company.get(*dep) {
            Some(department) => {
                department.add(*person);
                println!("Successfully added {} to {}", person, dep);
            }
            None => log_and_fail(format!("There is no {} department", dep)),
        },
        ["remove", person, "from", dep] => match company.get(*dep) {
            Some(department) => {
                if department.has(person) {
                    department.remove(person)
                } else {
                    log_and_fail(format!("There's no {} in {} department", person, dep))
                }
            }
            None => log_and_fail(format!("There's no {} department", dep)),
        },
        ["show", "department", department] => {
            match company.get(*department) {
                Some(dep) => println!("{}", dep),
                None => log_and_fail(format!("There is no department named {}", department)),
            };
            println!("foo")
        }
        ["show"] => println!("{}", company),
        _ => println!(
            "Unrecognised command, try one of these:
  show, 
  add department <department>, 
  remove department <department>,
  show department <department>, 
  add <person> to <department>, 
  remove <person> from <department>",
        ),
    }

    match save_file("save.bin", 0, &company) {
        Ok(_) => (),
        Err(e) => log_and_fail(format!("{}", e)),
    }
}

fn log_and_fail(message: String) -> ! {
    eprintln!("{}", message);
    exit(1);
}

mod models {
    use std::fmt::Display;

    #[derive(Debug, Savefile)]
    pub struct Company {
        departments: Vec<Department>,
    }

    impl Company {
        pub fn new() -> Company {
            Company {
                departments: Vec::new(),
            }
        }

        pub fn add(&mut self, dep_name: String) {
            self.departments.push(Department::new(dep_name))
        }

        pub fn has(&self, department: &str) -> bool {
            (&self.departments)
                .into_iter()
                .any(|dep| dep.name == department)
        }

        pub fn get(&mut self, dep_name: &str) -> Option<&mut Department> {
            self.departments.iter_mut().find(|dep| dep.name == dep_name)
        }

        pub fn remove(&mut self, dep_name: &str) {
            self.departments = self
                .departments
                .drain(..)
                .filter(|d| d.name != dep_name)
                .collect();
        }
    }

    impl Display for Company {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut departments: String = String::new();
            self.departments.iter().for_each(|d| {
                if !departments.is_empty() {
                    departments.push_str("\n");
                }
                departments.push_str(d.to_string().as_str());
            });
            write!(f, "{}", departments)
        }
    }

    #[derive(Debug, Savefile)]
    pub struct Department {
        name: String,
        persons: Vec<String>,
    }

    impl Department {
        pub fn new(name: String) -> Department {
            Department {
                name,
                persons: Vec::new(),
            }
        }

        pub fn has(&self, person: &str) -> bool {
            (&self.persons).into_iter().any(|p| p == person)
        }

        pub fn add(&mut self, person: &str) {
            self.persons.push(person.to_owned())
        }

        pub fn remove(&mut self, person: &str) {
            self.persons = self.persons.drain(..).filter(|p| p != person).collect();
        }
    }

    impl Display for Department {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut persons: String = String::new();
            self.persons.iter().for_each(|p| {
                if !persons.is_empty() {
                    persons.push_str(", ");
                }
                persons.push_str(p)
            });
            write!(f, "Department: {}\nStaff: {}", self.name, persons)
        }
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
