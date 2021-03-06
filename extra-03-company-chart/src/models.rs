use colored::*;
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

    pub fn add(&mut self, department: String) -> Result<Message, Error> {
        if self.has(department.as_str()) {
            Err(Error::DuplicatedDepartment { name: department })
        } else {
            self.departments.push(Department::new(department.clone()));
            Ok(Message::AddedDepartment { name: department })
        }
    }

    pub fn get(&mut self, department: &str) -> Result<&mut Department, Error> {
        self.departments
            .iter_mut()
            .find(|dep| dep.name == department)
            .ok_or_else(|| Error::DepartmentNotFound {
                name: department.to_owned(),
            })
    }

    pub fn remove(&mut self, department: &str) -> Result<Message, Error> {
        let dep = self.get(department)?;
        if dep.is_empty() {
            self.departments.retain(|d| d.name != department);
            Ok(Message::RemovedDepartment {
                name: department.to_owned(),
            })
        } else {
            Err(Error::NonEmptyDepartment {
                name: department.to_owned(),
            })
        }
    }

    pub fn print(&self) -> Result<Message, Error> {
        Ok(Message::Print {
            message: self.to_string(),
        })
    }

    fn has(&self, department: &str) -> bool {
        (&self.departments)
            .into_iter()
            .any(|dep| dep.name == department)
    }

    fn is_empty(&self) -> bool {
        self.departments.is_empty() || self.departments.iter().all(|d| d.is_empty())
    }
}

impl Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let divider = String::from("--------------------------------")
            .bold()
            .white();
        let mut result: String = String::new();
        if self.is_empty() {
            result.push_str("This is an empty company!")
        } else {
            self.departments.iter().for_each(|d| {
                if result.is_empty() {
                    result.push_str(format!("{}\n", divider).as_str());
                } else {
                    result.push_str("\n");
                }
                result.push_str(d.to_string().as_str());
            });
        }
        write!(f, "{}", result)
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

    pub fn add(&mut self, person: &str) -> Result<Message, Error> {
        if self.has(person) {
            Err(Error::DuplicatedUser {
                name: person.to_owned(),
                department: self.name.to_owned(),
            })
        } else {
            self.persons.push(person.to_owned());
            Ok(Message::AddedUser {
                name: person.to_owned(),
                department: self.name.to_owned(),
            })
        }
    }

    pub fn remove(&mut self, person: &str) -> Result<Message, Error> {
        if self.has(person) {
            self.persons.retain(|p| p != person);
            Ok(Message::RemovedUser {
                name: person.to_owned(),
                department: self.name.to_owned(),
            })
        } else {
            Err(Error::UserNotFound {
                name: person.to_owned(),
                department: self.name.to_owned(),
            })
        }
    }

    pub fn print(&self) -> Result<Message, Error> {
        Ok(Message::Print {
            message: self.to_string(),
        })
    }

    fn has(&self, person: &str) -> bool {
        (&self.persons).into_iter().any(|p| p == person)
    }

    fn is_empty(&self) -> bool {
        self.persons.is_empty()
    }
}

impl Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut persons: String = String::new();
        if self.is_empty() {
            persons.push_str(
                format!(
                    "{}",
                    String::from("No one, apparently that's an empty department").purple()
                )
                .as_str(),
            )
        } else {
            self.persons.iter().for_each(|p| {
                if !persons.is_empty() {
                    persons.push_str(", ");
                }
                persons.push_str(p)
            });
        }
        write!(
            f,
            "{}: {}\n{}: {}\n{}",
            String::from("Department").bold().blue(),
            self.name,
            String::from("Staff").bold().blue(),
            persons,
            String::from("--------------------------------")
                .bold()
                .white()
        )
    }
}

pub enum Error {
    DuplicatedUser { name: String, department: String },
    DuplicatedDepartment { name: String },
    UserNotFound { name: String, department: String },
    DepartmentNotFound { name: String },
    NonEmptyDepartment { name: String },
    ParsingError,
}

pub enum Message {
    AddedUser { name: String, department: String },
    RemovedUser { name: String, department: String },
    AddedDepartment { name: String },
    RemovedDepartment { name: String },
    Print { message: String },
}
