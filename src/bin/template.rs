use std::{
    fs::{File, OpenOptions},
    io::Write,
    process, env,
};

const SOLUTION_TEMPLATE: &str = r#"
#[aoc_generator(day{})]
fn input_generator(input: &str) -> String {
    input.into()
}

#[aoc(day{}, part1)]
fn part_one(input: &str) -> i32 {
    0
}

#[aoc(day{}, part2)]
fn part_two(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = "";
        let result = part_one(&input_generator(input));
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_1() {
        let input = "";
        let result = part_two(&input_generator(input));
        assert_eq!(result, 0);
    }
}
"#;

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

pub fn handle(day: u8) {
    let mod_path = format!{"src/solutions/mod.rs"};
    let solution_path = format!("src/solutions/day{day}.rs");
    let input_path = format!("input/2015/day{day}.txt");

    let mut solution_file = match safe_create_file(&solution_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            process::exit(1);
        }
    };

    match solution_file.write_all(
        SOLUTION_TEMPLATE
            .replace("{}", &day.to_string())
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &solution_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            process::exit(1);
        }
    }

    let mut mod_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(mod_path)
        .expect("Couldn't open solutions module file");

    let mod_string = format!{"mod day{day};\n"};
    mod_file
        .write(mod_string.as_bytes())
        .expect("Could not write to mod file");

    match safe_create_file(&input_path) {
        Ok(mut file) => {
            match file.write_all(
                b""
            ) {
                Ok(()) => {
                    println!("Created input file \"{}\"", &input_path);
                }
                Err(e) => {
                    eprintln!("Failed to write input contents: {e}");
                    process::exit(1);
                }
            }
        },
        Err(_) => {
            println!("Input file already exists");
        }
    };

    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");
    println!("Type `cargo aoc -d {}` to run your solution.", day);
}

fn main() {
    let args = env::args();

    let day_string = args
        .last()
        .expect("Please include a day number\nExample: cargo template 1");
    let day_number = day_string
        .parse()
        .expect("Please provide a valid number");

    handle(day_number)
}
