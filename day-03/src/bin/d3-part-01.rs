// AoC Day-03 Part01
//

use grid::*;
use regex::Regex;

#[derive(Debug)]
struct Input<'a>(Grid<&'a str>);

// Note: Rust complained about lifetime's so I had to include them. It looks
// like structs that use pointer data types require lifetime specifications.
impl Input<'_> {
    // Check symbol neighbors.
    fn has_symbol_neighbor(&self, indices: &Vec<(usize, usize)>) -> bool {
        // List of symbol to match on.
        let pattern = [
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '`', '~', '[', ']', '{', '}', '-',
            '_', '+', '|', '\\', '\"', '\'', '?', ',', ',', '>', '/', '=',
        ];

        for (x, y) in indices {
            // Check top-left neighbor. [x-1, y+1]
            match &self.0.get(x.wrapping_sub(1), y + 1) {
                Some(value) => {
                    if value.find(pattern).is_some() {
                        return true;
                    }
                }
                None => (),
            }

            // Check top neighbor. [x, y+1]
            match &self.0.get(*x, y + 1) {
                Some(value) => {
                    if value.find(pattern).is_some() {
                        return true;
                    }
                }
                None => (),
            }

            // Check top-right neighbor. [x+1, y+1]
            match &self.0.get(x + 1, y + 1) {
                Some(value) => {
                    if value.find(pattern).is_some() {
                        return true;
                    }
                }
                None => (),
            }

            // Check right neighbor. [x+1, y]
            match &self.0.get(x + 1, *y) {
                Some(value) => {
                    if value.find(pattern).is_some() {
                        return true;
                    }
                }
                None => (),
            }

            // Check bottom-right neighbor. [x+1, y-1]
            match &self.0.get(x + 1, y.wrapping_sub(1)) {
                Some(value) => {
                    if value.find(pattern).is_some() {
                        return true;
                    }
                }
                None => (),
            }

            // Check bottom neighbor. [x, y-1]
            match &self.0.get(*x, y.wrapping_sub(1)) {
                Some(value) => {
                    if value.find(pattern).is_some() {
                        return true;
                    }
                }
                None => (),
            }

            // Check bottom-left neighbor. [x-1, y-1]
            match &self.0.get(x.wrapping_sub(1), y.wrapping_sub(1)) {
                Some(value) => {
                    if value.find(pattern).is_some() {
                        return true;
                    }
                }
                None => (),
            }

            // Check left neighbor. [x-1, y]
            match &self.0.get(x.wrapping_sub(1), *y) {
                Some(value) => {
                    if value.find(pattern).is_some() {
                        return true;
                    }
                }
                None => (),
            }
        }

        return false;
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
    // Sum: the final required output.
    let mut sum: u32 = 0;

    // Accumulate the numbers currently being parsed.
    let mut current: String = "".to_string();

    // Track the coords of the numbers so we can loop up neighbors.
    let mut position_stack: Vec<(usize, usize)> = Vec::new();

    let parsed_input = parse(input);

    //Regex pattern.
    let symbol_pattern = Regex::new(r"\W").unwrap();

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

            // Check if symbol neighbors exist for current numbers.
            if parsed_input.has_symbol_neighbor(&position_stack) {
                sum += current.parse::<u32>().unwrap();
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
        assert_eq!(4361, got);
    }

    // Check for symbol between e.g."number{symbol}number" case.
    #[test]
    fn test_parse_edge() {
        let got: u32 = process("467..114..\n...*......\n..35..633.\n......#...\n617*1.....\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
        assert_eq!(4362, got);
    }
}
