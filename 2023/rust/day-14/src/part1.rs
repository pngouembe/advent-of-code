use std::collections::HashMap;

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = parse_input(input);

    let max_load = input.lines().count() as u32;
    let width = input.lines().next().unwrap().len() as u32;

    // let grid = grid_cycle(grid, 1000000000);

    let grid = tilt_grid_north(&grid);

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

fn tilt_grid_north(grid: &HashMap<(u32, u32), char>) -> HashMap<(u32, u32), char> {
    let mut displacement_mapping: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let grid_width = grid.keys().map(|(x, _)| x).max().unwrap() + 1;
    let grid_height = grid.keys().map(|(_, y)| y).max().unwrap() + 1;

    // break the grid into columns
    for x in 0..grid_width {
        let column = grid
            .iter()
            .filter(|((i, _), _)| *i == x)
            .map(|((_, j), c)| (j, *c))
            .sorted_by(|(a, _), (b, _)| a.cmp(b));

        // for each column, make the rounded rocks roll to the north and store their new position in the displacement_mapping
        let mut insertion_index = 0;
        for (j, c) in column {
            if c == 'O' {
                displacement_mapping.insert((x, *j), (x, insertion_index));
                insertion_index += 1;
            }

            if c == '#' {
                insertion_index = j + 1;
            }
        }
    }

    // analize the displacement_mapping to create the new grid
    let mut new_grid: HashMap<(u32, u32), char> = HashMap::new();
    for (x, y) in grid
        .iter()
        .filter_map(|((x, y), c)| if c == &'#' { Some((x, y)) } else { None })
    {
        new_grid.insert((*x, *y), '#');
    }

    for ((x, y), (i, j)) in displacement_mapping.iter() {
        new_grid.insert(
            (*i, *j),
            *grid
                .get(&(*x, *y))
                .expect("Trying to get a value from the grid that doesn't exist."),
        );
    }
    new_grid
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
        assert_eq!("136", process(input)?);
        Ok(())
    }
}
