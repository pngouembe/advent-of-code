use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let instructions = input.trim().split(',');

    let result: u32 = instructions.map(hash_function).sum();

    Ok(result.to_string())
}

fn hash_function(input: &str) -> u32 {
    input.chars().fold(0, |mut current_value, c| {
        current_value += c as u32;
        current_value *= 17;

        current_value %= 256;
        current_value
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input)?);
        Ok(())
    }
}
