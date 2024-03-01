use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::preceded, IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn get_winning_button_hold_times(&self) -> Vec<u64> {
        (1..self.time)
            .map(|pressed_time| self.distance_from_pressed_time(pressed_time))
            .filter(|distance| *distance > self.distance)
            .collect()
    }

    fn distance_from_pressed_time(&self, pressed_time: u64) -> u64 {
        (self.time - pressed_time) * pressed_time
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, race) = parse_input(_input).unwrap();

    let winning_solutions = race.get_winning_button_hold_times().len();

    Ok(winning_solutions.to_string())
}

fn parse_input(input: &str) -> IResult<&str, Race> {
    let (input, time) = preceded(tag("Time:"), list_parser)(input)?;
    let (input, _) = complete::line_ending(input)?;
    let (input, distance) = preceded(tag("Distance:"), list_parser)(input)?;

    let race = Race { time, distance };

    Ok((input, race))
}

fn list_parser(input: &str) -> IResult<&str, u64> {
    let (input, _) = complete::space1(input)?;
    let (input, numbers) = separated_list1(complete::space1, complete::digit1)(input)?;
    let numbers = numbers.concat().parse::<u64>().unwrap();
    Ok((input, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
