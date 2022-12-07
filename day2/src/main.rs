use std::{fs, env};
// x = Rock 1
// y = paper 2
// z = scissor 3
// X = L, 0
// Y = D, 3
// Z = W, 6
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let preprocessed = contents
        .replace("A", "X")
        .replace("B", "Y")
        .replace("C", "Z");

    let points: i32 = preprocessed
        .lines()
        .map(|line| {
        let split: Vec<&str> = line.split(" ").collect();
        let enemy = split[0];
        let me = split[1];
        let points_result = match me {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Could not parse")
        };

        let result_from_choice = match me {
            "X" => {
                match enemy {
                    "X" => 3,
                    "Y" => 1,
                    "Z" => 2,
                    _ => panic!("asd")
                }
            },
            "Y" => 
                match enemy {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => panic!("asd")
                },
            "Z" => 
                match enemy {
                    "X" => 2,
                    "Y" => 3,
                    "Z" => 1,
                    _ => panic!()
                },
            _ => panic!("Could not parse")
        };
        (points_result, result_from_choice)
    })
    .map(|(points_result, result_from_choice)| points_result + result_from_choice)
    .sum();

    println!("{:?}", points);
}
