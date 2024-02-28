use crate::custom_error::AocError;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut global_score = 0;

    for line in _input.lines() {
        let (winning_numbers, numbers) = parse_card(line);
        let score = get_card_score(winning_numbers, numbers);
        global_score += score;
    }
    Ok(global_score.to_string())
}

fn parse_card(card: &str) -> (HashMap<String, bool>, Vec<String>) {
    let mut winning_numbers: HashMap<String, bool> = HashMap::new();
    let mut numbers: Vec<String> = Vec::new();

    let card_content = card.split(':').last().unwrap();
    let mut card_parts = card_content.split('|');

    let winning_numbers_str = card_parts.next().unwrap().trim();
    let numbers_str = card_parts.next().unwrap().trim();

    for number in winning_numbers_str.split_whitespace() {
        winning_numbers.insert(number.to_string(), false);
    }

    for number in numbers_str.split_whitespace() {
        numbers.push(number.to_string());
    }

    (winning_numbers, numbers)
}

fn get_card_score(mut winning_numbers: HashMap<String, bool>, numbers: Vec<String>) -> u32 {
    for number in numbers {
        if winning_numbers.contains_key(&number) {
            winning_numbers.insert(number, true);
        }
    }

    winning_numbers.values().fold(0, |acc, v| {
        if *v {
            if acc == 0 {
                1
            } else {
                acc * 2
            }
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
