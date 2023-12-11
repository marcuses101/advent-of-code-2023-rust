use std::{env, fs, process};

fn get_changes(input: &[isize]) -> Vec<isize> {
    input.windows(2).map(|win| win[1] - win[0]).collect()
}

fn predict_next_value(input: Vec<isize>) -> isize {
    let next_set = get_changes(&input);
    if next_set.iter().all(|num| *num == 0) {
        return input.last().unwrap().to_owned();
    }
    return input.last().unwrap() + predict_next_value(next_set);
}

fn predict_previous_value(input: Vec<isize>) -> isize {
    let next_set = get_changes(&input);
    if next_set.iter().all(|num| *num == 0) {
        return input.first().unwrap().to_owned();
    }
    return input.first().unwrap() - predict_previous_value(next_set);
}

fn part_one(input: String) -> isize {
    input
        .lines()
        .map(|line| {
            let value_history = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            predict_next_value(value_history)
        })
        .sum()
}

fn part_two(input: String) -> isize {
    input
        .lines()
        .map(|line| {
            let value_history = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            predict_previous_value(value_history)
        })
        .sum()
}

fn main() {
    let mut args = env::args();
    let part = args.nth(1).unwrap_or("1".into());

    let input = fs::read_to_string("./input.txt").unwrap();
    let answer = match part.as_str() {
        "1" => part_one(input),
        "2" => part_two(input),
        _ => {
            eprintln!(
                "invalid argument: {}. Valid arguments are \"1\" or \"2\"",
                part
            );
            process::exit(2);
        }
    };
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const TEST_INPUT: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_get_changes() {
        let input: Vec<isize> = vec![0, 3, 6, 9, 12, 15];
        let expected: Vec<isize> = vec![3, 3, 3, 3, 3];
        let output = get_changes(&input);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_part_one() {
        // Arrange

        // Act
        let result = part_one(TEST_INPUT.to_string());

        // Assert
        assert_eq!(result, 114);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT.to_string());
        assert_eq!(result, 2);
    }
}
