use regex::Regex;

use crate::util::read_lines;

pub fn solve() {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    if let Ok(lines) = read_lines("day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calibration) = line {
                println!("{}", calibration);
                part1 += extract_calibration_value(&calibration);
                part2 += process_calibration_with_words(&calibration);
            }
        }
    }
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn extract_calibration_value(input: &str) -> i32 {
    let mut result: Vec<char> = Vec::new();
    for c in input.chars() {
        if c.is_numeric() {
            result.push(c);
            break;
        }
    }

    for c in input.chars().rev() {
        if c.is_numeric() {
            result.push(c);
            break;
        }
    }
    // println!("{}", result.join(""));
    format!("{}{}", result.get(0).unwrap(), result.get(1).unwrap())
        .parse::<i32>()
        .unwrap()
}

fn process_calibration_with_words(input: &str) -> i32 {
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    // let results: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    // let mut results: Vec<&str> = Vec::new();
    let results: Vec<String> = input
        .char_indices()
        .filter_map(|(i, _)| re.captures(&input[i..]))
        .map(|cap| cap[1].to_string())
        .collect();

    let processed = format!("{}{}", results.first().unwrap(), results.last().unwrap())
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");

    extract_calibration_value(processed.as_str())
}

#[cfg(test)]
mod tests {
    use crate::day1::{extract_calibration_value, process_calibration_with_words};

    #[test]
    fn part1_demo_input() {
        assert_eq!(extract_calibration_value("1abc2"), 12);
        assert_eq!(extract_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(extract_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(extract_calibration_value("treb7uchet"), 77);
        assert_eq!(extract_calibration_value("sevenhcgr6ninefour"), 66);
    }

    #[test]
    fn part2_demo_input() {
        assert_eq!(process_calibration_with_words("two1nine"), 29);
        assert_eq!(process_calibration_with_words("eightwothree"), 83);
        assert_eq!(process_calibration_with_words("abcone2threexyz"), 13);
        assert_eq!(process_calibration_with_words("xtwone3four"), 24);
        assert_eq!(process_calibration_with_words("4nineeightseven2"), 42);
        assert_eq!(process_calibration_with_words("zoneight234"), 14);
        assert_eq!(process_calibration_with_words("7pqrstsixteen"), 76);
        assert_eq!(
            process_calibration_with_words("trknlxnv43zxlrqjtwonect"),
            41
        );
    }
}
