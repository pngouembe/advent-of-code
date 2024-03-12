use std::vec;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let blocks = input.split("\n\n").collect::<Vec<&str>>();

    let (reflected_rows, reflected_columns) = blocks.iter().enumerate().fold(
        (vec![], vec![]),
        |(mut reflected_rows, mut reflected_columns), (i, block)| {
            println!("\nanalyzing block #{}", i + 1);
            reflected_rows.push(get_horizontal_reflection(block));
            reflected_columns.push(get_vertical_reflection(block));

            (reflected_rows, reflected_columns)
        },
    );

    let total_reflected_rows = reflected_rows.iter().sum::<u32>();

    let total_reflected_columns = reflected_columns.iter().sum::<u32>();

    let result = total_reflected_columns + (100 * total_reflected_rows);

    Ok(result.to_string())
}

fn get_horizontal_reflection(block: &str) -> u32 {
    println!("Looking for horizontal reflection");
    let lines = block.lines().collect::<Vec<_>>();

    let possible_reflections_id =
        lines
            .windows(2)
            .enumerate()
            .filter_map(|(i, rows)| if rows[0] == rows[1] { Some(i) } else { None });

    for id in possible_reflections_id {
        println!("id: {}", id + 1);
        let below_rows = lines[id + 2..].to_vec();
        let above_rows = lines[..id].to_vec();

        if below_rows
            .iter()
            .zip(above_rows.iter().rev())
            .inspect(|(a, b)| {
                println!("a: {}, b: {}", a, b);
            })
            .all(|(a, b)| a == b)
        {
            println!("Found reflection at {}", id + 1);
            return id as u32 + 1;
        }
    }

    0
}

fn get_vertical_reflection(block: &str) -> u32 {
    println!("Looking for vertical reflection");
    let transposed_block = transpose(block);

    get_horizontal_reflection(&transposed_block.join("\n"))
}

fn transpose(block: &str) -> Vec<String> {
    let lines = block.lines().collect::<Vec<_>>();

    let mut transposed_block = vec![];

    for i in 0..lines[0].len() {
        let mut row = String::new();
        for line in &lines {
            row.push(line.chars().nth(i).unwrap());
        }
        transposed_block.push(row);
    }

    transposed_block
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!("405", process(input)?);
        Ok(())
    }
}
