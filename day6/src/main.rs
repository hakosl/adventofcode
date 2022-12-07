use std::{fs, error::Error, process, env};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let expected_result = 7;
        run_test(input, expected_result);
    }

    #[test]
    fn test_part12() {
        let input = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let expected_result = 5;
        run_test(input, expected_result);
    }

    #[test]
    fn test_part13() {
        let input = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let expected_result = 6;
        run_test(input, expected_result);
    }

    #[test]
    fn test_part14() {
        let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let expected_result = 10;
        run_test(input, expected_result);
    }

    #[test]
    fn test_part15() {
        let input = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let expected_result = 11;
        run_test(input, expected_result);
    }

    
    #[test]
    fn test_part2() {
        let input = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let expected_result = 19;
        run_test2(input, expected_result);
    }

    #[test]
    fn test_part22() {
        let input = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let expected_result = 23;
        run_test2(input, expected_result);
    }

    #[test]
    fn test_part23() {
        let input = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let expected_result = 23;
        run_test2(input, expected_result);
    }

    #[test]
    fn test_part24() {
        let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let expected_result = 29;
        run_test2(input, expected_result);
    }

    #[test]
    fn test_part25() {
        let input = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let expected_result = 26;
        run_test2(input, expected_result);
    }

    fn run_test(input: String, expected_result: i32) {
        let actual_result = part1(input);
        assert_eq!(Ok(expected_result), actual_result);
    }

    fn run_test2(input: String, expected_result: i32) {
        let actual_result = part2(input);
        assert_eq!(Ok(expected_result), actual_result);
    }
}

fn main() {
    if let Err(e) = run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run () -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let challenge_input = fs::read_to_string(file_name)?;
    let challenge_solution = part2(challenge_input)?;
    println!("{challenge_solution}");
    Ok(())
}

fn part1(task_input: String) -> Result<i32, &'static str> {
    for i in 0..(task_input.len() - 4) {
        let signal = &task_input[i..(i+4)].as_bytes();

        let s1 = signal[0];
        let s2 = signal[1];
        let s3 = signal[2];
        let s4 = signal[3];

        if s1 != s2 && s1 != s3 && s1 != s4 && s2 != s3 && s2 != s4 && s3 != s4 {
            return Ok((i + 4) as i32);
        }
    }
    Err("Could not find signal")
}

fn part2(task_input: String) -> Result<i32, &'static str> {
    for i in 0..(task_input.len() - 14) {
        let signal = &task_input[i..(i+14)].as_bytes();

        if signal.into_iter().all(|s1| signal.into_iter().filter(|s2| s1 == *s2).count() == 1) {
            return Ok((i + 14) as i32);
        }
    }
    Err("Could not find signal")
}
