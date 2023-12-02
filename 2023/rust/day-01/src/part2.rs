use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines();
    let result = lines
        .map(|line| {
            let mut filtered_line = String::new();
            for index in 0..line.len() {
                let chunk = &line[index..];
                let first_char = chunk.chars().nth(0).expect("Chunk should not be empty");
                if first_char.is_ascii_digit() {
                    filtered_line.push(first_char)
                } else {
                    match chunk {
                        c if c.starts_with("one") => filtered_line.push('1'),
                        c if c.starts_with("two") => filtered_line.push('2'),
                        c if c.starts_with("three") => filtered_line.push('3'),
                        c if c.starts_with("four") => filtered_line.push('4'),
                        c if c.starts_with("five") => filtered_line.push('5'),
                        c if c.starts_with("six") => filtered_line.push('6'),
                        c if c.starts_with("seven") => filtered_line.push('7'),
                        c if c.starts_with("eight") => filtered_line.push('8'),
                        c if c.starts_with("nine") => filtered_line.push('9'),
                        _ => (),
                    }
                }
            }
            filtered_line
        })
        .map(|line| {
            let mut chars = line.chars();
            let first_digit = chars.nth(0).expect("Should be a number");
            let last_digit = match chars.nth_back(0) {
                Some(num) => num,
                None => first_digit,
            };
            format!("{}{}", first_digit, last_digit)
                .parse::<u32>()
                .expect("Should be valid numbers")
        })
        .sum::<u32>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
twone";
        assert_eq!("302", process(input)?);
        Ok(())
    }
}
