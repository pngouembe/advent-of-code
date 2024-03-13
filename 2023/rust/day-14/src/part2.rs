use std::collections::HashMap;

use cached::proc_macro::cached;
use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = parse_input(input);

    let max_load = input.lines().count() as u32;

    let grid = grid_cycle(grid, 1000000000);

    let load: u32 = grid
        .iter()
        .filter(|(_, c)| **c == 'O')
        .fold(0, |mut sum, ((_, y), _)| {
            sum += max_load - y;
            sum
        });

    Ok(load.to_string())
}

fn parse_input(input: &str) -> HashMap<(u32, u32), char> {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as u32, y as u32), c);
        }
    }
    grid
}

fn grid_cycle(grid: HashMap<(u32, u32), char>, cycles: u32) -> HashMap<(u32, u32), char> {
    let mut current_grid = grid.into_iter().collect();

    for i in 0..cycles {
        if i % 1000 == 0 {
            println!("cycle {}", i);
        }
        // print_grid(&current_grid);
        // println!("tilting north");
        current_grid = tilt_grid(current_grid, Direction::North);
        // print_grid(&current_grid);
        // println!("tilting west");
        current_grid = tilt_grid(current_grid, Direction::West);
        // print_grid(&current_grid);
        // println!("tilting south");
        current_grid = tilt_grid(current_grid, Direction::South);
        // print_grid(&current_grid);
        // println!("tilting east");
        current_grid = tilt_grid(current_grid, Direction::East);
        // println!("grid after {} cycles:", i + 1);
        // print_grid(&current_grid);
    }
    current_grid.into_iter().collect()
}

#[cached]
fn tilt_grid(grid: Vec<((u32, u32), char)>, direction: Direction) -> Vec<((u32, u32), char)> {
    let mut new_grid = Vec::new();
    // break the grid into columns
    for (i, stack) in get_stacks(grid, &direction).iter().enumerate() {
        // for each column, make the rounded rocks roll to the north and store their new position in the displacement_mapping
        let new_stack = collapse_stack(stack.clone());

        // println!(
        //     "stack {} direction {:?}: {:?} -> {:?}",
        //     i + 1,
        //     direction,
        //     stack.iter().map(|(_, c)| *c).join(""),
        //     new_stack.iter().map(|(_, c)| *c).join("")
        // );

        for (j, c) in new_stack {
            let coordinates = match direction {
                Direction::North => (i as u32, j),
                Direction::South => (i as u32, stack.len() as u32 - j - 1),
                Direction::West => (j, i as u32),
                Direction::East => (stack.len() as u32 - j - 1, i as u32),
            };
            new_grid.push((coordinates, c));
        }
    }

    new_grid
}

fn get_stacks(grid: Vec<((u32, u32), char)>, direction: &Direction) -> Vec<Vec<(u32, char)>> {
    let grid_width = grid.iter().map(|((i, _), _)| i).max().unwrap() + 1;
    let grid_height = grid.iter().map(|((_, j), _)| j).max().unwrap() + 1;

    let range = match direction {
        Direction::North | Direction::South => grid_width,
        Direction::East | Direction::West => grid_height,
    };

    (0..range).fold(Vec::new(), |mut stacks, i| {
        let stack = grid
            .iter()
            .filter_map(|((x, y), c)| match direction {
                Direction::North => {
                    if *x == i {
                        Some((*y, *c))
                    } else {
                        None
                    }
                }
                Direction::South => {
                    if *x == i {
                        Some((grid_height - *y - 1, *c))
                    } else {
                        None
                    }
                }
                Direction::West => {
                    if *y == i {
                        Some((*x, *c))
                    } else {
                        None
                    }
                }
                Direction::East => {
                    if *y == i {
                        Some((grid_width - *x - 1, *c))
                    } else {
                        None
                    }
                }
            })
            .sorted()
            .collect();
        stacks.push(stack);
        stacks
    })
}

fn collapse_stack(stack: Vec<(u32, char)>) -> Vec<(u32, char)> {
    let mut new_stack = Vec::new();
    let mut insertion_index = 0;
    for (i, c) in stack.iter() {
        if *c == 'O' {
            new_stack.push((insertion_index, *c));
            insertion_index += 1;
        }

        if *c == '#' {
            for j in insertion_index..*i {
                new_stack.push((j, '.'));
            }
            new_stack.push((*i, *c));
            insertion_index = i + 1;
        }
    }

    if insertion_index < stack.len() as u32 {
        for j in insertion_index..stack.len() as u32 {
            new_stack.push((j, '.'));
        }
    }

    new_stack
}

fn print_grid(grid: &HashMap<(u32, u32), char>) {
    let grid_width = grid.keys().map(|(x, _)| x).max().unwrap() + 1;
    let grid_height = grid.keys().map(|(_, y)| y).max().unwrap() + 1;

    for y in 0..grid_height {
        for x in 0..grid_width {
            print!("{}", grid.get(&(x, y)).unwrap_or(&'.'));
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("64", process(input)?);
        Ok(())
    }
}
