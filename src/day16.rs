use std::collections::HashMap;

const INPUT: &str = include_str!("input/16.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum DanceMove {
    Spin(usize),
    SwapPos(usize, usize),
    SwapName(char, char),
}

mod parse {
    use super::DanceMove;

    use nom::{
        branch::alt,
        character::complete::{anychar, char},
        combinator::map,
        sequence::tuple,
        IResult,
    };

    use crate::lib::parse_usize as parse_num;

    fn parse_dance_move(input: &str) -> IResult<&str, DanceMove> {
        let spin_parser = map(tuple((char('s'), parse_num)), |(_, val)| {
            DanceMove::Spin(val)
        });

        let swap_parser = map(
            tuple((char('x'), parse_num, char('/'), parse_num)),
            |(_, a, _, b)| DanceMove::SwapPos(a, b),
        );

        let part_parser = map(
            tuple((char('p'), anychar, char('/'), anychar)),
            |(_, a, _, b)| DanceMove::SwapName(a, b),
        );

        let mut full = alt((spin_parser, swap_parser, part_parser));
        full(input)
    }

    pub(super) fn parse(input: &str) -> Vec<DanceMove> {
        input
            .split(',')
            .map(|token| parse_dance_move(token).unwrap().1)
            .collect()
    }

    #[cfg(test)]
    mod parse_tests {
        use super::*;

        #[test]
        fn samples() {
            assert_eq!(
                parse("x1/2,pa/c,s12"),
                vec![
                    DanceMove::SwapPos(1, 2),
                    DanceMove::SwapName('a', 'c'),
                    DanceMove::Spin(12)
                ]
            );
        }
    }
}

fn process_move(dance_move: DanceMove, cohort: &mut [char]) {
    match dance_move {
        DanceMove::Spin(amt) => {
            cohort.rotate_right(amt);
        }
        DanceMove::SwapPos(pos_a, pos_b) => {
            let temp = cohort[pos_a];
            cohort[pos_a] = cohort[pos_b];
            cohort[pos_b] = temp;
        }
        DanceMove::SwapName(name_a, name_b) => {
            let mut a_ind = 0;
            let mut b_ind = 0;
            for (i, c) in cohort.iter().copied().enumerate() {
                if c == name_a {
                    a_ind = i;
                }
                if c == name_b {
                    b_ind = i;
                }
            }

            let temp = cohort[a_ind];
            cohort[a_ind] = cohort[b_ind];
            cohort[b_ind] = temp;
        }
    }
}

fn make_cohort(cohort_size: usize) -> Vec<char> {
    let mut cohort = Vec::with_capacity(cohort_size);
    let mut next_char = 'a' as u8;

    for _ in 0..cohort_size {
        cohort.push(next_char as char);
        next_char += 1;
    }

    cohort
}

fn cohort_str(cohort: &[char]) -> String {
    let mut out_str = String::with_capacity(cohort.len());
    for c in cohort.iter().copied() {
        out_str.push(c);
    }
    out_str
}

fn run_16a_with_input(input: &str, cohort_size: usize) -> String {
    let parsed = parse::parse(input);

    let mut cohort = make_cohort(cohort_size);

    for dance_move in parsed {
        process_move(dance_move, &mut cohort);
    }

    cohort_str(&cohort)
}

pub fn run_16a() -> String {
    run_16a_with_input(INPUT, 16)
}

fn run_16b_with_input(input: &str, cohort_size: usize) -> String {
    let moves = parse::parse(input);

    let mut seen: HashMap<Vec<char>, usize> = HashMap::new();

    let mut cohort = make_cohort(cohort_size);

    for loop_counter in 0..1_000_000_000 {
        if let Some(last_time) = seen.insert(cohort.clone(), loop_counter) {
            println!(
                "Saw a repeat; at ctr {} reach stage {}",
                loop_counter, last_time
            );

            let loop_length = loop_counter - last_time;
            let remaining = (1_000_000_000 - loop_counter) % loop_length;

            for _ in 0..remaining {
                for dm in &moves {
                    process_move(*dm, &mut cohort);
                }
            }

            return cohort_str(&cohort);
        }

        for dm in &moves {
            process_move(*dm, &mut cohort);
        }
    }

    // in practice this would never be reached, because you wouldn't let it run for a billion iterations
    cohort_str(&cohort)
}

pub fn run_16b() -> String {
    run_16b_with_input(INPUT, 16)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_16a() {
        let input = "s1,x3/4,pe/b";
        let actual = run_16a_with_input(input, 5);
        let expected = "baedc";

        assert_eq!(expected, &actual);
    }
}
