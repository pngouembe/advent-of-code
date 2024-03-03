use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};

use crate::custom_error::AocError;

type Network = BTreeMap<String, (String, String)>;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, (directions, nodes)) = parse_input(_input).unwrap();

    let directions = directions.into_iter().cycle();

    let mut steps = 0;
    let mut current_node = nodes.keys().next().unwrap().to_string();
    for direction in directions {
        let (left, right) = nodes.get(&current_node).unwrap();
        current_node = if direction == 'L' {
            left.to_string()
        } else {
            right.to_string()
        };

        steps += 1;

        if current_node == "ZZZ" {
            break;
        }
    }

    Ok((steps).to_string())
}

fn parse_input(input: &str) -> IResult<&str, (Vec<char>, Network)> {
    let (input, directions) = many1(one_of("LR"))(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, nodes) = separated_list1(line_ending, node_parser)(input)?;

    Ok((input, (directions, nodes.into_iter().collect())))
}

fn node_parser(input: &str) -> IResult<&str, (String, (String, String))> {
    let (input, node_name) = many1(alpha1)(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, (left, right)) = nom::sequence::delimited(
        tag("("),
        nom::sequence::separated_pair(many1(alpha1), tag(", "), many1(alpha1)),
        tag(")"),
    )(input)?;

    Ok((input, (node_name.join(""), (left.join(""), right.join("")))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
