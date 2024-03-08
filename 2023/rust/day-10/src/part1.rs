use std::collections::BTreeMap;

use crate::custom_error::AocError;

type Coordinate = (usize, usize);

type Grid = BTreeMap<Coordinate, Pipe>;

#[derive(Debug)]
struct Pipe {
    coordinates: Coordinate,
    holes: (Coordinate, Coordinate),
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (start_coordinates, grid) = parse_input(_input);

    let loop_lengths = grid
        .iter()
        .filter(|(_, pipe)| pipe.holes.0 == start_coordinates || pipe.holes.1 == start_coordinates)
        .fold(Vec::new(), |mut loop_length, (_, pipe)| {
            loop_length.push(follow_pipe(&start_coordinates, &pipe, &grid));
            loop_length
        });

    let max_loop_length = loop_lengths.iter().max().unwrap() / 2;

    Ok(max_loop_length.to_string())
}

fn parse_input(input: &str) -> (Coordinate, Grid) {
    let mut grid = Grid::new();
    let mut start_coordinates = (0, 0);

    let max_y = input.lines().count() - 1;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '-' => {
                    if x > 0 && x < (line.len() - 1) {
                        grid.insert(
                            (x, y),
                            Pipe {
                                coordinates: (x, y),
                                holes: ((x - 1, y), (x + 1, y)),
                            },
                        );
                    }
                }
                '|' => {
                    if y > 0 && y < max_y {
                        grid.insert(
                            (x, y),
                            Pipe {
                                coordinates: (x, y),
                                holes: ((x, y - 1), (x, y + 1)),
                            },
                        );
                    }
                }
                'L' => {
                    if y > 0 && x < (line.len() - 1) {
                        grid.insert(
                            (x, y),
                            Pipe {
                                coordinates: (x, y),
                                holes: ((x, y - 1), (x + 1, y)),
                            },
                        );
                    }
                }
                'J' => {
                    if x > 0 && y > 0 {
                        grid.insert(
                            (x, y),
                            Pipe {
                                coordinates: (x, y),
                                holes: ((x - 1, y), (x, y - 1)),
                            },
                        );
                    }
                }
                'F' => {
                    if y < max_y && x < (line.len() - 1) {
                        grid.insert(
                            (x, y),
                            Pipe {
                                coordinates: (x, y),
                                holes: ((x, y + 1), (x + 1, y)),
                            },
                        );
                    }
                }
                '7' => {
                    if x > 0 && y < max_y {
                        grid.insert(
                            (x, y),
                            Pipe {
                                coordinates: (x, y),
                                holes: ((x - 1, y), (x, y + 1)),
                            },
                        );
                    }
                }
                'S' => {
                    start_coordinates = (x, y);
                    grid.insert(
                        (x, y),
                        Pipe {
                            coordinates: (x, y),
                            holes: ((x, y), (x, y)),
                        },
                    );
                }
                _ => {}
            }
        }
    }

    (start_coordinates, grid)
}

fn follow_pipe(start_coordinates: &Coordinate, pipe: &Pipe, grid: &Grid) -> u32 {
    let mut loop_length = 1;

    let mut previous_coordinates = start_coordinates;
    let mut current_pipe = pipe;

    while current_pipe.coordinates != *start_coordinates {
        let next_pipe_coordinates = if current_pipe.holes.0 == *previous_coordinates {
            current_pipe.holes.1
        } else {
            current_pipe.holes.0
        };

        previous_coordinates = &current_pipe.coordinates;

        current_pipe = grid
            .get(&next_pipe_coordinates)
            .unwrap_or_else(|| panic!("Pipe should exist in grid. ({:?})", next_pipe_coordinates));
        loop_length += 1;
    }

    loop_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
