const INPUT: &str = include_str!("input/25.txt");

use nom::lib::std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
struct TuringMachine {
    start_state: char,
    diagnostic_cutoff: usize,
    trans: HashMap<char, FullTrans>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct FullTrans {
    if_zero: Transition,
    if_one: Transition,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Transition {
    next_state: char,
    write_val: bool,
    move_val: isize,
}

mod parse {
    use super::{FullTrans, Transition, TuringMachine};

    use std::collections::HashMap;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, char as exact_char, newline, satisfy},
        combinator::{eof, map},
        multi::{fold_many1, many0},
        IResult,
    };

    use crate::lib::parse_usize;

    fn start_state_line(input: &str) -> IResult<&str, char> {
        let (input, _) = tag("Begin in state ")(input)?;
        let (input, c) = anychar(input)?;
        let (input, _) = tag(".")(input)?;
        let (input, _) = newline(input)?;
        Ok((input, c))
    }

    fn diagnostic_state_line(input: &str) -> IResult<&str, usize> {
        let (input, _) = tag("Perform a diagnostic checksum after ")(input)?;
        let (input, steps) = parse_usize(input)?;
        let (input, _) = tag(" steps.")(input)?;
        let (input, _) = newline(input)?;
        Ok((input, steps))
    }

    fn parse_write(input: &str) -> IResult<&str, bool> {
        alt((
            map(exact_char('1'), |_| true),
            map(exact_char('0'), |_| false),
        ))(input)
    }

    fn parse_move(input: &str) -> IResult<&str, isize> {
        alt((map(tag("left"), |_| -1), map(tag("right"), |_| 1)))(input)
    }

    fn parse_write_line(input: &str) -> IResult<&str, bool> {
        let (input, _) = tag("    - Write the value ")(input)?;
        let (input, write) = parse_write(input)?;
        let (input, _) = tag(".\n")(input)?;
        Ok((input, write))
    }

    fn parse_move_line(input: &str) -> IResult<&str, isize> {
        let (input, _) = tag("    - Move one slot to the ")(input)?;
        let (input, out) = parse_move(input)?;
        let (input, _) = tag(".\n")(input)?;
        Ok((input, out))
    }

    fn parse_trans_line(input: &str) -> IResult<&str, char> {
        let (input, _) = tag("    - Continue with state ")(input)?;
        let (input, c) = anychar(input)?;
        let (input, _) = tag(".\n")(input)?;
        Ok((input, c))
    }

    fn parse_transition(input: &str) -> IResult<&str, (char, FullTrans)> {
        let (input, _) = tag("In state ")(input)?;
        let (input, c) = anychar(input)?;
        let (input, _) = tag(":\n")(input)?;

        let (input, _) = tag("  If the current value is 0:\n")(input)?;
        let (input, write_0) = parse_write_line(input)?;
        let (input, move_0) = parse_move_line(input)?;
        let (input, state_0) = parse_trans_line(input)?;

        let (input, _) = tag("  If the current value is 1:\n")(input)?;
        let (input, write_1) = parse_write_line(input)?;
        let (input, move_1) = parse_move_line(input)?;
        let (input, state_1) = parse_trans_line(input)?;

        let (input, _) = many0(satisfy(|c| c.is_whitespace()))(input)?;

        Ok((
            input,
            (
                c,
                FullTrans {
                    if_zero: Transition {
                        next_state: state_0,
                        write_val: write_0,
                        move_val: move_0,
                    },
                    if_one: Transition {
                        next_state: state_1,
                        write_val: write_1,
                        move_val: move_1,
                    },
                },
            ),
        ))
    }

    fn parse_helper(input: &str) -> IResult<&str, TuringMachine> {
        let (input, start_state) = start_state_line(input)?;
        let (input, steps) = diagnostic_state_line(input)?;
        let (input, _) = newline(input)?;

        // TODO: this parse fails if input doesn't have a trailing newline;
        // I had to modify the input and the sample, which feels icky

        let (input, map): (&str, HashMap<char, FullTrans>) =
            fold_many1(parse_transition, HashMap::new(), |mut acc, (c, ft)| {
                acc.insert(c, ft);
                acc
            })(input)
            .unwrap();

        let (_, _) = eof(input)?;

        Ok((
            "",
            TuringMachine {
                start_state,
                diagnostic_cutoff: steps,
                trans: map,
            },
        ))
    }

    pub(super) fn parse(input: &str) -> TuringMachine {
        let (_, tm) = parse_helper(input).unwrap();
        tm
    }
}

struct TM {
    state: char,
    dp: isize,
    ones: HashSet<isize>,
}

fn run_25a_with_input(input: &str) -> usize {
    let tm_defn = parse::parse(input);

    let mut tm = TM {
        state: tm_defn.start_state,
        dp: 0,
        ones: HashSet::new(),
    };

    for _ in 0..tm_defn.diagnostic_cutoff {
        let my_pos = tm.dp;
        let my_data = tm.ones.contains(&my_pos);

        let ft = tm_defn
            .trans
            .get(&tm.state)
            .expect("States should be defined");
        let my_trans = if my_data { &ft.if_one } else { &ft.if_zero };

        if my_trans.write_val {
            tm.ones.insert(my_pos);
        } else {
            tm.ones.remove(&my_pos);
        }

        tm.state = my_trans.next_state;
        tm.dp += my_trans.move_val;
    }

    tm.ones.len()
}

pub fn run_25a() -> usize {
    run_25a_with_input(INPUT)
}

pub fn run_25b() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.
";

    #[test]
    fn sample_25a() {
        assert_eq!(run_25a_with_input(SAMPLE_INPUT), 3);
    }
}
