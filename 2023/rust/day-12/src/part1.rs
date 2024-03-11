use std::collections::HashSet;

use itertools::{Itertools, Position};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::value,
    multi::{fold_many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct SpringField {
    rows: Vec<SpringRow>,
}

#[derive(Debug)]
struct SpringRow {
    springs: Vec<Spring>,
    damaged_continuous_groups: Vec<u32>,
}

#[derive(Debug)]
struct Spring {
    state: SpringState,
}

#[derive(Debug, Clone)]
enum SpringState {
    Operational,
    Damaged,
    Unkown,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, spring_field) = parse_input(_input).unwrap();

    let total_count = spring_field.rows.len();

    let result = spring_field
        .rows
        .iter()
        .enumerate()
        .map(|(i, row)| {
            println!("Processing the {}th line over {}", i + 1, total_count);
            compute_possible_arrangements(row)
        })
        .sum::<u32>();

    Ok(result.to_string())
}

fn parse_input(input: &str) -> IResult<&str, SpringField> {
    let (_, rows) = separated_list1(
        line_ending,
        separated_pair(spring_parser, space1, continuous_group_parser),
    )(input)?;

    let rows = rows
        .into_iter()
        .map(|(springs, damaged_continuous_groups)| SpringRow {
            springs,
            damaged_continuous_groups,
        })
        .collect();

    Ok(("", SpringField { rows }))
}

fn spring_parser(input: &str) -> IResult<&str, Vec<Spring>> {
    let (input, springs) = fold_many1(
        alt((
            value(SpringState::Operational, tag(".")),
            value(SpringState::Damaged, tag("#")),
            value(SpringState::Unkown, tag("?")),
        )),
        Vec::new,
        |mut acc: Vec<Spring>, item| {
            acc.push(Spring { state: item });
            acc
        },
    )(input)?;

    Ok((input, springs))
}

fn continuous_group_parser(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, groups) = separated_list1(tag(","), complete::u32)(input)?;
    Ok((input, groups))
}

fn compute_possible_arrangements(row: &SpringRow) -> u32 {
    let placement_count = row.damaged_continuous_groups.len() as u32 + 1;

    let groups = row
        .damaged_continuous_groups
        .iter()
        .map(|&group| "#".repeat(group as usize))
        .collect::<Vec<String>>();

    let elements_to_place =
        row.springs.len() as u32 - row.damaged_continuous_groups.iter().sum::<u32>();

    let dots_to_insert = arranged_dots(placement_count, elements_to_place);

    let mut count = 0;

    for dots in dots_to_insert {
        let string_to_compare: String = dots.iter().interleave(groups.iter()).join("");
        let mut is_a_possible_arrangement = true;

        for (a, b) in string_to_compare.chars().zip(row.springs.iter()) {
            match (a, &b.state) {
                ('#', SpringState::Operational) | ('.', SpringState::Damaged) => {
                    is_a_possible_arrangement = false;
                    break;
                }
                _ => (),
            }
        }

        if is_a_possible_arrangement {
            // println!(
            //     "possible arrangement found: {} ({:?} - {:?}) dots: {:?}",
            //     string_to_compare,
            //     row.springs
            //         .iter()
            //         .map(|s| match s {
            //             Spring {
            //                 state: SpringState::Operational,
            //             } => '.',
            //             Spring {
            //                 state: SpringState::Damaged,
            //             } => '#',
            //             Spring {
            //                 state: SpringState::Unkown,
            //             } => '?',
            //         })
            //         .collect::<String>(),
            //     row.damaged_continuous_groups,
            //     dots
            // );
            count += 1;
        }
    }

    count
}

fn arranged_dots(placement_count: u32, elements_to_place: u32) -> HashSet<Vec<String>> {
    let mut groups = vec![String::new(); placement_count as usize];
    let mut elements = vec!['.'; elements_to_place as usize];

    let mut arrangements = HashSet::new();

    distribute_elements(&mut groups, &mut elements, 0, &mut arrangements);

    arrangements
}

fn distribute_elements(
    groups: &mut Vec<String>,
    elements: &mut Vec<char>,
    index: usize,
    result: &mut HashSet<Vec<String>>,
) {
    if index == elements.len() {
        // dbg!(&groups);
        if groups
            .iter()
            .with_position()
            .any(|(i, g)| g.is_empty() && i == Position::Middle)
        {
            return;
        }
        result.insert(groups.clone());
        return;
    }

    for group_index in 0..groups.len() {
        groups[group_index].push(elements[index]); // Placez l'élément actuel dans ce groupe
        distribute_elements(groups, elements, index + 1, result);
        groups[group_index].pop(); // Retirez l'élément ajouté pour essayer les autres possibilités
    }
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
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
