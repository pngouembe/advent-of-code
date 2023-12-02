use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines();
    let result = lines
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>())
        .map(|line| {
            let first_digit = line.first().expect("Should be a number");
            let last_digit = match line.last() {
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
