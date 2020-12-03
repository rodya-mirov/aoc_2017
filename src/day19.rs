const INPUT: &str = include_str!("input/19.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Square {
    Empty,
    Corner,
    PipeVertical,
    PipeHorizontal,
    Letter(char),
}

impl Square {
    fn is_unidirectional(self) -> bool {
        match self {
            Square::Empty => false,
            Square::Corner => false,
            Square::PipeVertical => true,
            Square::PipeHorizontal => true,
            Square::Letter(_) => true,
        }
    }
}

struct Grid {
    data: Vec<Vec<Square>>,
    width: usize,
}

impl Grid {
    fn new(data: Vec<Vec<Square>>) -> Grid {
        let width = data.iter().map(|row| row.len()).max().unwrap();
        Grid { data, width }
    }

    // This is a little weird; as a quirk of parsing, the right (whitespace) edges of each line
    // have been trimmed off, so we just assume that if we're OOB, it was just empty space
    fn get(&self, x: usize, y: usize) -> Square {
        self.data
            .get(y)
            .map(|v| v.get(x).copied().unwrap_or(Square::Empty))
            .unwrap_or(Square::Empty)
    }
}

fn parse(input: &str) -> Grid {
    let rows = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    ' ' => Square::Empty,
                    '|' => Square::PipeVertical,
                    '-' => Square::PipeHorizontal,
                    '+' => Square::Corner,
                    letter => Square::Letter(letter),
                })
                .collect::<Vec<Square>>()
        })
        .collect();
    Grid::new(rows)
}

fn run_19a_with_input(input: &str) -> String {
    let grid = parse(input);

    let mut out = String::new();

    let start_x = (0..grid.width)
        .filter(|x| grid.get(*x, 0) == Square::PipeVertical)
        .next()
        .expect("Top row should have a vertical pipe");

    let mut pos = (start_x, 1);
    let mut dx = 0;
    let mut dy = 1;

    loop {
        let (x, y) = pos;
        match grid.get(x, y) {
            Square::Empty => {
                // This means we "got to the end" of the path and we're done
                return out;
            }
            Square::Corner => {
                if dy != 0 {
                    // then we're currently vertical, we need horizontal
                    // code assumes there is exactly one possible solution, does not validate
                    dy = 0;
                    if x > 0 && grid.get(x - 1, y).is_unidirectional() {
                        dx = -1;
                    } else {
                        dx = 1;
                    }
                } else {
                    dx = 0;
                    if y > 0 && grid.get(x, y - 1).is_unidirectional() {
                        dy = -1;
                    } else {
                        dy = 1;
                    }
                }
            }
            Square::PipeVertical | Square::PipeHorizontal => {
                // keep going
            }
            Square::Letter(letter) => {
                out.push(letter);
            }
        }
        pos.0 = (pos.0 as i32 + dx) as usize;
        pos.1 = (pos.1 as i32 + dy) as usize;
    }
}

pub fn run_19a() -> String {
    run_19a_with_input(INPUT)
}

fn run_19b_with_input(input: &str) -> usize {
    let grid = parse(input);

    let start_x = (0..grid.width)
        .filter(|x| grid.get(*x, 0) == Square::PipeVertical)
        .next()
        .expect("Top row should have a vertical pipe");

    let mut pos = (start_x, 1);
    let mut dx = 0;
    let mut dy = 1;

    let mut steps = 1;

    loop {
        let (x, y) = pos;
        match grid.get(x, y) {
            Square::Empty => {
                // This means we "got to the end" of the path and we're done
                return steps;
            }
            Square::Corner => {
                if dy != 0 {
                    // then we're currently vertical, we need horizontal
                    // code assumes there is exactly one possible solution, does not validate
                    dy = 0;
                    if x > 0 && grid.get(x - 1, y).is_unidirectional() {
                        dx = -1;
                    } else {
                        dx = 1;
                    }
                } else {
                    dx = 0;
                    if y > 0 && grid.get(x, y - 1).is_unidirectional() {
                        dy = -1;
                    } else {
                        dy = 1;
                    }
                }
            }
            Square::PipeVertical | Square::PipeHorizontal | Square::Letter(_) => {
                // keep going
            }
        }
        pos.0 = (pos.0 as i32 + dx) as usize;
        pos.1 = (pos.1 as i32 + dy) as usize;

        steps += 1;
    }
}

pub fn run_19b() -> usize {
    run_19b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
";

    #[test]
    fn sample_19a() {
        assert_eq!(run_19a_with_input(SAMPLE).as_str(), "ABCDEF");
    }

    #[test]
    fn sample_19b() {
        assert_eq!(run_19b_with_input(SAMPLE), 38);
    }
}
