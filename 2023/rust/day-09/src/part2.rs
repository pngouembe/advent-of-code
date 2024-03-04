use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, histories) = parse_input(_input).unwrap();
    let result = histories.iter().fold(0, |extrapolation_sum, history| {
        extrapolation_sum + extrapolate(history)
    });

    Ok(result.to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, histories) = separated_list1(line_ending, parse_history)(input)?;
    Ok((input, histories))
}

fn parse_history(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, history) = separated_list1(tag(" "), complete::i32)(input)?;
    Ok((input, history))
}

fn extrapolate(history: &[i32]) -> i32 {
    let mut differences = get_differences(history);
    let mut steps = vec![history.to_vec(), differences.clone()];

    loop {
        if differences.iter().all(|&x| x == 0) {
            break;
        }

        differences = get_differences(&differences);
        steps.push(differences.clone());
    }

    let mut prediction = 0;

    for step in steps[..steps.len() - 1].iter().rev() {
        let first_history_value = step.first().unwrap();
        prediction = first_history_value - prediction;
    }

    prediction
}

fn get_differences(history: &[i32]) -> Vec<i32> {
    let mut differences = Vec::new();
    for i in 0..history.len() - 1 {
        differences.push(history[i + 1] - history[i]);
    }
    differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
