use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = "src/bin/input1.txt";
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let file = File::open(input).expect("File was not found.");
    let reader = BufReader::new(file);
    let mut result: i64 = 0;

    let numbers: Vec<i64> = reader
    .lines()
    .map(|line| line.unwrap().parse::<i64>().unwrap())
    .collect();
    
    'outer: for number1 in 0..numbers.len()-1 {
        for number2 in number1+1..numbers.len() {
            if numbers[number1] + numbers[number2] == 2020
            {
                result = numbers[number1] * numbers[number2];
                break 'outer;
            }
        }
    }

    return result.to_string();
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let result = part1("1721
        979
        366
        299
        675
        1456");
        assert_eq!(result, "2020".to_string());
    }
}
