use itertools::Itertools;
use std::fs;

#[cfg(test)]
mod test {
    use super::*;

    static input: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn simple_input_preprocess() {
        let expected_stacks = vec!["ZN", "MCD", "P"];
        let expected_instructions = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
        let (stacks_local, instructions) = preprocess(String::from(input));

        assert_eq!(expected_stacks, stacks_local);
        assert_eq!(expected_instructions, instructions);
    }
    #[test]
    fn simple_input_runs() {
        let instructions = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];

        let stacks = vec!["ZN".to_string(), "MCD".to_string(), "P".to_string()];
        let expected_output = "CMZ";
        let actual_output = part1(stacks, instructions);

        assert_eq!(expected_output, actual_output);
    }
    #[test]
    fn simple_input_runs_part2() {
        let instructions = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];

        let stacks = vec!["ZN".to_string(), "MCD".to_string(), "P".to_string()];
        let expected_output = "MCD";
        let actual_output = part2(stacks, instructions);

        assert_eq!(expected_output, actual_output);
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let (stacks, instructions) = preprocess(file_contents);
    let result = part2(stacks, instructions);
    println!("{result}");
}

fn part1(stacks: Vec<String>, instructions: Vec<(u32, u32, u32)>) -> String {
    let mut output_stacks: Vec<String> = stacks.into_iter().map(|s| s.clone()).collect();
    for (count, from, to) in instructions {
        for _ in 0..count {
            let from_stack = &output_stacks[(from - 1) as usize];
            let to_stack = &output_stacks[(to - 1) as usize];

            let (new_from_stack, moved_char) = from_stack.split_at(from_stack.len() - 1);

            let new_to_stack = [to_stack, moved_char].join("");

            output_stacks[(from - 1) as usize] = new_from_stack.to_string();
            let new_owned_stack = new_to_stack.to_owned();
            output_stacks[(to - 1) as usize] = new_owned_stack.to_string();
        }
    }

    output_stacks
        .into_iter()
        .map(|s| s.chars().last().unwrap())
        .collect()
}

fn part2(stacks: Vec<String>, instructions: Vec<(u32, u32, u32)>) -> String {
    let mut output_stacks: Vec<String> = stacks.into_iter().map(|s| s.clone()).collect();
    for (count, from, to) in instructions {
        let from_stack = &output_stacks[(from - 1) as usize];
        let to_stack = &output_stacks[(to - 1) as usize];

        let (new_from_stack, moved_char) = from_stack.split_at(from_stack.len() - (count as usize));

        let new_to_stack = [to_stack, moved_char].join("");

        output_stacks[(from - 1) as usize] = new_from_stack.to_string();
        let new_owned_stack = new_to_stack.to_owned();
        output_stacks[(to - 1) as usize] = new_owned_stack.to_string();
    }

    output_stacks
        .into_iter()
        .map(|s| s.chars().last().unwrap())
        .collect()
}

fn preprocess(input: String) -> (Vec<String>, Vec<(u32, u32, u32)>) {
    let lines = input.lines();
    let number_of_stacks = (lines.clone().next().unwrap().len() + 1) / 4;
    let mut instructions: Vec<(u32, u32, u32)> = Vec::new();

    let mut stacks = vec![String::from(""); number_of_stacks as usize];
    let mut parsing_first_part = true;
    for line in lines {
        let line_bytes = line.as_bytes();
        if line == "" {
            parsing_first_part = false;
            continue;
        }
        if parsing_first_part {
            for i in 0..number_of_stacks {
                let stack_item = line_bytes[(i * 4) + 1] as char;
                if stack_item == ' ' || stack_item.is_digit(10) {
                    continue;
                }

                let temp_stack = &stacks[i];
                let mut new_stack = "".to_string();

                new_stack.push(line_bytes[(i * 4) + 1] as char);
                new_stack.push_str(temp_stack);

                stacks[i] = new_stack;
            }
        } else {
            let (count, from, to) = line
                .split(" ")
                .skip(1)
                .step_by(2)
                .map(|x| {
                    x.parse().unwrap()
                })
                .next_tuple()
                .unwrap();
            instructions.push((count, from, to));
        }
    }

    (stacks, instructions)
}
