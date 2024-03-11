use std::collections::{BTreeMap, HashSet};

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (expended_rows, expended_columns) = expend_universe(_input);

    let grid = parse_input(_input);

    let galaxies: Vec<(usize, usize)> = grid
        .iter()
        .filter_map(|(&coordinates, &c)| match c {
            '#' => Some(coordinates),
            _ => None,
        })
        .collect();

    let galaxy_pairs = galaxies.iter().combinations(2);

    let expension_multiplier = 1000000 - 1;

    let galaxy_distances = galaxy_pairs.map(|pair| {
        let mut a = *pair[0];
        let mut b = *pair[1];

        a.0 += expended_columns.iter().filter(|&c| c < &a.0).count() * expension_multiplier;
        b.0 += expended_columns.iter().filter(|&c| c < &b.0).count() * expension_multiplier;

        a.1 += expended_rows.iter().filter(|&r| r < &a.1).count() * expension_multiplier;
        b.1 += expended_rows.iter().filter(|&r| r < &b.1).count() * expension_multiplier;

        let distance = manhattan_distance(&a, &b);

        // println!(
        //     "{:?}:{:?} -> {:?}:{:?} = {}",
        //     pair[0], pair[1], a, b, distance
        // );

        distance
    });

    Ok(galaxy_distances.sum::<usize>().to_string())
}

fn expend_universe(input: &str) -> (HashSet<usize>, HashSet<usize>) {
    let expended_rows = expand_rows(input);
    let expended_columns = expand_columns(input);
    (expended_rows, expended_columns)
}

fn expand_rows(input: &str) -> HashSet<usize> {
    let mut expended_rows = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        if line.chars().all(|c| c == '.') {
            println!("expended row: {}", i);
            expended_rows.insert(i);
        }
    }
    expended_rows
}

fn expand_columns(input: &str) -> HashSet<usize> {
    let mut columns: BTreeMap<usize, String> = BTreeMap::new();

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            columns.entry(i).or_default().push(c);
        }
    }

    let mut expended_columns = HashSet::new();

    for (i, column) in columns {
        if column.chars().all(|c| c == '.') {
            println!("expended column: {}", i);
            expended_columns.insert(i);
        }
    }

    expended_columns
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

        assert_eq!("1030", process(input)?);
        Ok(())
    }
}
