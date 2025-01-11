use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let (mut left, right) = input.lines().fold(
        (Vec::new(), HashMap::new()),
        |(mut left, mut right), line| {
            let (l, r) = line.split_once("   ").unwrap();
            left.push(l.parse::<isize>().unwrap());

            let count = right.entry(r.parse::<isize>().unwrap()).or_default();
            *count += 1;

            (left, right)
        },
    );

    left.sort();

    let result = left.iter().fold(0, |acc, number| {
        acc + number * right.get(number).unwrap_or(&0)
    });

    dbg!(result);
}
