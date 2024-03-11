use std::collections::BTreeMap;

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let expended_universe = expend_universe(_input);

    let grid = parse_input(&expended_universe);

    let galaxies: Vec<(usize, usize)> = grid
        .iter()
        .filter_map(|(&coordinates, &c)| match c {
            '#' => Some(coordinates),
            _ => None,
        })
        .collect();

    let galaxy_pairs = galaxies.iter().combinations(2);

    let galaxy_distances: Vec<_> = galaxy_pairs
        .map(|pair| {
            let a = *pair[0];
            let b = *pair[1];

            let distance = manhattan_distance(&a, &b);

            println!("{:?} -> {:?} = {}", a, b, distance);

            distance
        })
        .collect();

    Ok(galaxy_distances.iter().sum::<usize>().to_string())
}

fn expend_universe(input: &str) -> String {
    let mut universe = expand_rows(input);
    universe = expand_columns(&universe);
    universe
}

fn expand_rows(input: &str) -> String {
    let mut universe = Vec::new();

    for (i, line) in input.lines().enumerate() {
        universe.push(line.to_string());
        if line.chars().all(|c| c == '.') {
            println!("expended row: {}", i + 1);
            for _ in 0..9 {
                universe.push(line.to_string());
            }
        }
    }
    universe.join("\n")
}

fn expand_columns(input: &str) -> String {
    let mut universe_columns = Vec::new();

    let mut columns: BTreeMap<usize, String> = BTreeMap::new();

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            columns.entry(i).or_default().push(c);
        }
    }

    for (i, column) in columns {
        universe_columns.push(column.to_string());
        if column.chars().all(|c| c == '.') {
            println!("expended column: {}", i + 1);
            for _ in 0..9 {
                universe_columns.push(column.to_string());
            }
        }
    }

    let mut rows: BTreeMap<usize, String> = BTreeMap::new();

    for column in universe_columns.iter() {
        for (i, c) in column.chars().enumerate() {
            rows.entry(i).or_default().push(c);
        }
    }

    let mut universe_rows = Vec::new();

    for (_, row) in rows.iter() {
        universe_rows.push(row.to_string());
    }

    universe_rows.join("\n")
}

fn parse_input(input: &str) -> BTreeMap<(usize, usize), char> {
    let mut grid = BTreeMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x, y), c);
        }
    }

    grid
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expend_rows() {
        let input = "#.........
..........";

        let expected = "#.........
..........
..........";

        assert_eq!(expected, expand_rows(input));
    }

    #[test]
    fn test_expension() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let ouput = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

        assert_eq!(ouput, expend_universe(input));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!("37", process(input)?);
        Ok(())
    }
}
