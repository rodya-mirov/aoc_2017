use nom::lib::std::collections::VecDeque;
use std::collections::HashSet;

const INPUT: &str = include_str!("input/12.txt");

#[derive(Clone, Eq, PartialEq, Debug)]
struct Pipe {
    source: usize,
    connections: Vec<usize>,
}

mod parse {

    use super::Pipe;
    use nom::{
        bytes::complete::tag, character::complete::digit1, multi::separated_list1, IResult, Parser,
    };

    fn parse_pipe(input: &str) -> IResult<&str, Pipe> {
        let (input, source) = digit1(input)?;
        let source = source.parse::<usize>().unwrap(); // guaranteed to succeed

        let (input, _) = tag(" <-> ")(input)?;

        let (input, connections) = separated_list1(
            tag(", "),
            digit1.map(|token: &str| token.parse::<usize>().unwrap()),
        )(input)?;

        let pipe = Pipe {
            source,
            connections,
        };

        Ok((input, pipe))
    }

    pub(super) fn parse(input: &str) -> Vec<Pipe> {
        let mut pipes = Vec::new();

        for line in input.lines() {
            let (leftover, pipe) = parse_pipe(line).unwrap();
            if !leftover.is_empty() {
                panic!("Unused input: '{}'", leftover);
            }
            pipes.push(pipe);
        }

        pipes
    }
}

fn run_12a_with_input(input: &str) -> usize {
    let pipes = parse::parse(input);

    let mut seen = HashSet::new();
    let mut to_process = VecDeque::new();
    to_process.push_back(0);

    let mut network_count = 0;

    while let Some(next) = to_process.pop_front() {
        if !seen.insert(next) {
            continue;
        }

        network_count += 1;

        let pipe = pipes.get(next).unwrap();
        for connection in pipe.connections.iter().copied() {
            to_process.push_back(connection);
        }
    }

    network_count
}

pub fn run_12a() -> usize {
    run_12a_with_input(INPUT)
}

fn run_12b_with_input(input: &str) -> usize {
    let pipes = parse::parse(input);

    let mut to_make_connections: Vec<usize> = (0..pipes.len()).collect();
    let mut num_groups = 0;

    let mut seen = HashSet::new();

    while let Some(network_start) = to_make_connections.pop() {
        if seen.contains(&network_start) {
            continue;
        }

        num_groups += 1;

        let mut to_process = VecDeque::new();
        to_process.push_back(network_start);

        while let Some(next) = to_process.pop_front() {
            if !seen.insert(next) {
                continue;
            }

            let pipe = pipes.get(next).unwrap();
            for connection in pipe.connections.iter().copied() {
                to_process.push_back(connection);
            }
        }
    }

    num_groups
}

pub fn run_12b() -> usize {
    run_12b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_12a() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        assert_eq!(run_12a_with_input(input), 6);
    }

    #[test]
    fn sample_12b() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        assert_eq!(run_12b_with_input(input), 2);
    }
}
