use std::fmt;
use std::{fs, path::Path};

fn main() {
    let path = Path::new("./resources/input.txt");
    let contents = fs::read_to_string(path).expect("Could not find the input file!");
    let foo = get_total_distance(contents);
    println!("total distance: {}", foo);
}

pub fn get_total_distance(input: String) -> i32 {
    let lists = get_integer_lists(input);
    let lhs_nums = lists.0;
    let mut rhs_nums = lists.1;

    let mut total = 0;
    for it in lhs_nums.iter().zip(rhs_nums.iter_mut()) {
        let (lhs, rhs) = it;
        let dif = (*lhs - *rhs).abs();
        total += dif;
    }
    total
}

fn get_integer_lists(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut lhs = Vec::<i32>::new();
    let mut rhs = Vec::<i32>::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        if let Some(left) = split.next() {
            if let Ok(num) = left.parse::<i32>() {
                let _ = &lhs.push(num);
            }
        }
        if let Some(right) = split.next() {
            if let Ok(num) = right.parse::<i32>() {
                let _ = &rhs.push(num);
            }
        }
    }
    lhs.sort();
    rhs.sort();
    (lhs, rhs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let foo = get_total_distance(String::from(
            "3   4
        4   3
        2   5
        1   3
        3   9
        3   3",
        ));
        assert_eq!(11, foo);
    }
}
