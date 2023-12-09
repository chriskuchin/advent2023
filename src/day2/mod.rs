use crate::util::read_lines;
use regex::Regex;

pub fn solve() {
    if let Ok(lines) = read_lines("day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        let part1 = lines
            .map(|line| line.unwrap())
            .map(|input| {
                // println!("{}", input);
                process_game_part1(input.as_str(), 12, 14, 13)
            })
            .filter(|v| v.1)
            .fold(0, |acc: i32, i| acc + i.0);
        println!("Part 1: {}", part1);
    }
    if let Ok(lines) = read_lines("day2.txt") {
        let part2 = lines
            .map(|line| get_minimum_viable_game(line.unwrap().as_str()))
            .map(|(blue, red, green)| blue * red * green)
            .reduce(|acc, val| acc + val).unwrap();
        println!("Part 2: {}", part2);
    }
}

fn process_game_part1(input: &str, max_red: i32, max_blue: i32, max_green: i32) -> (i32, bool) {
    let header = input.split(":").collect::<Vec<&str>>();

    let game_id = header
        .first()
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let (blue_total, red_total, green_total) = get_minimum_viable_game(header.last().unwrap());
    (
        game_id,
        blue_total <= max_blue && red_total <= max_red && green_total <= max_green,
    )
}

fn get_minimum_viable_game(game: &str) -> (i32, i32, i32) {
    let re = Regex::new(r"(\d+ (?:blue|red|green))+").unwrap();
    re.find_iter(game)
        .fold((0, 0, 0), |acc: (i32, i32, i32), i| {
            if i.as_str().to_string().contains("blue") {
                let blue = i
                    .as_str()
                    .to_string()
                    .replace("blue", "")
                    .trim()
                    .parse::<i32>()
                    .unwrap();
                return (std::cmp::max(acc.0, blue), acc.1, acc.2);
            } else if i.as_str().to_string().contains("red") {
                let red = i
                    .as_str()
                    .to_string()
                    .replace("red", "")
                    .trim()
                    .parse::<i32>()
                    .unwrap();
                return (acc.0, std::cmp::max(acc.1, red), acc.2);
            } else if i.as_str().to_string().contains("green") {
                let green = i
                    .as_str()
                    .to_string()
                    .replace("green", "")
                    .trim()
                    .parse::<i32>()
                    .unwrap();
                return (acc.0, acc.1, std::cmp::max(acc.2, green));
            }
            acc
        })
}
#[cfg(test)]
mod tests {
    use crate::day2::get_minimum_viable_game;

    use super::process_game_part1;

    #[test]
    fn part1_demo_input() {
        assert_eq!(
            process_game_part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                12,
                14,
                13
            ),
            (1, true)
        );
        assert_eq!(
            process_game_part1(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                12,
                14,
                13
            ),
            (2, true)
        );
        assert_eq!(
            process_game_part1(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                12,
                14,
                13
            ),
            (3, false)
        );
        assert_eq!(
            process_game_part1(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                12,
                14,
                13
            ),
            (4, false)
        );
        assert_eq!(
            process_game_part1(
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                12,
                14,
                13
            ),
            (5, true)
        );
    }

    #[test]
    fn minimum_viable_game() {
        assert_eq!(
            get_minimum_viable_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            (6, 4, 2)
        );
        assert_eq!(
            get_minimum_viable_game(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
            ),
            (4, 1, 3)
        );
        assert_eq!(
            get_minimum_viable_game(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            (6, 20, 13)
        );
        assert_eq!(
            get_minimum_viable_game(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            (15, 14, 3)
        );
        assert_eq!(
            get_minimum_viable_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            (2, 6, 3)
        );
    }

    #[test]
    fn part2_demo_input() {
        // assert_eq!(process_calibration_with_words("two1nine"), 29);
    }
}
