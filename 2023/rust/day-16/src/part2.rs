use std::{ops::Range, vec};

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // parse the input grid

    let grid = parse_input(input);

    let mut initial_steps = vec![];

    for y in grid.height.clone() {
        initial_steps.push(Step {
            coordinates: Coordinates { x: 0, y },
            direction: Direction::Right,
        });
        initial_steps.push(Step {
            coordinates: Coordinates {
                x: grid.width.end - 1,
                y,
            },
            direction: Direction::Left,
        });
    }

    for x in grid.width.clone() {
        initial_steps.push(Step {
            coordinates: Coordinates { x, y: 0 },
            direction: Direction::Down,
        });
        initial_steps.push(Step {
            coordinates: Coordinates {
                x,
                y: grid.height.end - 1,
            },
            direction: Direction::Up,
        });
    }

    let max_count = initial_steps
        .into_iter()
        .map(|initial_step| {
            let beam = traverse_grid(&grid, &initial_step);

            let count = beam
                .visited_steps
                .iter()
                .unique_by(|step| step.coordinates)
                .count();

            count
        })
        .max()
        .unwrap();

    Ok(max_count.to_string())
}

fn parse_input(input: &str) -> Grid {
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    Tile::from_char(c).unwrap_or_else(|| panic!("unexpected character: {}", c))
                })
                .collect::<Vec<Tile>>()
        })
        .collect();

    Grid::new(grid)
}

fn traverse_grid(grid: &Grid, initial_step: &Step) -> LightBeam {
    let mut beam = LightBeam::new(initial_step.clone());

    beam.raycast(grid);

    beam
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    width: Range<i32>,
    height: Range<i32>,
}

impl Grid {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let height = tiles.len() as i32;
        let width = tiles.first().unwrap().len() as i32;

        Self {
            tiles,
            width: (0..width),
            height: (0..height),
        }
    }

    fn get(&self, coordinates: &Coordinates) -> Option<Tile> {
        if !self.width.contains(&coordinates.x) || !self.height.contains(&coordinates.y) {
            return None;
        }

        let x = coordinates.x as usize;
        let y = coordinates.y as usize;

        self.tiles.get(y).and_then(|row| row.get(x).copied())
    }

    fn get_next_steps(&self, step: &Step) -> Vec<Step> {
        let tile = self
            .get(&step.coordinates)
            .expect("The given coordinates are out of grid");

        tile.get_next_steps(step, &self.width, &self.height)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    SlashMirror,
    BackslashMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empty),
            '/' => Some(Self::SlashMirror),
            '\\' => Some(Self::BackslashMirror),
            '|' => Some(Self::VerticalSplitter),
            '-' => Some(Self::HorizontalSplitter),
            _ => None,
        }
    }

    fn get_next_steps(&self, step: &Step, x_range: &Range<i32>, y_range: &Range<i32>) -> Vec<Step> {
        let coordinates = step.coordinates;
        let incoming_direction = step.direction;

        match (self, incoming_direction) {
            (Self::Empty, direction) => {
                if let Some(next_coordinates) =
                    coordinates.ranged_next(&direction, x_range, y_range)
                {
                    vec![Step {
                        coordinates: next_coordinates,
                        direction,
                    }]
                } else {
                    vec![]
                }
            }

            (Self::SlashMirror, direction) => {
                let next_direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                };

                if let Some(next_coordinates) =
                    coordinates.ranged_next(&next_direction, x_range, y_range)
                {
                    vec![Step {
                        coordinates: next_coordinates,
                        direction: next_direction,
                    }]
                } else {
                    vec![]
                }
            }

            (Self::BackslashMirror, direction) => {
                let next_direction = match direction {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                };

                if let Some(next_coordinates) =
                    coordinates.ranged_next(&next_direction, x_range, y_range)
                {
                    vec![Step {
                        coordinates: next_coordinates,
                        direction: next_direction,
                    }]
                } else {
                    vec![]
                }
            }

            (Self::VerticalSplitter, direction) => {
                if direction == Direction::Up || direction == Direction::Down {
                    if let Some(next_coordinates) =
                        coordinates.ranged_next(&direction, x_range, y_range)
                    {
                        vec![Step {
                            coordinates: next_coordinates,
                            direction,
                        }]
                    } else {
                        vec![]
                    }
                } else {
                    let mut possible_next_steps = vec![];

                    if let Some(next_coordinates) =
                        coordinates.ranged_next(&Direction::Up, x_range, y_range)
                    {
                        possible_next_steps.push(Step {
                            coordinates: next_coordinates,
                            direction: Direction::Up,
                        });
                    }

                    if let Some(next_coordinates) =
                        coordinates.ranged_next(&Direction::Down, x_range, y_range)
                    {
                        possible_next_steps.push(Step {
                            coordinates: next_coordinates,
                            direction: Direction::Down,
                        });
                    }

                    possible_next_steps
                }
            }
            (Self::HorizontalSplitter, direction) => {
                if direction == Direction::Left || direction == Direction::Right {
                    if let Some(next_coordinates) =
                        coordinates.ranged_next(&direction, x_range, y_range)
                    {
                        vec![Step {
                            coordinates: next_coordinates,
                            direction,
                        }]
                    } else {
                        vec![]
                    }
                } else {
                    let mut possible_next_steps = vec![];

                    if let Some(next_coordinates) =
                        coordinates.ranged_next(&Direction::Left, x_range, y_range)
                    {
                        possible_next_steps.push(Step {
                            coordinates: next_coordinates,
                            direction: Direction::Left,
                        });
                    }

                    if let Some(next_coordinates) =
                        coordinates.ranged_next(&Direction::Right, x_range, y_range)
                    {
                        possible_next_steps.push(Step {
                            coordinates: next_coordinates,
                            direction: Direction::Right,
                        });
                    }

                    possible_next_steps
                }
            }
        }
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Step {
    coordinates: Coordinates,
    direction: Direction,
}

#[derive(Debug)]
struct LightBeam {
    current_step: Step,
    visited_steps: Vec<Step>,
}

impl LightBeam {
    fn new(initial_step: Step) -> Self {
        LightBeam {
            current_step: initial_step.clone(),
            visited_steps: vec![initial_step],
        }
    }

    fn set_current_step(&mut self, step: Step) {
        self.current_step = step.clone();
        self.visited_steps.push(step);
    }

    fn raycast(&mut self, grid: &Grid) {
        let next_possible_steps = grid.get_next_steps(&self.current_step);

        let next_possible_steps: Vec<_> = next_possible_steps
            .into_iter()
            .filter(|step| !self.already_visited(step))
            .collect();

        if next_possible_steps.is_empty() {
            return;
        }

        for next_step in next_possible_steps {
            self.set_current_step(next_step);
            self.raycast(grid);
        }
    }

    fn already_visited(&self, step: &Step) -> bool {
        self.visited_steps.iter().any(|visited_step| {
            visited_step.coordinates == step.coordinates && visited_step.direction == step.direction
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!("51", process(input)?);
        Ok(())
    }
}
