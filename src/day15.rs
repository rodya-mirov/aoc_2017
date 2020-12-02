const INPUT: &str = include_str!("input/15.txt");

const MODULUS: i64 = 2147483647;

struct Generator {
    val: i64,
    factor: i64,
}

impl Generator {
    fn next(&self) -> Generator {
        let next_val = (self.val * self.factor) % MODULUS;
        Generator {
            val: next_val,
            factor: self.factor,
        }
    }

    fn next_checked(&self, mask: i64) -> Generator {
        let mut gen = self.next();
        while gen.val & mask != 0 {
            gen = gen.next();
        }
        gen
    }
}

// lol so complicated to parse two lines
fn parse_input(input: &str) -> (Generator, Generator) {
    use nom::{bytes::complete::tag, character::complete::digit1, sequence::pair, IResult};

    fn parse_line<'a>(tag_str: &str, input: &'a str) -> IResult<&'a str, i64> {
        let (input, res) = pair(tag(tag_str), digit1)(input)?;

        let (_, digits) = res;
        let val = digits.parse().unwrap();

        assert!(input.is_empty());

        Ok((input, val))
    }

    let mut lines = input.lines();

    let line_a = lines.next().unwrap();
    let (_, val_a) = parse_line("Generator A starts with ", line_a).unwrap();

    let line_b = lines.next().unwrap();
    let (_, val_b) = parse_line("Generator B starts with ", line_b).unwrap();

    assert_eq!(lines.next(), None);

    (
        Generator {
            val: val_a,
            factor: 16807,
        },
        Generator {
            val: val_b,
            factor: 48271,
        },
    )
}

fn run_15a_with_input(input: &str) -> usize {
    const MASK: i64 = (1 << 16) - 1;

    let (mut a, mut b) = parse_input(input);

    let mut agreements = 0;

    for _ in 0..40_000_000 {
        let a_val = a.val & MASK;
        let b_val = b.val & MASK;

        if a_val == b_val {
            agreements += 1;
        }

        a = a.next();
        b = b.next();
    }

    agreements
}

pub fn run_15a() -> usize {
    run_15a_with_input(INPUT)
}

fn run_15b_with_input(input: &str) -> usize {
    const MASK: i64 = (1 << 16) - 1;

    const A_MASK: i64 = (1 << 2) - 1;
    const B_MASK: i64 = (1 << 3) - 1;

    let (mut a, mut b) = parse_input(input);

    let mut agreements = 0;

    for _ in 0..5_000_000 {
        let a_val = a.val & MASK;
        let b_val = b.val & MASK;

        if a_val == b_val {
            agreements += 1;
        }

        a = a.next_checked(A_MASK);
        b = b.next_checked(B_MASK);
    }

    agreements
}

pub fn run_15b() -> usize {
    run_15b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_15a() {
        let input = "Generator A starts with 65
Generator B starts with 8921";

        assert_eq!(run_15a_with_input(input), 588);
    }

    #[test]
    fn sample_15b() {
        let input = "Generator A starts with 65
Generator B starts with 8921";

        assert_eq!(run_15b_with_input(input), 309);
    }
}
