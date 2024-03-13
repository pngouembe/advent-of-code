use std::collections::VecDeque;

use nom::{character::complete, IResult};

use crate::custom_error::AocError;

// TODO: Change for a more suitable type
type Box = VecDeque<(String, u32)>;

#[derive(Debug)]
enum Instruction {
    Add(String, u32),
    Remove(String),
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let instructions = input.trim().split(',');

    let result: u32 = instructions
        .map(|instruction| {
            let (_, (instruction, label)) =
                parse_instruction(instruction).expect("Invalid instruction");

            let box_number = hash_function(&label);

            (box_number, instruction)
        })
        .fold(
            vec![Box::new(); 256],
            |mut boxes, (box_number, instruction)| {
                let lens_box = boxes.get_mut(box_number as usize).unwrap();

                match instruction {
                    Instruction::Remove(label) => {
                        if let Some(i) = lens_box.iter().position(|(l, _)| l == &label) {
                            lens_box.remove(i);
                        }
                    }
                    Instruction::Add(label, focal_length) => {
                        if let Some(i) = lens_box.iter().position(|(l, _)| l == &label) {
                            lens_box[i] = (label, focal_length);
                        } else {
                            lens_box.push_back((label, focal_length));
                        }
                    }
                }

                boxes
            },
        )
        .iter()
        .enumerate()
        .map(|(i, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(j, (_, focal_length))| (i as u32 + 1) * (j as u32 + 1) * focal_length)
                .sum::<u32>()
        })
        .sum();

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

fn parse_instruction(instruction: &str) -> IResult<&str, (Instruction, String)> {
    let (instruction, label) = complete::alpha1(instruction)?;
    let (instruction, operation) = complete::one_of("=-")(instruction)?;

    let result = match operation {
        '-' => Instruction::Remove(label.to_string()),
        '=' => {
            let (_, focal_length) = complete::u32(instruction)?;
            Instruction::Add(label.to_string(), focal_length)
        }
        _ => unreachable!(),
    };

    Ok(("", (result, label.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input)?);
        Ok(())
    }
}
