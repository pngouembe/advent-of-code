use regex::Regex;
use std::collections::HashMap;

use crate::custom_error::AocError;

#[derive(Debug)]
struct Number {
    pub x: u32,
    pub y: u32,
    pub value: String,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut result = 0;
    let (mut symbols, numbers) = parse_input(input);

    for number in numbers.iter() {
        let search_coordonates: Vec<(u32, u32)> = get_search_coordonates(number);

        for (x, y) in search_coordonates.iter() {
            if let Some(parts) = symbols.get_mut(&(*x, *y)) {
                println!("number {} is a part number", number.value);
                parts.push(number.value.clone());
            }
        }
    }

    result = symbols
        .iter()
        .filter(|(_, parts)| parts.len() == 2)
        .map(|(_, parts)| parts[0].parse::<u32>().unwrap() * parts[1].parse::<u32>().unwrap())
        .sum();

    Ok(result.to_string())
}

fn parse_input(input: &str) -> (HashMap<(u32, u32), Vec<String>>, Vec<Number>) {
    let mut symbols = HashMap::new();
    let mut numbers = Vec::new();

    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"[*]").unwrap();

    for (y, line) in input.lines().enumerate() {
        for symbol in symbol_regex.find_iter(line) {
            symbols.insert((symbol.start() as u32, y as u32), vec![]);
        }

        for number in number_regex.find_iter(line) {
            numbers.push(Number {
                x: number.start() as u32,
                y: y as u32,
                value: number.as_str().to_string(),
            });
        }
    }
    (symbols, numbers)
}

fn get_search_coordonates(number: &Number) -> Vec<(u32, u32)> {
    let mut coordinates = Vec::new();

    // Todo: improve this to not search useless coordinates
    for x in number.x.saturating_sub(1)..=(number.x + number.value.len() as u32) {
        for y in number.y.saturating_sub(1)..=number.y + 1 {
            coordinates.push((x, y));
        }
    }
    coordinates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
