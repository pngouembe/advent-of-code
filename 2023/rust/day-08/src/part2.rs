use std::collections::BTreeMap;

use itertools::{FoldWhile, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};

use crate::custom_error::AocError;

type Network = BTreeMap<String, (String, String)>;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, (directions, nodes)) = parse_input(_input).unwrap();

    let mut directions = directions.into_iter().cycle();

    let current_nodes: Vec<String> = nodes
        .keys()
        .filter(|node| node.to_string().ends_with('A'))
        .map(|node| node.to_string())
        .collect();

    let (steps, _) = directions
        .fold_while(
            (0, current_nodes),
            |(mut steps, mut current_nodes), directions| {
                current_nodes = current_nodes
                    .iter()
                    .map(|node| {
                        let (left, right) = nodes.get(node).unwrap();
                        if directions == 'L' {
                            left.to_string()
                        } else {
                            right.to_string()
                        }
                    })
                    .collect();

                steps += 1;

                println!("{:?}: {:?}", steps, current_nodes);

                if current_nodes.iter().all(|node| node.ends_with('Z')) {
                    FoldWhile::Done((steps, current_nodes))
                } else {
                    FoldWhile::Continue((steps, current_nodes))
                }
            },
        )
        .into_inner();

    Ok((steps).to_string())
}

fn parse_input(input: &str) -> IResult<&str, (Vec<char>, Network)> {
    let (input, directions) = many1(one_of("LR"))(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, nodes) = separated_list1(line_ending, node_parser)(input)?;

    Ok((input, (directions, nodes.into_iter().collect())))
}

fn node_parser(input: &str) -> IResult<&str, (String, (String, String))> {
    let (input, node_name) = many1(alphanumeric1)(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, (left, right)) = nom::sequence::delimited(
        tag("("),
        nom::sequence::separated_pair(many1(alphanumeric1), tag(", "), many1(alphanumeric1)),
        tag(")"),
    )(input)?;

    Ok((input, (node_name.join(""), (left.join(""), right.join("")))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
