// AoC Day-01 Part-01
//

fn process(input: &str) -> i32 {
    let mut sum: i32 = 0;
    for value in input.lines() {
        let mut number: i32 = 0;
        let value_vec: Vec<_> = value.chars().enumerate().collect();

        // Look left to right for first number.
        for digit in value_vec.clone().iter() {
            if digit.1.is_digit(10) {
                // add to the left of the number (10's  place).
                number = 10 * digit.1.to_digit(10).unwrap() as i32;
                break;
            }
        }

        // Look from right to left for the first numebr.
        for digit in value_vec.iter().rev() {
            if digit.1.is_digit(10) {
                // add to the left of the number (10's  place).
                number += digit.1.to_digit(10).unwrap() as i32;
                break;
            }
        }
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
        let got: i32 = process("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(142, got);
    }
}
