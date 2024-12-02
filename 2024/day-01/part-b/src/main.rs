use std::{fs, path::Path};

fn main() {
    let path = Path::new("./resources/input.txt");
    let contents = fs::read_to_string(path).expect("Could not find the input file!");
    let mut nums = get_integer_lists(contents);
    let distance = get_total_distance(&mut nums);
    let similarity_score = get_similarity_score(&nums);
    println!("total distance: {}", distance);
    println!("similarity score: {}", similarity_score);
}

pub fn get_similarity_score(nums: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut similarity_score = 0;
    for num in &nums.0 {
        let matches = &nums.1.iter().filter(|n| **n == *num).collect::<Vec<_>>();
        let similarity = matches.iter().count() as i32 * num;
        similarity_score += similarity;
    }
    similarity_score
}

pub fn get_total_distance(lists: &mut (Vec<i32>, Vec<i32>)) -> i32 {
    let lhs_nums = &lists.0;
    let rhs_nums = &mut lists.1;

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
    fn test_case_1_total_distance() {
        let mut nums = get_integer_lists(String::from(
            "3   4
        4   3
        2   5
        1   3
        3   9
        3   3",
        ));
        let foo = get_total_distance(&mut nums);
        assert_eq!(11, foo);
    }

    #[test]
    fn test_case_1_similarity_score() {
        let nums = get_integer_lists(String::from(
            "3   4
        4   3
        2   5
        1   3
        3   9
        3   3",
        ));
        let score = get_similarity_score(&nums);
        assert_eq!(31, score);
    }
}
