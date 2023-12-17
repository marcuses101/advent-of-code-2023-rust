use std::{env, fs, process};

#[derive(Debug, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}

fn expand_vertical(input: String) -> String {
    let lines = input.lines();
    lines
        .map(|line| {
            if line.contains("#") {
                line.to_string()
            } else {
                line.to_string() + "\n" + line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_empty_row_indices(input: &str) -> Vec<usize> {
    let lines = input.lines();
    lines
        .enumerate()
        .filter_map(|(index, line)| {
            if line.contains("#") {
                None
            } else {
                Some(index)
            }
        })
        .collect()
}

fn get_empty_column_indices(input: &str) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let line_length = lines.get(0).unwrap().len();
    let line_count = lines.len();
    for x in 0..line_length {
        let mut is_empty = true;
        for y in 0..line_count {
            let char = lines[y].chars().nth(x).unwrap();
            if char == '#' {
                is_empty = false;
            }
        }
        if is_empty {
            indices.push(x);
        }
    }
    indices
}

fn expand_horizontal(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let indices = get_empty_column_indices(input.as_str());
    lines
        .into_iter()
        .map(|line| {
            let new_line =
                line.chars()
                    .enumerate()
                    .fold(String::new(), |mut acc, (index, char)| {
                        acc.push(char);
                        if indices.contains(&index) {
                            acc.push(char);
                        }
                        acc
                    });
            new_line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn expand(input: String) -> String {
    expand_vertical(expand_horizontal(input))
}

fn get_locations(input: &str) -> Vec<Location> {
    let locations: Vec<Location> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if char == '#' {
                    Some(Location { x, y })
                } else {
                    None
                }
            })
        })
        .collect();
    locations
}

fn part_one(input: String) -> Result<usize, ()> {
    let expanded_universe = expand(input);
    let locations = get_locations(expanded_universe.as_str());
    let distance_total: usize = locations
        .iter()
        .enumerate()
        .map(|(index, current_location)| {
            let mut differences = 0;
            for other_location_index in index + 1..locations.len() {
                let other_location = locations.get(other_location_index).unwrap();
                differences += current_location.x.abs_diff(other_location.x);
                differences += current_location.y.abs_diff(other_location.y);
            }
            differences
        })
        .sum();
    Ok(distance_total)
}

fn expand_location(
    initial_location: &Location,
    empty_row_indices: &Vec<usize>,
    empty_column_indices: &Vec<usize>,
    expansion_factor: usize,
) -> Location {
    let column_expansions: usize = empty_column_indices
        .iter()
        .filter(|i| **i < initial_location.x)
        .count();
    let row_expansions: usize = empty_row_indices
        .iter()
        .filter(|i| **i < initial_location.y)
        .count();
    let output = Location {
        x: initial_location.x + column_expansions * expansion_factor - column_expansions,
        y: initial_location.y + row_expansions * expansion_factor - row_expansions,
    };
    output
}

fn calculate_distances_sum(input: String, factor: usize) -> usize {
    let empty_row_indices = get_empty_row_indices(input.trim());
    let empty_column_indices = get_empty_column_indices(input.trim());
    let locations = get_locations(input.as_str());
    let expanded_locations: Vec<Location> = locations
        .iter()
        .map(|loc| expand_location(loc, &empty_row_indices, &empty_column_indices, factor))
        .collect();
    let distance_total: usize = expanded_locations
        .iter()
        .enumerate()
        .map(|(index, current_location)| {
            let mut differences = 0;
            for other_location_index in index + 1..expanded_locations.len() {
                let other_location = expanded_locations.get(other_location_index).unwrap();
                differences += current_location.x.abs_diff(other_location.x);
                differences += current_location.y.abs_diff(other_location.y);
            }
            differences
        })
        .sum();
    distance_total
}

fn part_two(input: String) -> usize {
    calculate_distances_sum(input, 1_000_000)
}

fn main() {
    let mut args = env::args();
    let part = args.nth(1).unwrap_or("1".into());

    let input = fs::read_to_string("./input.txt").unwrap();
    let answer = match part.as_str() {
        "1" => part_one(input).unwrap(),
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
    use super::*;

    const TEST_INPUT_ONE: &str = r"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_compare_expand_methods() {
        // Method one
        let expanded_universe = expand(TEST_INPUT_ONE.trim().to_string());
        let expanded_locations_one = get_locations(expanded_universe.as_str());

        // METHOD TWO
        let empty_row_indices = get_empty_row_indices(TEST_INPUT_ONE.trim());
        let empty_column_indices = get_empty_column_indices(TEST_INPUT_ONE.trim());
        let locations = get_locations(TEST_INPUT_ONE.trim());
        let expanded_locations_two: Vec<Location> = locations
            .iter()
            .map(|loc| expand_location(loc, &empty_row_indices, &empty_column_indices, 2))
            .collect();
        assert_eq!(expanded_locations_one, expanded_locations_two);
    }

    #[test]
    fn test_get_empty_row_indices() {
        let result = get_empty_row_indices(TEST_INPUT_ONE.trim());
        let expected = vec![3, 7];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        // Arrange

        // Act
        let result_one = part_one(TEST_INPUT_ONE.trim().to_string()).unwrap();

        // Assert
        assert_eq!(result_one, 374);
    }

    #[test]
    fn test_expand() {
        let result = expand(TEST_INPUT_ONE.trim().to_string());
        let expected = r"
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
"
        .trim();
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_calculate_distances() {
        for (input, output) in [(2, 374), (10, 1030), (100, 8410)] {
            let result = calculate_distances_sum(TEST_INPUT_ONE.trim().to_string(), input);
            assert_eq!(result, output);
        }
    }
}
