use std::{
    collections::HashSet,
    env,
    fs::{self},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_input() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let expected_result: Vec<u32> = vec![16, 38, 42, 22, 20, 19];
        let result = part1(input.into());
        assert_eq!(expected_result, result);
    }
    #[test]
    fn simple_input_part2() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let expected_result: Vec<u32> = vec![18, 52];
        let result = part2(input.into());
        assert_eq!(expected_result, result);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];

    let file_contents = read(String::from(file_path)).expect("Could not read file");
    let result = if part == "part1" {
        part1(file_contents)
    } else {
        part2(file_contents)
    };

    println!("{:?}", result.into_iter().sum::<u32>());
}

fn read(file_path: String) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn part1(file_contents: String) -> Vec<u32> {
    let parsed = parse_input(file_contents);
    let mut result = Vec::new();
    for line in parsed {
        let length = line.len() / 2;
        let mut chunks = line.chunks(length);
        let rucksack1 = chunks.next().unwrap().iter().collect::<HashSet<_>>();
        let rucksack2 = chunks.next().unwrap().iter().collect::<HashSet<_>>();

        let intersection = rucksack1.intersection(&rucksack2).collect::<Vec<_>>();
        result.push(*intersection[0].to_owned());
    }
    result
}

fn part2(file_contents: String) -> Vec<u32> {
    let parsed = parse_input(file_contents);
    let mut result = Vec::new();
    for line in parsed.chunks(3) {
        let mut rucksack1 = line[0].iter().collect::<HashSet<_>>();
        let rucksack2 = line[1].iter().collect::<HashSet<_>>();
        let rucksack3 = line[2].iter().collect::<HashSet<_>>();
        rucksack1.retain(|item| rucksack2.contains(item) && rucksack3.contains(item));
        result.push(**rucksack1.iter().next().unwrap());
    }
    result
}

fn parse_input(file_contents: String) -> Vec<Vec<u32>> {
    file_contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c.is_ascii_lowercase() {
                        c as u32 - 96
                    } else {
                        c as u32 - 64 + 26
                    }
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
