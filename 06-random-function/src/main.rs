use rand::{thread_rng, Rng};
fn main() {
    // ? I don't get the point in doing this exercize
    let number = match thread_rng().gen_range(0..3) {
        0 => function1(),
        1 => function2(),
        2 => function3(),
        _ => panic!("Fooo!"),
    };
    println!("{}", number)
}

fn function1() -> i8 {
    1
}

fn function2() -> i8 {
    2
}

fn function3() -> i8 {
    3
}
