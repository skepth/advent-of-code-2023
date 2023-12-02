// AoC Day-01 Part-02
//
use std::collections::HashMap;

fn process(input: &str) -> i32 {
    let map: HashMap<&str, i32> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum: i32 = 0;

    // Using matches() method to parse the lines.
    for line in input.lines() {
        // Note: Need to trim the newline in the or else the code gets borked.
        // e.g. the test case accounts for this.
        if line == "" {
            continue;
        }

        let mut number: i32;
        let mut ordered_matches: Vec<(usize, &str)> = Vec::new();

        // Search the line for word matches.
        for (key, _) in &map {
            ordered_matches.append(&mut line.match_indices(key).collect::<Vec<(usize, &str)>>());
        }

        // Search the line for number matches.
        ordered_matches.append(
            &mut line
                .match_indices(char::is_numeric)
                .collect::<Vec<(usize, &str)>>(),
        );

        // Sort the matches by indices.
        ordered_matches.sort_by(|a, b| a.0.cmp(&b.0));

        // Construct number using first and last matches.
        match map.get_key_value(ordered_matches.first().unwrap().1) {
            Some((_, &v)) => number = 10 * v,
            None => {
                number = 10 * ordered_matches.first().unwrap().1.parse::<i32>().unwrap();
            }
        }

        match map.get_key_value(ordered_matches.last().unwrap().1) {
            Some((_, &v)) => number += v,
            None => {
                number += ordered_matches.last().unwrap().1.parse::<i32>().unwrap();
            }
        }

        // Sum the numbers.
        sum += number;
    }

    return sum;
}

fn main() {
    let input: &str = include_str!("input.txt");
    println!("Result: {}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let got: i32 = process("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n\n");
        assert_eq!(281, got);
    }
}
