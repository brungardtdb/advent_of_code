use std::{fs, path::Path};

fn main() {
    let path = Path::new("./resources/input.txt");
    let contents = fs::read_to_string(path).expect("Could not find the input file!");
    let qty = get_safe_report_qty(contents);
    println!("Number of safe reports: {}", qty);
}

fn get_safe_report_qty(input: String) -> i32 {
    input
        .lines()
        .map(|line| SafetyLevel::from(Report::from(line), true))
        .filter(|lvl| lvl.safe())
        .collect::<Vec<_>>()
        .into_iter()
        .count() as i32
}

struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn from(input: &str) -> Self {
        let mut rep = Report {
            levels: Vec::<i32>::new(),
        };
        let mut split = input.split_whitespace();
        while let Some(next) = split.next() {
            if let Ok(num) = next.parse::<i32>() {
                let _ = &rep.levels.push(num);
            }
        }
        rep
    }
}

impl Clone for Report {
    fn clone(&self) -> Report {
        Report {
            levels: self.levels.clone(),
        }
    }
}

enum SafetyLevel {
    Safe(Report),
    Unsafe,
}

enum Direction {
    Increasing,
    Decreasing,
    Undetermined,
}

impl SafetyLevel {
    fn from(input: Report, include_problem_dampener: bool) -> Self {
        let mut direction = Direction::Undetermined;
        let mut previous = None;
        let it = input.levels.iter();
        for level in it {
            if !previous.is_some() {
                previous = Some(*level);
                continue;
            }

            if let Some(prev) = previous {
                if matches!(direction, Direction::Undetermined) {
                    direction = get_direction(&prev, level);
                }
                if !level_is_safe(&prev, level, &direction) {
                    dbg!("level is not safe: {}", &level);
                    if include_problem_dampener {
                        for (idx, _) in input.levels.iter().enumerate() {
                            let mut input_copy = input.clone();
                            input_copy.levels.remove(idx);
                            let safety_level = SafetyLevel::from(input_copy, false);
                            if safety_level.safe() {
                                return safety_level;
                            } else {
                                continue;
                            }
                        }
                        return SafetyLevel::Unsafe;
                    } else {
                        return SafetyLevel::Unsafe;
                    }
                }
                previous = Some(*level);
            }
        }
        SafetyLevel::Safe(input)
    }

    fn safe(&self) -> bool {
        matches!(*self, SafetyLevel::Safe(_))
    }
}

fn level_is_safe(prev: &i32, level: &i32, direction: &Direction) -> bool {
    !changed_direction(&prev, level, &direction) && !exceeds_range(&prev, level, 1, 3)
}

fn get_direction(prev: &i32, level: &i32) -> Direction {
    if prev < level {
        Direction::Increasing
    } else if prev > level {
        Direction::Decreasing
    } else {
        Direction::Undetermined
    }
}

fn changed_direction(prev: &i32, level: &i32, direction: &Direction) -> bool {
    match direction {
        &Direction::Increasing => prev > level,
        &Direction::Decreasing => prev < level,
        &Direction::Undetermined => false,
    }
}

fn exceeds_range(prev: &i32, level: &i32, min: i32, max: i32) -> bool {
    (prev - *level).abs() < min || (prev - *level).abs() > max
}

#[cfg(test)]
mod test {
    use crate::{get_safe_report_qty, Report, SafetyLevel};

    #[test]
    fn report_one() {
        let report = Report::from("7 6 4 2 1");
        let mut it = report.levels.into_iter();
        assert_eq!(Some(7), it.next());
        assert_eq!(Some(6), it.next());
        assert_eq!(Some(4), it.next());
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(1), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn report_two() {
        let report = Report::from("1 2 7 8 9");
        let mut it = report.levels.into_iter();
        assert_eq!(Some(1), it.next());
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(7), it.next());
        assert_eq!(Some(8), it.next());
        assert_eq!(Some(9), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn report_is_safe() {
        let report = Report::from("7 6 4 2 1");
        let status = SafetyLevel::from(report, true);
        assert_eq!(true, status.safe());
    }

    #[test]
    fn gap_bigger_than_3_unsafe() {
        let report = Report::from("1 2 7 8 9");
        let status = SafetyLevel::from(report, true);
        assert_eq!(false, status.safe());
    }

    #[test]
    fn second_gap_bigger_than_3_unsafe() {
        let report = Report::from("9 7 6 2 1");
        let status = SafetyLevel::from(report, true);
        assert_eq!(false, status.safe());
    }

    #[test]
    fn change_in_direction_safe_w_level_removed() {
        let report = Report::from("1 3 2 4 5");
        let status = SafetyLevel::from(report, true);
        assert_eq!(true, status.safe());
    }

    #[test]
    fn repeated_level_safe_w_level_removed() {
        let report = Report::from("8 6 4 4 1");
        let status = SafetyLevel::from(report, true);
        assert_eq!(true, status.safe());
    }

    #[test]
    fn second_safe_report_is_safe() {
        let report = Report::from("1 3 6 7 9");
        let status = SafetyLevel::from(report, true);
        assert_eq!(true, status.safe());
    }

    #[test]
    fn correct_safe_report_qty() {
        let qty = get_safe_report_qty(String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        ));
        assert_eq!(4, qty);
    }
}