use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut results = Vec::new();
    results.push(0);

    for line in contents.lines() {
        if line.is_empty() {
            results.push(0);
        } else {
            let i = line.trim().parse::<i32>().unwrap();
            *results.last_mut().unwrap() += i;
        }

    }
    results
        .sort();
    results.reverse();
    let res = results[0] + results[1] + results[2];
    println!("{res}");
}