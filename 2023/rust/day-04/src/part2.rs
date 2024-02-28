use crate::custom_error::AocError;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut cards: HashMap<u32, u32> = HashMap::new();

    for line in _input.lines() {
        let (card_number, winning_numbers, numbers) = parse_card(line);

        let card_count = cards.entry(card_number).or_insert(0);
        *card_count += 1;

        let multiplier = *card_count;

        let score = get_match_count(winning_numbers, numbers);
        for i in card_number + 1..=card_number + score {
            let c = cards.entry(i).or_insert(0);
            *c += multiplier;
        }
    }

    let cards_count: u32 = cards.values().sum();
    Ok(cards_count.to_string())
}

fn parse_card(card: &str) -> (u32, HashMap<String, bool>, Vec<String>) {
    let mut winning_numbers: HashMap<String, bool> = HashMap::new();
    let mut numbers: Vec<String> = Vec::new();

    let mut card_parts = card.splitn(2, ':');
    let card_number = card_parts
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let card_content = card_parts.next().unwrap();

    let mut card_parts = card_content.splitn(2, '|');

    let winning_numbers_str = card_parts.next().unwrap().trim();
    let numbers_str = card_parts.next().unwrap().trim();

    for number in winning_numbers_str.split_whitespace() {
        winning_numbers.insert(number.to_string(), false);
    }

    for number in numbers_str.split_whitespace() {
        numbers.push(number.to_string());
    }

    (card_number, winning_numbers, numbers)
}

fn get_match_count(mut winning_numbers: HashMap<String, bool>, numbers: Vec<String>) -> u32 {
    for number in numbers {
        if winning_numbers.contains_key(&number) {
            winning_numbers.insert(number, true);
        }
    }

    winning_numbers
        .values()
        .fold(0, |acc, v| if *v { acc + 1 } else { acc })
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
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
