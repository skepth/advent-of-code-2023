// AoC Day-03 Part01
//

use grid::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Input<'a>(Grid<&'a str>);

// Note: Rust complained about lifetime's so I had to include them. It looks like structs that use pointer data types require lifetime specifications.
impl Input<'_> {
    // Check if '*' is a neighbor and return it's indices (x, y) if exists.
    fn has_star_neighbor(&self, indices: &Vec<(usize, usize)>) -> Option<(usize, usize)> {
        // List of symbol to match on.

        for (x, y) in indices {
            // Check top-left neighbor. [x-1, y+1]
            match &self.0.get(x.wrapping_sub(1), y + 1) {
                Some(value) => {
                    if value.find("*").is_some() {
                        return Some((x - 1, y + 1));
                    }
                }
                None => (),
            }

            // Check top neighbor. [x, y+1]
            match &self.0.get(*x, y + 1) {
                Some(value) => {
                    if value.find("*").is_some() {
                        return Some((*x, y + 1));
                    }
                }
                None => (),
            }

            // Check top-right neighbor. [x+1, y+1]
            match &self.0.get(x + 1, y + 1) {
                Some(value) => {
                    if value.find("*").is_some() {
                        return Some((x + 1, y + 1));
                    }
                }
                None => (),
            }

            // Check right neighbor. [x+1, y]
            match &self.0.get(x + 1, *y) {
                Some(value) => {
                    if value.find("*").is_some() {
                        return Some((x + 1, *y));
                    }
                }
                None => (),
            }

            // Check bottom-right neighbor. [x+1, y-1]
            match &self.0.get(x + 1, y.wrapping_sub(1)) {
                Some(value) => {
                    if value.find("*").is_some() {
                        return Some((x + 1, y - 1));
                    }
                }
                None => (),
            }

            // Check bottom neighbor. [x, y-1]
            match &self.0.get(*x, y.wrapping_sub(1)) {
                Some(value) => {
                    if value.find("*").is_some() {
                        return Some((*x, y - 1));
                    }
                }
                None => (),
            }

            // Check bottom-left neighbor. [x-1, y-1]
            match &self.0.get(x.wrapping_sub(1), y.wrapping_sub(1)) {
                Some(value) => {
                    if value.find("*").is_some() {
                        return Some((x - 1, y - 1));
                    }
                }
                None => (),
            }

            // Check left neighbor. [x-1, y]
            match &self.0.get(x.wrapping_sub(1), *y) {
                Some(value) => {
                    if value.find("*").is_some() {
                        return Some((x - 1, *y));
                    }
                }
                None => (),
            }
        }

        return None;
    }
}

// Parse the input into data structs.
fn parse(input: &str) -> Input {
    let mut result: Input = Input(grid![]);

    for line in input.lines() {
        result
            .0
            .push_row(line.split_terminator("").skip(1).collect::<Vec<&str>>());
    }
    return result;
}

// Process the input.
fn process(input: &str) -> u32 {
    //Regex pattern.
    let symbol_pattern = Regex::new(r"\W").unwrap();

    // Sum: the final required output.
    let mut sum: u32 = 0;

    // Accumulate the numbers currently being parsed.
    let mut current: String = "".to_string();

    // Track the coords of the numbers so we can loop up neighbors.
    let mut position_stack: Vec<(usize, usize)> = Vec::new();

    // HashMap used to track pair of numbers adjacent to '*'.
    let mut ratio_map: HashMap<(usize, usize), u32> = HashMap::new();

    let parsed_input = parse(input);

    for ((x, y), value) in parsed_input.0.indexed_iter() {
        // If the value is a number.
        if value.parse::<u32>().is_ok() {
            current = current + value.to_owned();
            position_stack.push((x, y))
        } else if symbol_pattern.is_match(value) {
            // If the current string is empty, continue.
            if current == "".to_string() {
                continue;
            }

            // Check if '*' neighbors exist for current numbers and returns the
            // location of the star neighbor.
            match parsed_input.has_star_neighbor(&position_stack) {
                Some(coord) => {
                    match ratio_map.get(&coord) {
                        // If the '*' is in ratio_map, that means we just hit a
                        // pair of numbers for the gear ratio.
                        Some(previous) => {
                            sum += current.parse::<u32>().unwrap() * previous;
                        }
                        // If the '*' is not found, then this is the first number
                        // adjacent to the '*'. So we save it for the future.
                        None => {
                            ratio_map.insert(coord, current.parse::<u32>().unwrap());
                        }
                    }
                }
                None => (),
            }

            // Reset string to empty.
            current = "".to_string();

            // Reset coords to empty.
            position_stack.clear();
        }
    }
    return sum;
}

fn main() {
    let input = include_str!("input.txt");
    println!("Result: {}", process(input));
}

// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let got: u32 = process("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
        assert_eq!(467835, got);
    }
}
