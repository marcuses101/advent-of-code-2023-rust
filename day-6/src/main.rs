use std::fs;

#[derive(Debug)]
struct Race {
    duration: i32,
    distance_to_beat: i32,
}

fn extract_numbers(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter_map(|chars| chars.parse().ok())
        .collect()
}

#[derive(Debug)]
struct Races(Vec<Race>);

impl TryFrom<&str> for Races {
    type Error = (); // You can use a custom error type here

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let durations_line = lines.next().unwrap();
        let distances_line = lines.next().unwrap();

        let durations = extract_numbers(durations_line);
        let distances = extract_numbers(distances_line);
        let races: Vec<Race> = durations
            .into_iter()
            .zip(distances.into_iter())
            .map(|(duration, distance)| Race {
                duration,
                distance_to_beat: distance,
            })
            .collect();
        Ok(Self(races))
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let races: Races = input.as_str().try_into().unwrap();
    println!("{:?}", races);
    let posibility_count_product: i32 = races
        .0
        .iter()
        .map(|race| {
            (1..race.duration).fold(0, |acc, time| {
                let distance = time * (race.duration - time);
                if distance > race.distance_to_beat {
                    return acc + 1;
                }
                acc
            })
        })
        .fold(1, |acc, posibility_count| acc * posibility_count);
    println!("ANSWER: {:?}", posibility_count_product);
}
