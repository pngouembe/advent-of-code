use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{self, line_ending, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, tuple};
use nom::IResult;

use std::ops::Range;

use crate::custom_error::AocError;

struct Almanac {
    pub seeds: Vec<u64>,
    pub conversion_maps: Vec<ConversionMap>,
}

type ConversionMap = Vec<Mapping>;

#[derive(Debug)]
struct Mapping {
    pub src: Range<u64>,
    pub dst: Range<u64>,
}

impl Mapping {
    pub fn translate(&self, elem: u64) -> u64 {
        self.dst.start + (elem - self.src.start)
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, almanac) = parse_input(_input).unwrap();

    let locations = almanac.seeds.iter().map(|seed| {
        almanac
            .conversion_maps
            .iter()
            .fold(*seed, |acc, conversion_map| {
                let mapping_to_use = conversion_map
                    .iter()
                    .find(|mapping| mapping.src.contains(&acc));

                match mapping_to_use {
                    Some(mapping) => mapping.translate(acc),
                    None => acc,
                }
            })
    });

    Ok(locations
        .min()
        .expect("Minimum location should exist")
        .to_string())
}

fn parse_input(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = preceded(
        tag("seeds: "),
        separated_list1(complete::space1, complete::u64),
    )(input)?;

    let (input, conversion_maps) = many1(conversion_map_parser)(input)?;

    Ok((
        input,
        Almanac {
            seeds,
            conversion_maps,
        },
    ))
}

fn conversion_map_parser(input: &str) -> IResult<&str, ConversionMap> {
    let (input, _) = take_until("map:")(input)?;
    let (input, _) = tag("map:")(input)?;
    let (input, _) = complete::newline(input)?;
    let (input, conversion_map) = separated_list1(line_ending, mapping_parser)(input)?;

    Ok((input, conversion_map))
}

fn mapping_parser(input: &str) -> IResult<&str, Mapping> {
    let (input, (dst, src, length)) = tuple((
        complete::u64,
        preceded(space1, complete::u64),
        preceded(space1, complete::u64),
    ))(input)?;

    let mapping = Mapping {
        src: src..(src + length),
        dst: dst..(dst + length),
    };

    Ok((input, mapping))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
