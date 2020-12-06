use std::collections::HashMap;

const INPUT: &str = include_str!("input/22.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

struct Grid {
    data: HashMap<(i32, i32), State>,
}

const DEFAULT: State = State::Clean;

impl Grid {
    fn get_state(&self, x: i32, y: i32) -> State {
        self.data.get(&(x, y)).copied().unwrap_or(DEFAULT)
    }

    fn set_state(&mut self, x: i32, y: i32, b: State) {
        if b == DEFAULT {
            self.data.remove(&(x, y));
        } else {
            self.data.insert((x, y), b);
        }
    }
}

fn parse(input: &str) -> Grid {
    let mut data: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(c == '#');
        }

        data.push(row);
    }

    let length = data.len();
    // no validation done, input must be correct and a square
    assert_eq!(length % 2, 1);

    let offset = ((length - 1) / 2) as i32;

    let mut data_map = HashMap::new();

    for (y, row) in data.into_iter().enumerate() {
        for (x, b) in row.into_iter().enumerate() {
            let state = if b { State::Infected } else { State::Clean };
            data_map.insert((x as i32 - offset, y as i32 - offset), state);
        }
    }

    Grid { data: data_map }
}

fn turn_left(dx: &mut i32, dy: &mut i32) {
    let (new_dx, new_dy) = match (*dx, *dy) {
        (-1, 0) => (0, 1),
        (0, -1) => (-1, 0),
        (1, 0) => (0, -1),
        (0, 1) => (1, 0),
        _ => unreachable!(),
    };
    *dx = new_dx;
    *dy = new_dy;
}

fn turn_right(dx: &mut i32, dy: &mut i32) {
    // it's lazy, whatever, sue me
    turn_left(dx, dy);
    turn_left(dx, dy);
    turn_left(dx, dy);
}

fn run_22a_with_input(input: &str, num_bursts: usize) -> usize {
    let mut grid = parse(input);

    let mut x = 0;
    let mut dx = 0;

    let mut y = 0;
    let mut dy = -1;

    let mut infections = 0;

    for _ in 0..num_bursts {
        match grid.get_state(x, y) {
            State::Clean => {
                grid.set_state(x, y, State::Infected);
                turn_left(&mut dx, &mut dy);
                infections += 1;
            }
            State::Infected => {
                grid.set_state(x, y, State::Clean);
                turn_right(&mut dx, &mut dy);
            }
            _ => {
                unreachable!()
            }
        }

        x += dx;
        y += dy;
    }

    infections
}

pub fn run_22a() -> usize {
    run_22a_with_input(INPUT, 10_000)
}

fn run_22b_with_input(input: &str, num_bursts: usize) -> usize {
    let mut grid = parse(input);

    let mut x = 0;
    let mut dx = 0;

    let mut y = 0;
    let mut dy = -1;

    let mut infections = 0;

    for _ in 0..num_bursts {
        match grid.get_state(x, y) {
            State::Clean => {
                grid.set_state(x, y, State::Weakened);
                turn_left(&mut dx, &mut dy);
            }
            State::Weakened => {
                grid.set_state(x, y, State::Infected);
                infections += 1;
            }
            State::Infected => {
                grid.set_state(x, y, State::Flagged);
                turn_right(&mut dx, &mut dy);
            }
            State::Flagged => {
                grid.set_state(x, y, State::Clean);
                turn_left(&mut dx, &mut dy);
                turn_left(&mut dx, &mut dy);
            }
        }

        x += dx;
        y += dy;
    }

    infections
}

pub fn run_22b() -> usize {
    run_22b_with_input(INPUT, 10_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_22a() {
        let input = "..#
#..
...";

        assert_eq!(run_22a_with_input(input, 7), 5);
        assert_eq!(run_22a_with_input(input, 70), 41);
        assert_eq!(run_22a_with_input(input, 10_000), 5587);
    }

    #[test]
    fn sample_22b() {
        let input = "..#
#..
...";

        assert_eq!(run_22b_with_input(input, 100), 26);
        assert_eq!(run_22b_with_input(input, 10000000), 2511944);
    }
}
