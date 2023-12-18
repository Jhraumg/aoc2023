use std::fmt::{Display, Formatter, Write};

fn factory_hash(input: &str) -> usize {
    input.as_bytes().iter().fold(0, |hsh, c| ((hsh + *c as usize) * 17) % 256)
}

fn sum_instructions_hash(input: &str) -> usize {
    input.split(',').map(factory_hash).sum()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Lens {
    label: &'static str,
    focal: usize,
}

// TODO : use proper boxes
struct Boxes<'b>(&'b [Vec<Lens>]);
impl<'b> Display for Boxes<'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, b) in self.0.iter().enumerate() {
            if !b.is_empty() {
                f.write_fmt(format_args!("Box({i}):"))?;
                for l in b {
                    f.write_fmt(format_args!(" [{} {}]", l.label, l.focal))?;
                }
                f.write_char('\n')?;
            };
        }
        Ok(())
    }
}

fn sum_focusing_power(instructions: &'static str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = vec![Default::default(); 256];
    // let mut encoutered_lens: Vec<Lens> = vec![];
    for str_instruction in instructions.split(',') {
        // FIXME : define a proper Instruction struct
        let instruction = str_instruction.as_bytes();
        if instruction[instruction.len() - 1] as char == '-' {
            let label = &str_instruction[..str_instruction.len() - 1];
            let index = factory_hash(label);
            boxes[index].retain(|l| l.label != label);
        } else {
            let label = &str_instruction[..str_instruction.len() - 2];
            let index = factory_hash(label);
            let focal: usize = str_instruction[instruction.len() - 1..].parse().unwrap();
            // encoutered_lens.push(Lens { focal, label });
            if boxes[index].iter().any(|l| l.label == label) {
                boxes[index] = boxes[index]
                    .iter()
                    .map(|l| {
                        if l.label == label {
                            Lens { label, focal }
                        } else {
                            *l
                        }
                    })
                    .collect();
            } else {
                boxes[index].push(Lens { label, focal })
            }
        }
        // println!("\nafter '{str_instruction}':");
        // println!("{}",Boxes(&boxes));
    }

    boxes
        .into_iter()
        .map(|b| {
            b.into_iter()
                .enumerate()
                .map(|(i, l)| (1 + factory_hash(l.label)) * (i + 1) * l.focal)
                .sum::<usize>()
        })
        .sum()
}
pub fn init_factory() {
    let instructions = include_str!("../resources/day15_init_instructions.txt");
    let sum = sum_instructions_hash(instructions);

    println!("init sum {sum}");
    let total_power = sum_focusing_power(instructions);
    println!("total power {total_power}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc_axemplae_works() {
        assert_eq!(52, factory_hash("HASH"));
        assert_eq!(30, factory_hash("rn=1"));
        assert_eq!(253, factory_hash("cm-"));
        assert_eq!(
            1320,
            sum_instructions_hash("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        );

        assert_eq!(0, factory_hash("rn"));
        assert_eq!(3, factory_hash("pc"));

        assert_eq!(
            145,
            sum_focusing_power("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        );
    }
}
