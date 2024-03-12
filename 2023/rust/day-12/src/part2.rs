use cached::proc_macro::cached;
use itertools::{Itertools, Position};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input
        .lines()
        .map(count_damaged_springs_arrangements)
        .sum::<u32>();

    Ok(result.to_string())
}

fn count_damaged_springs_arrangements(line: &str) -> u32 {
    let (_, (springs, groups)) = parse_line(line).unwrap();

    let extended_springs = String::from(".") + &springs + ".";

    count_arrangements(extended_springs, groups)
}

fn parse_line(input: &str) -> IResult<&str, (String, Vec<u32>)> {
    let (input, (springs, groups)) = separated_pair(
        is_a(".?#"),
        space1,
        separated_list1(tag(","), complete::u32),
    )(input)?;

    let springs = std::iter::repeat(springs).take(5).join("?");
    let groups = std::iter::repeat(groups).take(5).flatten().collect();

    Ok((input, (springs, groups)))
}

#[cached]
fn count_arrangements(springs: String, groups: Vec<u32>) -> u32 {
    if groups.is_empty() {
        return match springs.contains('#') {
            true => 0,
            false => 1,
        };
    }

    let group_to_place = groups.first().unwrap();

    let mut count = 0;

    for (i, window) in springs
        .chars()
        .collect::<Vec<_>>()
        .windows((*group_to_place + 2) as usize)
        .enumerate()
    {
        if !springs[..i].contains('#') && group_fits(window) {
            count += count_arrangements(
                springs[i + window.len() - 1..].to_string(),
                groups[1..].to_vec(),
            );
        }
    }

    count
}

fn group_fits(window: &[char]) -> bool {
    if window.iter().with_position().all(|(i, c)| match (i, c) {
        (i, &'#') if i == Position::First || i == Position::Last => false,
        (Position::Middle, &'.') => false,
        _ => true,
    }) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("525152", process(input)?);
        Ok(())
    }
}
