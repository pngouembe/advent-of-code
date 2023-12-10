use crate::custom_error::AocError;
use std::cmp;

#[derive(Debug)]
struct SearchArea {
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

fn symbol_in_search_area(search_area: &SearchArea, lines: &[&str]) -> bool {
    let search_area = dbg!(search_area);
    for line_index in search_area.min_y..=search_area.max_y {
        for colomun_index in search_area.min_x..=search_area.max_x {
            let char: char = lines[line_index]
                .chars()
                .nth(colomun_index)
                .expect("Char should exist");

            if !char.is_numeric() && char != '.' {
                return true;
            }
        }
    }

    false
}
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut result = 0;
    let lines: Vec<_> = input.lines().collect();

    for (line_index, line) in lines.iter().enumerate() {
        let mut part_number = String::new();
        let mut previous_char_is_numeric = false;

        let mut search_area = SearchArea {
            min_x: 0,
            max_x: 0,
            min_y: line_index.saturating_sub(1),
            max_y: cmp::min(line_index + 1, lines.len()),
        };

        for (colomn_index, char) in line.chars().enumerate() {
            let mut should_check_number = match (char.is_numeric(), previous_char_is_numeric) {
                // First number found
                (true, false) => {
                    search_area.min_x = colomn_index.saturating_sub(1);
                    part_number.push(char);
                    previous_char_is_numeric = true;
                    false
                }
                // Last number found
                (false, true) => {
                    search_area.max_x = cmp::min(colomn_index, line.len());
                    previous_char_is_numeric = false;
                    true
                }
                // In a number
                (true, true) => {
                    part_number.push(char);
                    false
                }
                // Not in a number
                (false, false) => false,
            };

            if colomn_index == line.len() {
                should_check_number = true;
            }

            if should_check_number && symbol_in_search_area(&search_area, &lines) {
                result += part_number
                    .parse::<u32>()
                    .expect("The part number should be a valid number");
                part_number = String::new();
            }
        }
    }

    Ok(result.to_string())
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
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
