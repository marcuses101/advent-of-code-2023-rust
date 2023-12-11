use std::{env, fs, process, str::FromStr};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn invert(self: &Self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}
use Direction::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseTileTypeError;

impl FromStr for TileType {
    type Err = ParseTileTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(TileType::Pipe(North, South)),
            "-" => Ok(TileType::Pipe(East, West)),
            "L" => Ok(TileType::Pipe(North, East)),
            "J" => Ok(TileType::Pipe(North, West)),
            "7" => Ok(TileType::Pipe(South, West)),
            "F" => Ok(TileType::Pipe(East, South)),
            "." => Ok(TileType::Ground),
            "S" => Ok(TileType::Start),
            _ => Err(ParseTileTypeError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Tile {
    x: usize,
    y: usize,
    tile_type: TileType,
}

impl Tile {
    fn new(x: usize, y: usize, char: &str) -> Result<Self, ParseTileTypeError> {
        let tile_type: TileType = char.parse()?;
        Ok(Self { x, y, tile_type })
    }
}

#[derive(Debug)]
struct Board(Vec<Vec<Tile>>);

impl Board {
    fn get_start_tile(self: &Self) -> Option<&Tile> {
        let mut start: Option<&Tile> = None;
        for tile_line in self.0.iter() {
            for tile in tile_line {
                if tile.tile_type == TileType::Start {
                    start = Some(&tile);
                }
            }
        }
        start
    }
    fn get_tile_at_location(self: &Self, x: usize, y: usize) -> Option<&Tile> {
        let row = self.0.get(y)?;
        row.get(x)
    }

    fn move_in_direction(self: &Self, current: &Tile, direction: Direction) -> Option<&Tile> {
        let next: Option<(usize, usize)> = match direction {
            North => {
                if current.y == 0 {
                    None
                } else {
                    Some((current.x, current.y - 1))
                }
            }
            East => Some((current.x + 1, current.y)),
            South => Some((current.x, current.y + 1)),
            West => {
                if current.x == 0 {
                    None
                } else {
                    Some((current.x - 1, current.y))
                }
            }
        };
        match next {
            Some((x, y)) => self.get_tile_at_location(x, y),
            None => None,
        }
    }
    fn get_next_tile(
        self: &Self,
        current: &Tile,
        previous_direction: Direction,
    ) -> Option<(&Tile, Direction)> {
        dbg!(current, previous_direction);
        let (a_dir, b_dir): (Direction, Direction) = match current.tile_type {
            TileType::Pipe(a, b) => Some((a, b)),
            _ => None,
        }?;
        if a_dir != previous_direction && b_dir != previous_direction {
            return None;
        }
        let next_dir = if a_dir == previous_direction {
            b_dir
        } else {
            a_dir
        };
        let next_tile = self.move_in_direction(current, next_dir)?;
        Some((next_tile, next_dir.invert()))
    }
    fn follow_and_count(self: &Self, current: &Tile, direction: Direction) -> Option<usize> {
        let mut moves = 1;
        let first_tile = self.move_in_direction(current, direction)?;
        let mut current_tile: Option<(&Tile, Direction)> = Some((first_tile, direction.invert()));

        while let Some((tile, previous_direction)) = current_tile {
            moves += 1;
            let next_tile_option = self.get_next_tile(tile, previous_direction);
            if let Some((new_tile, _)) = next_tile_option {
                if let TileType::Start = new_tile.tile_type {
                    return Some(moves);
                }
            }
            current_tile = next_tile_option
        }
        None
    }
}

impl FromStr for Board {
    type Err = ParseTileTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: Vec<Vec<Tile>> = s
            .trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(index, char)| Tile::new(index, y, char.to_string().as_str()).ok())
                    .collect()
            })
            .collect();
        Ok(Self(board))
    }
}

fn part_one(input: String) -> Result<usize, ()> {
    let board: Board = input.parse().unwrap();
    let start: &Tile = board.get_start_tile().ok_or(())?;
    for direction in [East] {
        let moves = board.follow_and_count(start, direction);
        if let Some(num) = moves {
            return Ok(num / 2);
        }
    }
    Err(())
}

fn part_two(input: String) -> usize {
    todo!();
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
.....
.S-7.
.|.|.
.L-J.
.....
";
    const TEST_INPUT_TWO: &str = r"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
    #[test]
    fn test_part_one() {
        // Arrange

        // Act
        let result_one = part_one(TEST_INPUT_ONE.trim().to_string()).unwrap();

        // Assert
        assert_eq!(result_one, 4);

        let result_two = part_one(TEST_INPUT_TWO.trim().to_string()).unwrap();
        assert_eq!(result_two, 8);
    }

    /*
    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT.to_string());
        assert_eq!(result, 2);
    }
    */
}
