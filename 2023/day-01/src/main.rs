use std::{fs, path::Path};

// current number is 53453
// for some reason this isn't the correct answer?
// all test cases are passing, I am at a loss

fn main() {
    let path = Path::new("./resources/day_1.txt");
    let contents = fs::read_to_string(path).expect("Could not find the input file!");
    let sum: u32 = get_calibration_sum(&contents);
    println!("Calibration sum is: {}", sum);
}

fn get_calibration_value(input: &str) -> u32 {
    let p = DigitParser::new(input);
    p.get_calibration_value()
}

fn get_calibration_sum(input: &str) -> u32 {
    let parts = input.lines();
    let mut total: u32 = 0;
    for p in parts {
        total += get_calibration_value(&p);
    }
    total
}

#[derive(Debug)]
struct DigitParser {
    indices: Vec<DigitIndexMap>,
}

#[derive(Debug)]
struct DigitIndexMap {
    digit: u32,
    digit_indices: Vec<u32>,
}

impl DigitParser {
    fn new(input: &str) -> DigitParser {
        DigitParser {
            indices: DigitParser::parse_input(input),
        }
    }

    fn parse_input(input: &str) -> Vec<DigitIndexMap> {
        let mut result: Vec<DigitIndexMap> = Vec::new();
        for d in DIGIT_MAP {
            let mut indices = Vec::new();
            let ind: Vec<_> = input.match_indices(d.0).collect();
            for i in ind {
                indices.push(i.0 as u32);
            }
            let map = DigitIndexMap {
                digit: d.1 as u32,
                digit_indices: indices,
            };
            result.push(map);
        }

        let chars = input.chars().enumerate();
        for (i, ch) in chars {
            let num: Option<u32> = ch.to_digit(10);
            match num {
                Some(n) => {
                    for r in &mut result[..] {
                        if r.digit == n {
                            r.digit_indices.push(i as u32);
                        }
                    }
                }
                None => (),
            }
        }
        result
    }

    pub fn get_calibration_value(&self) -> u32 {
        let mut lowest_index_map: Option<DigitIndexMap> = None;
        let mut highest_index_map: Option<DigitIndexMap> = None;
        let mut lowest_index: u32 = u32::MAX;
        let mut highest_index: u32 = u32::MIN;

        self.indices
            .iter()
            .map(|map| {
                let lowest = map.digit_indices.iter().min();
                let highest = map.digit_indices.iter().max();
                match lowest {
                    Some(l) => {
                        if lowest_index > *l {
                            lowest_index = *l;
                            lowest_index_map = Some(DigitIndexMap {
                                digit: map.digit,
                                digit_indices: map.digit_indices.clone(),
                            });
                        }
                    }
                    None => (),
                }
                match highest {
                    Some(h) => {
                        if highest_index < *h {
                            highest_index = *h;
                            highest_index_map = Some(DigitIndexMap {
                                digit: map.digit,
                                digit_indices: map.digit_indices.clone(),
                            });
                        }
                    }
                    None => (),
                }
            })
            .count();

        let mut tens: u32 = 0;
        let mut ones: u32 = 0;
        match highest_index_map {
            Some(map) => ones = map.digit,
            None => (),
        }
        match lowest_index_map {
            Some(map) => tens = map.digit * 10,
            None => (),
        }
        tens + ones
    }
}

static DIGIT_MAP: &[(&str, u8)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibration_value_is_12() {
        assert_eq!(get_calibration_value("1abc2"), 12);
    }

    #[test]
    fn calibration_value_is_38() {
        assert_eq!(get_calibration_value("pqr3stu8vwx"), 38);
    }

    #[test]
    fn calibration_value_is_15() {
        assert_eq!(get_calibration_value("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn calibration_value_is_77() {
        assert_eq!(get_calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn calibration_sum_is_142() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(get_calibration_sum(input), 142);
    }
    #[test]
    fn calibration_value_is_29() {
        assert_eq!(get_calibration_value("two1nine"), 29);
    }

    #[test]
    fn calibration_value_is_83() {
        assert_eq!(get_calibration_value("eightwothree"), 83);
    }

    #[test]
    fn calibration_value_is_13() {
        assert_eq!(get_calibration_value("abcone2threexyz"), 13);
    }

    #[test]
    fn calibration_value_is_24() {
        assert_eq!(get_calibration_value("xtwone3four"), 24);
    }

    #[test]
    fn calibration_value_is_42() {
        assert_eq!(get_calibration_value("4nineeightseven2"), 42);
    }

    #[test]
    fn calibration_value_is_14() {
        assert_eq!(get_calibration_value("zoneight234"), 14);
    }

    #[test]
    fn calibration_value_is_76() {
        assert_eq!(get_calibration_value("7pqrstsixteen"), 76);
    }

    #[test]
    fn calibration_sum_is_281() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(get_calibration_sum(input), 281);
    }

    #[test]
    fn calibration_value_is_18() {
        assert_eq!(get_calibration_value("oneight"), 18);
    }
}
