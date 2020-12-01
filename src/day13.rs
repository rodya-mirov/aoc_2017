const INPUT: &str = include_str!("input/13.txt");

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Scanner {
    depth: i32,
    range: i32,
}

mod parse {

    use super::Scanner;

    use nom::{bytes::complete::tag, character::complete::digit1, sequence::tuple, IResult};

    fn parse_num(input: &str) -> IResult<&str, i32> {
        let (input, out) = digit1(input)?;
        Ok((input, out.parse::<i32>().unwrap()))
    }

    fn parse_line(input: &str) -> IResult<&str, Scanner> {
        let (input, (depth, _, range)) = tuple((parse_num, tag(": "), parse_num))(input)?;

        Ok((input, Scanner { depth, range }))
    }

    pub(super) fn parse(input: &str) -> Vec<Scanner> {
        input
            .lines()
            .map(|line| match parse_line(line) {
                Ok(("", scanner)) => scanner,
                other => panic!("Parse issue: {:?}; line '{}'", other, line),
            })
            .collect()
    }
}

#[derive(Clone)]
struct ScannersState {
    scanners: Vec<Scanner>,
    scanner_pos: Vec<i32>,
    scanner_dir: Vec<i32>,
}

impl ScannersState {
    fn new(scanners: Vec<Scanner>) -> ScannersState {
        let scanner_pos: Vec<i32> = vec![0; scanners.len()];
        let scanner_dir: Vec<i32> = vec![1; scanners.len()]; // 1 for forward, -1 for backward

        ScannersState {
            scanners,
            scanner_dir,
            scanner_pos,
        }
    }

    fn move_scanners(&mut self) {
        for (scanner_ind, range) in self.scanners.iter().map(|s| s.range).enumerate() {
            let dir = self.scanner_dir[scanner_ind];
            let pos = self.scanner_pos[scanner_ind];

            if dir > 0 {
                if pos + 1 >= range {
                    self.scanner_dir[scanner_ind] = -1;
                }
            } else {
                if pos <= 0 {
                    self.scanner_dir[scanner_ind] = 1;
                }
            }

            self.scanner_pos[scanner_ind] += self.scanner_dir[scanner_ind];
        }
    }

    fn try_collision<F: FnMut(Scanner)>(&self, curr_depth: i32, mut on_collide: F) {
        for (scanner_ind, scanner) in self.scanners.iter().copied().enumerate() {
            if scanner.depth == curr_depth && self.scanner_pos[scanner_ind] == 0 {
                on_collide(scanner);
            }
        }
    }
}

fn run_13a_with_input(input: &str) -> i32 {
    let scanners = parse::parse(input);

    let max_depth = scanners.iter().map(|s| s.depth).max().unwrap_or(0);

    let mut state = ScannersState::new(scanners);

    let mut curr_depth = -1;
    let mut severity = 0;

    while curr_depth < max_depth {
        curr_depth += 1;

        state.try_collision(curr_depth, |scanner| severity += curr_depth * scanner.range);

        state.move_scanners();
    }

    severity
}

pub fn run_13a() -> i32 {
    run_13a_with_input(INPUT)
}

// TODO perf: this takes 3.2s to run and the answer is like 3.8m
// can probably improve it with M A T H but whatever
fn run_13b_with_input(input: &str) -> i32 {
    let scanners = parse::parse(input);

    let max_depth = scanners.iter().map(|s| s.depth).max().unwrap_or(0);

    let mut state = ScannersState::new(scanners);

    let mut wait = 0;

    loop {
        let mut start_state = state.clone();
        let mut curr_depth = -1;
        let mut found = false;

        'sim: while curr_depth < max_depth {
            curr_depth += 1;

            start_state.try_collision(curr_depth, |_scanner| {
                found = true;
            });

            if found {
                break 'sim;
            }

            start_state.move_scanners();
        }

        if !found {
            return wait;
        }

        wait += 1;
        state.move_scanners();
    }
}

pub fn run_13b() -> i32 {
    run_13b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_13a() {
        let input = "0: 3
1: 2
4: 4
6: 4";

        assert_eq!(run_13a_with_input(input), 24);
    }

    #[test]
    fn sample_13b() {
        let input = "0: 3
1: 2
4: 4
6: 4";

        assert_eq!(run_13b_with_input(input), 10);
    }
}
