use std::{collections::HashMap, env, fs, process, str::FromStr};

struct RestartableIterator<'a, T> {
    items: &'a [T],
    current_index: usize,
}

impl<'a, T> RestartableIterator<'a, T> {
    fn new(items: &'a [T]) -> Self {
        Self {
            items,
            current_index: 0,
        }
    }
}

impl<'a, T> Iterator for RestartableIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return None;
        }

        let result = &self.items[self.current_index];
        self.current_index = (self.current_index + 1) % self.items.len();
        Some(result)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(InstructionParseError),
        }
    }
}

fn process_line(line: &str) -> Option<(String, (String, String))> {
    let mut uppercase_iter = line
        .chars()
        .filter(|c| c.is_uppercase() || c.is_ascii_digit());
    let key: String = uppercase_iter.by_ref().take(3).collect();
    if key.is_empty() {
        return None;
    }
    let left: String = uppercase_iter.by_ref().take(3).collect();
    if left.is_empty() {
        return None;
    }

    let right: String = uppercase_iter.by_ref().take(3).collect();
    if right.is_empty() {
        return None;
    }

    Some((key, (left, right)))
}

fn part_one(
    instructions: Vec<Instruction>,
    location_map: HashMap<String, (String, String)>,
) -> Result<usize, ()> {
    let mut instruction_iter = RestartableIterator::new(&instructions);
    let mut current = "AAA";
    let end = "ZZZ";
    let mut steps: usize = 0;
    while current != end {
        let direction = instruction_iter.next().unwrap();
        let (left, right) = location_map.get(current).unwrap();
        current = match direction {
            Instruction::Left => left,
            Instruction::Right => right,
        };
        steps += 1;
    }
    Ok(steps)
}

fn find_number_of_steps(
    start: String,
    instructions: &[Instruction],
    location_map: &HashMap<String, (String, String)>,
) -> usize {
    let mut instruction_iter = RestartableIterator::new(instructions);
    let mut steps: usize = 0;
    let mut current = start.as_str();
    loop {
        let direction = instruction_iter.next().unwrap();
        let (left, right) = location_map.get(current).unwrap();
        current = match direction {
            Instruction::Left => left,
            Instruction::Right => right,
        };
        steps += 1;
        if current.ends_with('Z') {
            return steps;
        }
    }
}

fn prime_factors(mut n: usize) -> HashMap<usize, usize> {
    let mut factors = HashMap::new();
    let mut i = 2;

    while i * i <= n {
        while n % i == 0 {
            *factors.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1;
    }

    if n > 1 {
        *factors.entry(n).or_insert(0) += 1;
    }

    factors
}

// Function to calculate the lowest common product for a set of numbers
fn lowest_common_product(numbers: &[usize]) -> usize {
    let mut lcm_factors = HashMap::new();

    for &num in numbers {
        let factors = prime_factors(num);

        for (factor, count) in factors {
            let current_count = lcm_factors.entry(factor).or_insert(usize::MAX);
            let min = (*current_count).min(count);
            *current_count = min;
        }
    }

    let mut result = 1;

    for (factor, &count) in &lcm_factors {
        result *= factor.pow(count as u32);
    }

    result
}

fn part_two(
    instructions: Vec<Instruction>,
    location_map: HashMap<String, (String, String)>,
) -> Result<usize, ()> {
    let locations: Vec<String> = location_map
        .keys()
        .filter_map(|key| {
            if key.ends_with('A') {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect();
    if locations.is_empty() {
        return Err(());
    }
    dbg!(&locations);
    let steps_per_start: Vec<usize> = locations
        .iter()
        .map(|location| find_number_of_steps(location.clone(), &instructions, &location_map))
        .collect();
    Ok(lowest_common_product(&steps_per_start))
}

fn main() {
    let mut args = env::args();
    let part = args.nth(1).unwrap_or("1".into());

    let input = fs::read_to_string("./input.txt").unwrap();
    let mut lines = input.lines();
    let raw_instructions = lines.next().unwrap();
    let instructions: Vec<Instruction> = raw_instructions
        .trim_end()
        .split("")
        .filter_map(|c| c.parse().ok())
        .collect();
    let location_map: HashMap<String, (String, String)> =
        lines.skip(1).filter_map(process_line).collect();
    let answer = match part.as_str() {
        "1" => part_one(instructions, location_map),
        "2" => part_two(instructions, location_map),
        _ => {
            eprintln!(
                "invalid argument: {}. Valid arguments are \"1\" or \"2\"",
                part
            );
            process::exit(2);
        }
    }
    .unwrap();
    println!("Answer: {}", answer);
}
