use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_one("test.txt");
    part_two("actual.txt");
}

fn part_one<P>(path: P)
where
    P: AsRef<Path>,
{
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let unwrapped = line.unwrap();
        let mut numbers = unwrapped.split_whitespace();
        let left_number = numbers.next().unwrap().parse::<i64>().unwrap();
        left.push(left_number);

        let right_number = numbers.next().unwrap().parse::<i64>().unwrap();
        right.push(right_number);
    }

    left.sort();
    right.sort();

    let mut total_difference = 0;
    for (num1, num2) in left.iter().zip(right.iter()) {
        total_difference += num1.abs_diff(*num2);
    }

    println!("The total difference is {}", total_difference);
}

fn part_two<P>(path: P)
where
    P: AsRef<Path>,
{
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut left = HashMap::new();
    let mut right = HashMap::new();

    for line in lines {
        let unwrapped = line.unwrap();
        let mut numbers = unwrapped.split_whitespace();
        let left_number = numbers.next().unwrap().parse::<i64>().unwrap();
        left.entry(left_number)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        let right_number = numbers.next().unwrap().parse::<i64>().unwrap();
        right
            .entry(right_number)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut similarity_score = 0;
    for (number, left_count) in &left {
        if let Some(right_count) = right.get(number) {
            similarity_score += (number * right_count) * left_count;
        }
    }

    println!("{:?}", similarity_score);
}
