use std::u32;

use regex::Regex;

use crate::custom_error::AocError;

fn parse_game_data(line: &str) -> (u32, Vec<Vec<(u32, String)>>) {
    let game_id_pattern = Regex::new(r"Game (?<game_id>\d+)").unwrap();
    let cube_pattern = Regex::new(r"(?<count>\d+) (?<color>\w+)").unwrap();

    let game_id = game_id_pattern.captures(line).unwrap()["game_id"]
        .parse::<u32>()
        .unwrap();

    let game_data = line
        .split(':')
        .last()
        .expect("line should contain draws descriptions");

    let draws = game_data.split(';');

    let parsed_draws = draws
        .map(|draw| {
            let cubes_data = draw.split(',');

            cubes_data
                .map(|cube_data| {
                    let caps = cube_pattern.captures(cube_data).expect("invalid cube data");

                    (
                        caps["count"].parse::<u32>().unwrap(),
                        caps["color"].to_owned(),
                    )
                })
                .collect()
        })
        .collect();

    (game_id, parsed_draws)
}

fn get_game_minimum_cube_count(game_data: Vec<Vec<(u32, String)>>) -> u32 {
    let mut max_green = 0;
    let mut max_red = 0;
    let mut max_blue = 0;

    for draw in game_data.iter() {
        for (count, color) in draw.iter() {
            match color.as_str() {
                "green" => max_green = std::cmp::max(*count, max_green),
                "red" => max_red = std::cmp::max(*count, max_red),
                "blue" => max_blue = std::cmp::max(*count, max_blue),
                _ => panic!("Coucou"),
            }
        }
    }

    max_green * max_red * max_blue
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines();
    let sets_of_cubes = lines.filter_map(|line| {
        let (_, game_data) = parse_game_data(line);

        Some(get_game_minimum_cube_count(game_data))
    });
    let result = sets_of_cubes.sum::<u32>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
