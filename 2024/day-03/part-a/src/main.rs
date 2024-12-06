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
    let re = Regex::new(r"mul\([0-9]*,[0-9]*\)").expect("Failed to initialize Regex!");
    let matches = re.find_iter(input).map(|m| m.as_str()).collect::<Vec<_>>();
    let sum = matches.into_iter()
        .map(|m| {
            let mul_result = Mul::from(m);
            match mul_result {
                Ok(mul) => mul.get_product(),
                Err(_) => 0,
            }
        })
        .sum();

    Ok(sum)
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
    fn test_case_eq_161() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = get_instruction_sum(input);
        match result {
            Ok(num) => assert_eq!(161, num),
            Err(_) => unreachable!(),
        }
    }
}
