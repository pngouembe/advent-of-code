use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::{
        complete::{self, line_ending},
        streaming::alphanumeric1,
    },
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug, Clone)]
struct Instruction {
    direction: char,
    distance: u32,
    edge_color: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Step {
    coordinates: Coordinate,
    direction: Direction,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpToRight,
    UpToLeft,
    DownToRight,
    DownToLeft,
    LeftToUp,
    LeftToDown,
    RightToUp,
    RightToDown,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, mut instruction_list) = parse_input(input).unwrap();

    let mut digged_tiles = Vec::new();

    let mut current_position = Coordinate { x: 0, y: 0 };

    instruction_list.push(instruction_list.first().unwrap().clone());

    for instruction in instruction_list {
        let mut new_position = current_position;
        match instruction.direction {
            'R' => {
                for _ in 0..instruction.distance {
                    new_position.x += 1;
                    digged_tiles.push(new_position);
                }
            }
            'L' => {
                for _ in 0..instruction.distance {
                    new_position.x -= 1;
                    digged_tiles.push(new_position);
                }
            }
            'U' => {
                for _ in 0..instruction.distance {
                    new_position.y -= 1;
                    digged_tiles.push(new_position);
                }
            }
            'D' => {
                for _ in 0..instruction.distance {
                    new_position.y += 1;
                    digged_tiles.push(new_position);
                }
            }
            d => panic!("Invalid direction {}", d),
        }

        current_position = new_position;
    }

    let lagoon_area = get_area(digged_tiles);

    Ok(lagoon_area.to_string())
}

fn get_area(polygone: Vec<Coordinate>) -> u32 {
    let min_x = polygone.iter().map(|c| c.x).min().unwrap() - 1;
    let max_x = polygone.iter().map(|c| c.x).max().unwrap() + 1;

    let min_y = polygone.iter().map(|c| c.y).min().unwrap() - 1;
    let max_y = polygone.iter().map(|c| c.y).max().unwrap() + 1;

    let mut visited_coordinates: HashSet<Coordinate> = HashSet::new();
    let mut coordinates_to_visit: BinaryHeap<Coordinate> = BinaryHeap::new();

    let mut area = (min_x..max_x + 1).len() as u32 * (min_y..max_y + 1).len() as u32;

    println!("Starting Area: {}", area);

    coordinates_to_visit.push(Coordinate { x: min_x, y: min_y });

    while !coordinates_to_visit.is_empty() {
        let current_coordinate = coordinates_to_visit.pop().unwrap();

        print!("({}, {}) -> ", current_coordinate.x, current_coordinate.y);

        if visited_coordinates.contains(&current_coordinate) {
            println!("Already Visited!");
            continue;
        } else {
            visited_coordinates.insert(current_coordinate);
        }

        if polygone.contains(&current_coordinate) {
            println!("In Polygone!");
            continue;
        }

        let neighbors = get_neighbors(current_coordinate, min_x..max_x + 1, min_y..max_y + 1);

        neighbors
            .iter()
            .filter(|c| !visited_coordinates.contains(c))
            .filter(|c| !polygone.contains(c))
            .for_each(|c| coordinates_to_visit.push(*c));

        area -= 1;
        println!("Is outside! Area: {}", area);
    }

    area
}

fn get_neighbors(
    current_coordinate: Coordinate,
    x_range: Range<i32>,
    y_range: Range<i32>,
) -> Vec<Coordinate> {
    let mut neighbors = Vec::new();

    if x_range.contains(&current_coordinate.x) && y_range.contains(&(current_coordinate.y - 1)) {
        neighbors.push(Coordinate {
            x: current_coordinate.x,
            y: current_coordinate.y - 1,
        });
    }

    if x_range.contains(&current_coordinate.x) && y_range.contains(&(current_coordinate.y + 1)) {
        neighbors.push(Coordinate {
            x: current_coordinate.x,
            y: current_coordinate.y + 1,
        });
    }

    if x_range.contains(&(current_coordinate.x - 1)) && y_range.contains(&current_coordinate.y) {
        neighbors.push(Coordinate {
            x: current_coordinate.x - 1,
            y: current_coordinate.y,
        });
    }

    if x_range.contains(&(current_coordinate.x + 1)) && y_range.contains(&current_coordinate.y) {
        neighbors.push(Coordinate {
            x: current_coordinate.x + 1,
            y: current_coordinate.y,
        });
    }

    neighbors
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(line_ending, parse_instruction)(input)?;

    Ok((input, instructions))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = complete::one_of("UDLR")(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, distance) = complete::u32(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, edge_color) =
        delimited(tag("("), preceded(tag("#"), alphanumeric1), tag(")"))(input)?;

    Ok((
        input,
        Instruction {
            direction,
            distance,
            edge_color: edge_color.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!("62", process(input)?);
        Ok(())
    }
}
