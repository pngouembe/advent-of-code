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

fn is_game_valid(game_data: Vec<Vec<(u32, String)>>) -> bool {
    game_data.iter().all(|draw| is_draw_valid(draw))
}

fn is_draw_valid(draw: &[(u32, String)]) -> bool {
    const MAX_GREEN_COUNT: u32 = 13;
    const MAX_RED_COUNT: u32 = 12;
    const MAX_BLUE_COUNT: u32 = 14;

    draw.iter().all(|cubes_data| match cubes_data {
        (count, color) if color == "green" => MAX_GREEN_COUNT >= *count,
        (count, color) if color == "red" => MAX_RED_COUNT >= *count,
        (count, color) if color == "blue" => MAX_BLUE_COUNT >= *count,
        (count, color) => {
            println!("invalid draw: {} {} cubes", count, color);
            false
        }
    })
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines();
    let valid_games = lines.filter_map(|line| {
        let (game_id, game_data) = parse_game_data(line);

        let game_id = dbg!(game_id);

        match dbg!(is_game_valid(game_data)) {
            true => Some(game_id),
            false => None,
        }
    });
    let result = valid_games.sum::<u32>();
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
