use std::{env, fs};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_input() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let expected_result = vec![false, false, false, true, true, false];

        let task_input = preprocess(input.to_string());
        let result = part1(task_input);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_simple_input_part2() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let expected_result = vec![false, false, true, true, true, true];

        let task_input = preprocess(input.to_string());
        let result = part2(task_input);
        assert_eq!(result, expected_result);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part_query = &args[2];

    let file_contents = read(String::from(file_path)).expect("Could not read file");

    let task_input = preprocess(file_contents);
    let result = if part_query == "part1" {
        part1(task_input)
    } else {
        part2(task_input)
    };
    let sum: i32 = result.into_iter().map(|r| if r { 1 } else { 0 }).sum();
    println!("{sum}");
}

fn part1(input: Vec<((i32, i32), (i32, i32))>) -> Vec<bool> {
    input
        .into_iter()
        .map(|((x1, x2), (y1, y2)): ((i32, i32), (i32, i32))| {
            (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2)
        })
        .collect()
}

fn part2(input: Vec<((i32, i32), (i32, i32))>) -> Vec<bool> {
    input
        .into_iter()
        .map(|((x1, x2), (y1, y2)): ((i32, i32), (i32, i32))| {
            (x1 <= y1 && x2 >= y1)
                || (x1 <= y2 && x2 >= y2)
                || (y1 <= x1 && y2 >= x1)
                || (y1 <= x2 && y2 >= x2)
        })
        .collect()
}

fn preprocess(file_input: String) -> Vec<((i32, i32), (i32, i32))> {
    file_input
        .lines()
        .map(|line| {
            let mut s = line.split(",");
            let mut first = s.next().unwrap().split("-").map(|c| c.parse().unwrap());
            let mut second = s.next().unwrap().split("-").map(|c| c.parse().unwrap());

            (
                (first.next().unwrap(), first.next().unwrap()),
                (second.next().unwrap(), second.next().unwrap()),
            )
        })
        .collect()
}

fn read(file_path: String) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}
