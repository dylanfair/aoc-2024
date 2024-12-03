use std::{fs::read_to_string, path::Path};

fn main() {
    part_one("src/day02/test.txt");
    part_two("src/day02/actual.txt");
}

fn part_one<P>(input: P)
where
    P: AsRef<Path>,
{
    let text = read_to_string(input).unwrap();
    let mut safe_reports = 0;
    for line in text.lines() {
        let (report_outcome, _) = parse_line(line);

        if report_outcome {
            safe_reports += 1;
        }
    }
    println!("The number of safe reports is: {}", safe_reports);
}

fn parse_line(line: &str) -> (bool, u64) {
    let mut text_numbers = line.split_whitespace();
    // let mut numbers = vec![];

    let first_number = text_numbers.next().unwrap().parse::<i64>().unwrap();
    let mut prior_number = text_numbers.next().unwrap().parse::<i64>().unwrap();

    let increasing = match prior_number.cmp(&first_number) {
        std::cmp::Ordering::Greater => true,
        std::cmp::Ordering::Less => false,
        std::cmp::Ordering::Equal => return (false, 0),
    };
    if prior_number.abs_diff(first_number) > 3 {
        return (false, 0);
    }

    let mut position = 1;
    while let Some(number) = text_numbers.next() {
        let parsed_number = number.parse::<i64>().unwrap();

        if !increase_check(prior_number, parsed_number, increasing) {
            return (false, position);
        }
        if !size_check(prior_number, parsed_number) {
            return (false, position);
        }

        prior_number = parsed_number;
        position += 1;
    }

    (true, 0)
}

fn increase_check(prior: i64, current: i64, prior_increase: bool) -> bool {
    let current_increase = match current.cmp(&prior) {
        std::cmp::Ordering::Greater => true,
        std::cmp::Ordering::Less => false,
        std::cmp::Ordering::Equal => return false,
    };

    if current_increase != prior_increase {
        return false;
    }
    true
}

fn size_check(prior: i64, current: i64) -> bool {
    if prior.abs_diff(current) > 3 {
        return false;
    }
    true
}

fn part_two<P>(input: P)
where
    P: AsRef<Path>,
{
    let text = read_to_string(input).unwrap();
    let mut safe_reports = 0;
    for line in text.lines() {
        let (report_outcome, failed_position) = parse_line(line);

        if !report_outcome {
            if parse_line_two(line, failed_position) {
                safe_reports += 1;
            } else {
                println!("{}", line);
            }
        } else {
            safe_reports += 1;
        }
    }
    println!("The number of safe reports is: {}", safe_reports);
}

fn parse_line_two(line: &str, failed_position: u64) -> bool {
    let mut text_numbers: Vec<u64> = line
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut second_numbers = text_numbers.clone();

    text_numbers.remove(failed_position as usize);
    if !check_report(text_numbers) {
        second_numbers.remove(second_numbers.len() - 1);
        if !check_report(second_numbers) {
            return false;
        } else {
            return true;
        }
    } else {
        return true;
    }
}

fn check_report(numbers: Vec<u64>) -> bool {
    let first_number = numbers[0];
    let mut prior_number = numbers[1];

    let increasing = match prior_number.cmp(&first_number) {
        std::cmp::Ordering::Greater => true,
        std::cmp::Ordering::Less => false,
        std::cmp::Ordering::Equal => return false,
    };
    if prior_number.abs_diff(first_number) > 3 {
        return false;
    }

    for number in numbers.iter().skip(2) {
        let parsed_number = *number;
        if !increase_check(prior_number as i64, parsed_number as i64, increasing) {
            return false;
        }
        if !size_check(prior_number as i64, parsed_number as i64) {
            return false;
        }
        prior_number = parsed_number;
    }

    true
}
