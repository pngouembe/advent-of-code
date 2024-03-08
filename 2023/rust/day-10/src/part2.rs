use std::collections::BTreeMap;

use crate::custom_error::AocError;

type Coordinate = (usize, usize);

type Grid = BTreeMap<Coordinate, Pipe>;

type PipeType = char;

type PipeCoordinates<'a> = BTreeMap<Coordinate, &'a Pipe>;

#[derive(Debug)]
struct Pipe {
    pipe_type: PipeType,
    coordinates: Coordinate,
    holes: (Coordinate, Coordinate),
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (start_coordinates, grid, grid_width, grid_height) = parse_input(_input);

    let loop_coordinates = grid
        .iter()
        .filter(|(_, pipe)| pipe.holes.0 == start_coordinates || pipe.holes.1 == start_coordinates)
        .fold(Vec::new(), |mut loop_coordinates, (_, pipe)| {
            loop_coordinates.push(follow_pipe(&start_coordinates, &pipe, &grid));
            loop_coordinates
        });

    let main_loop = loop_coordinates.iter().max_by_key(|v| v.len()).unwrap();

    let max_area = get_in_loop_area(&main_loop, grid_height, grid_width);

    Ok(max_area.to_string())
}

fn parse_input(input: &str) -> (Coordinate, Grid, usize, usize) {
    let mut grid = Grid::new();
    let mut start_coordinates = (0, 0);

    let max_y = input.lines().count() - 1;
    let max_x = input.lines().next().unwrap().len() - 1;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '-' => {
                    if x > 0 && x < (line.len() - 1) {
                        grid.insert(
                            (x, y),
                            Pipe {
                                pipe_type: c,
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
                                pipe_type: c,
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
                                pipe_type: c,
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
                                pipe_type: c,
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
                                pipe_type: c,
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
                                pipe_type: c,
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
                            pipe_type: c,
                            coordinates: (x, y),
                            holes: ((x, y), (x, y)),
                        },
                    );
                }
                _ => {}
            }
        }
    }

    (start_coordinates, grid, max_x, max_y)
}

fn follow_pipe<'a>(
    start_coordinates: &'a Coordinate,
    pipe: &'a Pipe,
    grid: &'a Grid,
) -> PipeCoordinates<'a> {
    let mut pipe_coordinates: BTreeMap<(usize, usize), &Pipe> = BTreeMap::new();

    let mut previous_coordinates = start_coordinates;
    let mut current_pipe = pipe;

    pipe_coordinates.insert(current_pipe.coordinates, current_pipe);

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

        pipe_coordinates.insert(current_pipe.coordinates, current_pipe);
    }

    pipe_coordinates.insert(*start_coordinates, pipe.to_owned());

    pipe_coordinates
}

fn get_in_loop_area(
    loop_coordinates: &PipeCoordinates,
    grid_height: usize,
    grid_width: usize,
) -> usize {
    let mut points_in_loop: Vec<Coordinate> = Vec::new();

    for y in 0..grid_height {
        for x in 0..grid_width {
            let ray_intersection_count = calculate_ray_intersections(&loop_coordinates, x, y);

            if ray_intersection_count % 2 == 1 && !loop_coordinates.contains_key(&(x, y)) {
                points_in_loop.push((x, y));
            }
        }
    }

    points_in_loop.len()
}

fn calculate_ray_intersections(loop_coordinates: &PipeCoordinates, x: usize, y: usize) -> usize {
    let mut ray_intersection_count = 0;

    for i in 0..x {
        if let Some(pipe) = loop_coordinates.get(&(i, y)) {
            match pipe.pipe_type {
                '|' | 'L' | 'J' => {
                    ray_intersection_count += 1;
                }
                _ => {}
            }
        }
    }

    ray_intersection_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
