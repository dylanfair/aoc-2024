use regex::Regex;
use std::{fs::read_to_string, path::Path};

fn main() {
    part_one("src/day03/test.txt");
    part_two("src/day03/actual.txt");
}

fn part_one<P>(input: P)
where
    P: AsRef<Path>,
{
    let input_text = read_to_string(input).unwrap();

    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let answer: i64 = re
        .captures_iter(&input_text)
        .map(|caps| {
            let one = caps[1].parse::<i64>().unwrap();
            let two = caps[2].parse::<i64>().unwrap();
            one * two
        })
        .collect::<Vec<i64>>()
        .iter()
        .sum();

    println!("{}", answer);
}

fn part_two<P>(input: P)
where
    P: AsRef<Path>,
{
    let input_text = read_to_string(input).unwrap();

    let mul_re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let dos = input_text.split("do()");

    let mut answer = 0;
    for doer in dos {
        let mut donts = doer.split("don't()");
        let valids = donts.next().unwrap();

        let valid_mul: i64 = mul_re
            .captures_iter(valids)
            .map(|caps| {
                let one = caps[1].parse::<i64>().unwrap();
                let two = caps[2].parse::<i64>().unwrap();
                one * two
            })
            .collect::<Vec<i64>>()
            .iter()
            .sum();

        answer += valid_mul;
    }

    println!("{}", answer);
}
