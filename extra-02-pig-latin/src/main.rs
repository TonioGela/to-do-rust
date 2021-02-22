//! I HATE THIS FUCKING THING

fn main() {
    println!("{}", pig("Allo"));
}

fn pig(input: &str) -> String {
    if input.len() == 0 {
        String::new()
    } else {
        let (head, tail) = input.split_at(1);
        match head.to_lowercase().as_str() {
            "a" | "e" | "i" | "o" | "u" => format!("{}-hay", input.to_owned()),
            _ => format!("{}-{}ay", tail, head),
        }
    }
}

// The difference between &str and String::as_str is the lifetime.
// A String is allocated in the heap, so the lifetime of the &str contained inside
// (that is returned by the as_str() method provided by the standard library) is
// the one of the container, that is dropped at the end of the scope in which
// the String lives.
// The string literal (&'static str) instead is in your code and is valid for
// the static lifetime, i.e. for the whole duration of your program.
