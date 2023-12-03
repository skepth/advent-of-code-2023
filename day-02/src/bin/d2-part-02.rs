// AoC Day-02 Part-02
//
use enum_map::{Enum, EnumMap};

#[derive(Debug, Enum, Clone, Copy)]
enum COLORS {
    RED,
    GREEN,
    BLUE,
}

// Note: &str use within the HashMap threw a lifetime needed error.
#[derive(Debug, Default)]
struct Game {
    id: u32,
    turns: Vec<EnumMap<COLORS, u32>>,
}

impl Game {
    // Get power of cubes in a minimum viable game.
    fn get_power_min(&self) -> u32 {
        let mut min_map: EnumMap<COLORS, u32> = EnumMap::default();

        for game in &self.turns {
            if game[COLORS::RED] > min_map[COLORS::RED] {
                min_map[COLORS::RED] = game[COLORS::RED];
            }

            if game[COLORS::GREEN] > min_map[COLORS::GREEN] {
                min_map[COLORS::GREEN] = game[COLORS::GREEN];
            }

            if game[COLORS::BLUE] > min_map[COLORS::BLUE] {
                min_map[COLORS::BLUE] = game[COLORS::BLUE];
            }
        }

        let mut result: u32 = 1;
        for value in min_map.values() {
            result *= value;
        }
        return result;
    }
}

// Parses the input into manageable structs.
fn parse(input: &str) -> Vec<Game> {
    let mut result: Vec<Game> = Vec::new();

    for line in input.lines() {
        let mut new_game: Game = Game::default();

        // Split the string at index 7, where game id & colors can be parsed.
        let (game_info, color_info) = line.split_at(8);

        // Extract game id from the string.
        new_game.id = game_info
            .matches(char::is_numeric)
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u32>()
            .unwrap();

        // Extract plays from each game (split string by '; ').
        for play in color_info.split("; ").collect::<Vec<&str>>() {
            let mut color_map: EnumMap<COLORS, u32> = EnumMap::default();

            // Extract cube color count.
            for cube_pick in play.split(", ").collect::<Vec<&str>>() {
                let picks = cube_pick.rsplit(" ").collect::<Vec<&str>>();
                match picks[0] {
                    "red" => color_map[COLORS::RED] = picks[1].parse::<u32>().unwrap(),
                    "green" => color_map[COLORS::GREEN] = picks[1].parse::<u32>().unwrap(),
                    "blue" => color_map[COLORS::BLUE] = picks[1].parse::<u32>().unwrap(),
                    _ => unreachable!(),
                }
            }
            new_game.turns.push(color_map);
        }
        result.push(new_game);
    }
    return result;
}

fn process(input: &str) -> u32 {
    let parsed_input = parse(input);
    let mut sum: u32 = 0;

    for game in parsed_input {
        sum += game.get_power_min();
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
        let got = process("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(2286, got);
    }
}
