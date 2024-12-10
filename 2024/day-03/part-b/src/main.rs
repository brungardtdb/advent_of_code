use regex::Regex;
use std::{fs, path::Path, num::ParseIntError};

fn main() -> Result<(), ParseIntError> {
    let path = Path::new("./resources/input.txt");
    let contents = fs::read_to_string(path).expect("Could not find the input file!");
    let sum = get_instruction_sum(contents.as_str())?;
    println!("Sum of the instructions is: {}", sum);
    Ok(())
}

fn get_instruction_sum(input: &str) -> Result<i32, ParseIntError> {
    let re = Regex::new(r"mul\([0-9]*,[0-9]*\)|do\(\)|don't\(\)").expect("Failed to initialize Regex!");
    let matches = re.find_iter(input).map(|m| m.as_str()).collect::<Vec<_>>();
    let states = matches.into_iter().map(|m| InstructionState::from(m)).collect::<Vec<_>>();
    let mut sum = 0;
    let mut active = true;
    for state in states {
        match state {
            InstructionState::START => active = true,
            InstructionState::STOP => active = false,
            InstructionState::PROCESS(mul) => {
                if active {
                    sum += mul.get_product();
                }
            },
            InstructionState::INVALID => ()
        }
    }

    Ok(sum)
}
enum InstructionState {
    START,
    STOP,
    PROCESS(Mul),
    INVALID
}

impl InstructionState {
    fn from(input: &str) -> InstructionState {
        if input.contains("do()") {
            return InstructionState::START;
        } else if input.contains("don't()") {
            return InstructionState::STOP;
        }
        let mul_result = Mul::from(input);
        match mul_result {
            Ok(mul) => InstructionState::PROCESS(mul),
            Err(_) => InstructionState::INVALID
        }
    }
}

struct Mul {
    first: i32,
    second: i32,
}

enum MultParseError {
    MissingInteger(String),
    RegexError(String),
}

impl Mul {
    fn new(first: i32, second: i32) -> Self {
        Mul { first, second }
    }

    fn from(input: &str) -> Result<Self, MultParseError> {
        let re = Regex::new("[0-9]*");
        match re {
            Ok(regex) => {
                let matches: Vec<_> = regex
                    .find_iter(input)
                    .map(|m| m.as_str())
                    .filter(|s| !s.is_empty())
                    .collect();
                let mut it = matches.into_iter();
                let lhs = it.next();
                let rhs = it.next();
                if let (Some(first), Some(second)) = (lhs, rhs) {
                    if let (Ok(lhs_num), Ok(rhs_num)) =
                        (first.parse::<i32>(), second.parse::<i32>())
                    {
                        return Ok(Mul::new(lhs_num, rhs_num));
                    }
                }
                Err(MultParseError::MissingInteger(String::from(
                    "Could not find numbers to multiply",
                )))
            }
            Err(err) => Err(MultParseError::RegexError(err.to_string())),
        }
    }

    fn get_product(&self) -> i32 {
        self.first * self.second
    }
}

#[cfg(test)]
mod test {
    use crate::get_instruction_sum;

    #[test]
    fn handles_start_stop_instructions() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = get_instruction_sum(input);
        match result {
            Ok(num) => assert_eq!(48, num),
            Err(_) => unreachable!(),
        }
    }
}
