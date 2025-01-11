fn main() {
    let input = include_str!("../input.txt");

    let (mut left, mut right) =
        input
            .lines()
            .fold((Vec::new(), Vec::new()), |(mut left, mut right), line| {
                let (l, r) = line.split_once("   ").unwrap();
                left.push(l.parse::<isize>().unwrap());
                right.push(r.parse::<isize>().unwrap());
                (left, right)
            });

    left.sort();
    right.sort();

    let sum = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (left, right)| acc + (left - right).abs() as usize);

    dbg!(sum);
}
