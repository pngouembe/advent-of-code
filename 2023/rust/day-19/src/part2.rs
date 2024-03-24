use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ops::Range,
};

use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

type WorkflowMap = HashMap<String, Vec<Rule>>;

#[derive(Debug)]
struct Rule {
    redirection: String,
    condition: Option<(char, Ordering, u64)>,
}
#[derive(Debug, Clone)]
struct Part {
    ratings: HashMap<char, Range<u64>>,
}

impl Part {
    fn possibilities(&self) -> u64 {
        self.ratings
            .values()
            .map(|range| range.end - range.start)
            .product()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Accepted,
    Rejected,
}

const MAX_RATING: u64 = 4000;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, workflow_map) = parse_input(input).unwrap();

    let result = compute_part_combinations_count(&workflow_map);

    Ok(result.to_string())
}

fn compute_part_combinations_count(workflow_map: &WorkflowMap) -> u64 {
    let initial_part = Part {
        ratings: vec![
            ('x', 0..MAX_RATING + 1),
            ('m', 0..MAX_RATING + 1),
            ('a', 0..MAX_RATING + 1),
            ('s', 0..MAX_RATING + 1),
        ]
        .into_iter()
        .collect(),
    };

    let mut already_visited_workflows = HashSet::new();

    compute_part_combinations_count_recursive(
        workflow_map,
        "in",
        initial_part,
        &mut already_visited_workflows,
    )
}

// TODO: Fix the overlapping problem (we are looking for unique posibilities and we are counting them multiple times)
fn compute_part_combinations_count_recursive(
    workflow_map: &WorkflowMap,
    workflow_name: &str,
    parts: Part,
    already_visited_workflows: &mut HashSet<(String, usize)>,
) -> u64 {
    let rules = workflow_map.get(workflow_name).unwrap();

    let mut count = 0;

    for (i, rule) in rules.iter().enumerate() {
        let mut new_parts = parts.clone();

        match &rule.condition {
            Some((rating, Ordering::Less, value)) => {
                new_parts
                    .ratings
                    .entry(*rating)
                    .and_modify(|range| range.end = range.end.min(*value));
            }
            Some((rating, Ordering::Greater, value)) => {
                new_parts
                    .ratings
                    .entry(*rating)
                    .and_modify(|range| range.start = range.start.max(*value + 1));
            }
            _ => {}
        }

        dbg!(workflow_name, &new_parts);

        if rule.redirection == "A" {
            count += dbg!(new_parts.possibilities());
            already_visited_workflows.insert((workflow_name.to_string(), i));
        } else if rule.redirection != "R" {
            count += compute_part_combinations_count_recursive(
                workflow_map,
                &rule.redirection,
                new_parts,
                already_visited_workflows,
            );
            already_visited_workflows.insert((workflow_name.to_string(), i));
        }
    }

    count
}

fn parse_input(input: &str) -> IResult<&str, WorkflowMap> {
    let (input, workflows) = workflows_parser(input)?;

    let workflows = workflows
        .into_iter()
        .map(|workflow| (workflow.name, workflow.rules))
        .collect();

    Ok((input, workflows))
}

fn workflows_parser(input: &str) -> IResult<&str, Vec<Workflow>> {
    let (input, workflows) = separated_list1(line_ending, workflow_parser)(input)?;

    Ok((input, workflows))
}

fn workflow_parser(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = complete::alpha1(input)?;
    let (input, rules) = delimited(
        complete::char('{'),
        separated_list1(complete::char(','), rule_parser),
        complete::char('}'),
    )(input)?;

    Ok((
        input,
        Workflow {
            name: name.to_string(),
            rules,
        },
    ))
}

fn rule_parser(input: &str) -> IResult<&str, Rule> {
    let (input, rule) = alt((conditionnal_rule_parser, unconditionnal_rule_parser))(input)?;

    Ok((input, rule))
}

fn conditionnal_rule_parser(input: &str) -> IResult<&str, Rule> {
    let (input, condition) = terminated(condition_parser, complete::char(':'))(input)?;
    let (input, redirection) = complete::alpha1(input)?;

    Ok((
        input,
        Rule {
            redirection: redirection.to_string(),
            condition: Some(condition),
        },
    ))
}

fn unconditionnal_rule_parser(input: &str) -> IResult<&str, Rule> {
    let (input, redirection) = complete::alpha1(input)?;

    Ok((
        input,
        Rule {
            redirection: redirection.to_string(),
            condition: None,
        },
    ))
}

fn condition_parser(input: &str) -> IResult<&str, (char, Ordering, u64)> {
    let (input, field) = complete::one_of("xmas")(input)?;
    let (input, op) = complete::one_of("<>")(input)?;
    let (input, value) = complete::u64(input)?;

    let op = match op {
        '<' => Ordering::Less,
        '>' => Ordering::Greater,
        _ => unreachable!(),
    };

    Ok((input, (field, op, value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!("167409079868000", process(input)?);
        Ok(())
    }
}
