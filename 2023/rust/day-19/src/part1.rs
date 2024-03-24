use std::{cmp::Ordering, collections::HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
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
    condition: Option<(char, Ordering, u32)>,
}
#[derive(Debug)]
struct Part {
    ratings: HashMap<char, u32>,
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Accepted,
    Rejected,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, (workflow_map, parts)) = parse_input(input).unwrap();

    let result: u32 = parts
        .iter()
        .map(|part| (part, process_part(&workflow_map, part)))
        .filter(|(_, part_status)| *part_status == Status::Accepted)
        .map(|(part, _)| part.ratings.values().sum::<u32>())
        .sum();
    Ok(result.to_string())
}

fn process_part(workflow_map: &WorkflowMap, part: &Part) -> Status {
    execute_workflow(workflow_map, part, "in")
}

fn execute_workflow(workflow_map: &WorkflowMap, part: &Part, workflow_name: &str) -> Status {
    let rules = workflow_map.get(workflow_name).unwrap();

    for rule in rules {
        if let Some((rating, comparison, value)) = &rule.condition {
            let part_rating = part.ratings.get(rating).expect("Rating not found");
            if part_rating.cmp(value) != *comparison {
                continue;
            }
        }
        if rule.redirection == "A" {
            return Status::Accepted;
        } else if rule.redirection == "R" {
            return Status::Rejected;
        } else {
            return execute_workflow(workflow_map, part, &rule.redirection);
        }
    }
    unreachable!()
}

fn parse_input(input: &str) -> IResult<&str, (WorkflowMap, Vec<Part>)> {
    let (input, workflows) = workflows_parser(input)?;
    let (input, _) = multispace1(input)?;

    let (input, parts) = parts_parser(input)?;

    let workflows = workflows
        .into_iter()
        .map(|workflow| (workflow.name, workflow.rules))
        .collect();

    Ok((input, (workflows, parts)))
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

fn condition_parser(input: &str) -> IResult<&str, (char, Ordering, u32)> {
    let (input, field) = complete::one_of("xmas")(input)?;
    let (input, op) = complete::one_of("<>")(input)?;
    let (input, value) = complete::u32(input)?;

    let op = match op {
        '<' => Ordering::Less,
        '>' => Ordering::Greater,
        _ => unreachable!(),
    };

    Ok((input, (field, op, value)))
}

fn parts_parser(input: &str) -> IResult<&str, Vec<Part>> {
    let (input, parts) = separated_list1(line_ending, part_parser)(input)?;
    Ok((input, parts))
}

fn part_parser(input: &str) -> IResult<&str, Part> {
    let (input, ratings) = delimited(
        tag("{"),
        separated_list1(
            tag(","),
            separated_pair(complete::one_of("xmas"), complete::char('='), complete::u32),
        ),
        tag("}"),
    )(input)?;

    Ok((
        input,
        Part {
            ratings: ratings.into_iter().collect(),
        },
    ))
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
        assert_eq!("19114", process(input)?);
        Ok(())
    }
}
