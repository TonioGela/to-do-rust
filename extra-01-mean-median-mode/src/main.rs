use std::collections::HashMap;

fn main() {
    let mut numbers: [i32; 13] = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];

    println!("AVERAGE: {}", mean(&numbers));
    println!("MEDIAN: {}", median(&mut numbers));
    println!("MODE: {}", mode(&numbers));
}

fn mean(xs: &[i32]) -> f32 {
    xs.iter().sum::<i32>() as f32 / xs.len() as f32
}

fn median(xs: &mut [i32]) -> i32 {
    xs.sort();
    let index = xs.len() / 2;
    xs[index + 1]
}

fn mode(xs: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    for &n in xs {
        *counts.entry(n).or_insert(0) += 1;
    }

    let mode = counts
        .into_iter()
        .max_by_key(|&(_, val)| val)
        .map(|x| x.0)
        .expect("KABOOM");

    mode
}

//* p1.distance(&p2);
//* (&p1).distance(&p2);
//* The first one looks much cleaner. This automatic referencing behavior works
//* because methods have a clear receiver—the type of self. Given the receiver and
//* name of a method, Rust can figure out definitively whether the method is
//* reading (&self), mutating (&mut self), or consuming (self). The fact that
//* Rust makes borrowing implicit for method receivers is a big part of making
//* ownership ergonomic in practice.
