use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::preceded, IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn get_winning_button_hold_times(&self) -> Vec<u32> {
        (1..self.time)
            .map(|pressed_time| self.distance_from_pressed_time(pressed_time))
            .filter(|distance| *distance > self.distance)
            .collect()
    }

    fn distance_from_pressed_time(&self, pressed_time: u32) -> u32 {
        (self.time - pressed_time) * pressed_time
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, races) = parse_input(_input).unwrap();

    let winning_solutions = races
        .iter()
        .map(|race| race.get_winning_button_hold_times().len());

    Ok(winning_solutions.product::<usize>().to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = preceded(tag("Time:"), list_parser)(input).unwrap();
    let (input, _) = complete::line_ending(input)?;
    let (input, distances) = preceded(tag("Distance:"), list_parser)(input).unwrap();

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect();

    Ok((input, races))
}

fn list_parser(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = complete::space1(input)?;
    let (input, times) = separated_list1(complete::space1, complete::u32)(input)?;
    Ok((input, times))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
