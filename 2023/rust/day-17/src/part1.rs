use std::{
    collections::{BinaryHeap, HashMap},
    ops::Range,
};

use crate::custom_error::AocError;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn orthogonal(&self) -> Vec<Self> {
        match self {
            Self::Up | Self::Down => vec![Self::Left, Self::Right],
            Self::Left | Self::Right => vec![Self::Down, Self::Up],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Step {
    coordinates: Coordinates,
    direction: Direction,
    heat_loss: u32,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    tiles: Vec<Vec<u32>>,
    width: Range<i32>,
    height: Range<i32>,
}

impl Grid {
    fn new(tiles: Vec<Vec<u32>>) -> Self {
        let height = tiles.len() as i32;
        let width = tiles.first().unwrap().len() as i32;

        Self {
            tiles,
            width: (0..width),
            height: (0..height),
        }
    }

    fn get(&self, coordinates: &Coordinates) -> u32 {
        self.tiles[coordinates.y as usize][coordinates.x as usize]
    }

    fn get_possible_steps(&self, current_step: &Step) -> Vec<Step> {
        let mut possible_steps = vec![];

        for direction in current_step.direction.orthogonal() {
            let mut coordinates = current_step.coordinates;
            let mut heat_loss = current_step.heat_loss;

            for _ in 0..3 {
                if let Some(next_coordinates) =
                    coordinates.ranged_next(&direction, &self.width, &self.height)
                {
                    heat_loss += self.get(&next_coordinates);
                    coordinates = next_coordinates;
                    possible_steps.push(Step {
                        coordinates,
                        direction,
                        heat_loss,
                    });
                } else {
                    break;
                }
            }
        }

        possible_steps
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn next(&self, direction: &Direction) -> Coordinates {
        match direction {
            Direction::Up => Coordinates {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Coordinates {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Coordinates {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coordinates {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn ranged_next(
        &self,
        direction: &Direction,
        width_range: &Range<i32>,
        height_range: &Range<i32>,
    ) -> Option<Coordinates> {
        let next = self.next(direction);

        if width_range.contains(&next.x) && height_range.contains(&next.y) {
            Some(next)
        } else {
            None
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // get the grid
    let grid: Grid = parse_input(input);

    // get starting point
    let start = Coordinates {
        x: grid.width.start,
        y: grid.height.start,
    };

    // get end point
    let end = Coordinates {
        x: grid.width.end - 1,
        y: grid.height.end - 1,
    };

    // implement the algorithm to find the path with the least amount of heat loss
    let minimal_heat_loss = find_minimal_heat_loss(&grid, &start, &end);

    Ok(minimal_heat_loss.to_string())
}

fn parse_input(input: &str) -> Grid {
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid input"))
                .collect::<Vec<u32>>()
        })
        .collect();

    Grid::new(grid)
}

fn find_minimal_heat_loss(grid: &Grid, start: &Coordinates, end: &Coordinates) -> u32 {
    let mut minimal_heat_loss_per_coordinate: HashMap<(Coordinates, Direction), u32> =
        HashMap::new();

    let mut steps_to_check: BinaryHeap<Step> = BinaryHeap::new();

    // Add the two initial steps in the steps to check
    steps_to_check.push(Step {
        coordinates: *start,
        direction: Direction::Right,
        heat_loss: 0,
    });

    steps_to_check.push(Step {
        coordinates: *start,
        direction: Direction::Down,
        heat_loss: 0,
    });

    while let Some(step) = steps_to_check.pop() {
        println!("{:?}", step);

        if step.coordinates == *end {
            return step.heat_loss;
        }

        if step.heat_loss
            > *minimal_heat_loss_per_coordinate
                .get(&(step.coordinates, step.direction))
                .unwrap_or(&u32::MAX)
        {
            // another path already reached those coordinates while losing less heat, nothing else to do.
            println!(
                "Another path reached {:?} losing less heat",
                step.coordinates
            );
            continue;
        }

        for next_step in grid.get_possible_steps(&step).iter() {
            println!("possible next step: {:?}", next_step);

            if next_step.heat_loss
                < *minimal_heat_loss_per_coordinate
                    .get(&(next_step.coordinates, next_step.direction))
                    .unwrap_or(&u32::MAX)
            {
                // we found the most efficient path to next_step yet, continue looking for the goal from there.
                steps_to_check.push(*next_step);

                minimal_heat_loss_per_coordinate.insert(
                    (next_step.coordinates, next_step.direction),
                    next_step.heat_loss,
                );
            }
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!("102", process(input)?);
        Ok(())
    }
}
