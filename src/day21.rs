const INPUT: &str = include_str!("input/21.txt");

type TwoCell = [[bool; 2]; 2];
type ThreeCell = [[bool; 3]; 3];
type FourCell = [[bool; 4]; 4];

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct TwoPattern(TwoCell, ThreeCell);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct ThreePattern(ThreeCell, FourCell);

trait GridMe: Sized {
    fn flip(&self) -> Self;
    fn rotate(&self) -> Self;
}

impl GridMe for TwoCell {
    fn flip(&self) -> Self {
        // a b  ->  b a
        // c d  ->  d c
        let [[a, b], [c, d]] = *self;
        [[b, a], [d, c]]
    }

    fn rotate(&self) -> Self {
        // a b  ->  c a
        // c d  ->  d b
        let [[a, b], [c, d]] = *self;
        [[c, a], [d, b]]
    }
}

impl GridMe for ThreeCell {
    fn flip(&self) -> Self {
        // a b c      c b a
        // d e f  ->  f e d
        // g h i      i h g
        let [[a, b, c], [d, e, f], [g, h, i]] = *self;
        [[c, b, a], [f, e, d], [i, h, g]]
    }

    fn rotate(&self) -> Self {
        // a b c      g d a
        // d e f  ->  h e b
        // g h i      i f c
        let [[a, b, c], [d, e, f], [g, h, i]] = *self;
        [[g, d, a], [h, e, b], [i, f, c]]
    }
}

fn all_rotations<T: GridMe + Copy>(t: T) -> [T; 8] {
    let t0 = t;
    let t1 = t0.rotate();
    let t2 = t1.rotate();
    let t3 = t2.rotate();
    [t0, t1, t2, t3, t0.flip(), t1.flip(), t2.flip(), t3.flip()]
}

trait MatchPattern: GridMe {
    type PatternType;
    type OutType;

    fn try_match(&self, pattern: Self::PatternType) -> Option<Self::OutType>;
}

impl MatchPattern for TwoCell {
    type PatternType = TwoPattern;
    type OutType = ThreeCell;

    fn try_match(&self, pattern: Self::PatternType) -> Option<Self::OutType> {
        for rot in all_rotations(*self).iter().copied() {
            if rot == pattern.0 {
                return Some(pattern.1);
            }
        }
        None
    }
}

impl MatchPattern for ThreeCell {
    type PatternType = ThreePattern;
    type OutType = FourCell;

    fn try_match(&self, pattern: Self::PatternType) -> Option<Self::OutType> {
        for rot in all_rotations(*self).iter().copied() {
            if rot == pattern.0 {
                return Some(pattern.1);
            }
        }
        None
    }
}

mod grid {
    use super::{ThreeCell, ThreePattern, TwoCell, TwoPattern};
    use crate::day21::{FourCell, MatchPattern};

    pub(super) struct Grid {
        // Assumes data.len() and data[i].len() are the same for all i
        data: Vec<Vec<bool>>,
    }

    impl Grid {
        pub(super) fn start() -> Grid {
            // .#.
            // ..#
            // ###
            Grid {
                data: vec![
                    vec![false, true, false],
                    vec![false, false, true],
                    vec![true, true, true],
                ],
            }
        }

        pub(super) fn num_lights(&self) -> usize {
            self.data
                .iter()
                .map(|row| row.iter().filter(|&&b| b).count())
                .sum()
        }

        pub(super) fn next(
            &self,
            two_patterns: &[TwoPattern],
            three_patterns: &[ThreePattern],
        ) -> Grid {
            if self.data.len() % 2 == 0 {
                self.next_twos(two_patterns)
            } else if self.data.len() % 3 == 0 {
                self.next_threes(three_patterns)
            } else {
                panic!("Length {} can't be fractalized", self.data.len());
            }
        }

        fn next_twos(&self, patterns: &[TwoPattern]) -> Grid {
            assert_eq!(self.data.len() % 2, 0);

            let old_len = self.data.len();
            let new_len = old_len / 2 * 3;

            let mut new_data: Vec<Vec<bool>> =
                (0..new_len).map(|_| Vec::with_capacity(new_len)).collect();

            for two_row in 0..(old_len / 2) {
                let top = self.data.get(two_row * 2).unwrap();
                let bot = self.data.get(two_row * 2 + 1).unwrap();

                for cell_idx in 0..(old_len / 2) {
                    let lef = cell_idx * 2;

                    let row_item: TwoCell = [[top[lef], top[lef + 1]], [bot[lef], bot[lef + 1]]];

                    let patt_match: ThreeCell = patterns
                        .iter()
                        .copied()
                        .flat_map(|pattern| row_item.try_match(pattern))
                        .next()
                        .expect("Something should match");

                    for y in 0..3 {
                        for x in 0..3 {
                            new_data[two_row * 3 + y].push(patt_match[y][x]);
                        }
                    }
                }
            }

            Grid { data: new_data }
        }

        fn next_threes(&self, patterns: &[ThreePattern]) -> Grid {
            assert_eq!(self.data.len() % 3, 0);

            let old_len = self.data.len();
            let new_len = old_len / 3 * 4;

            let mut new_data: Vec<Vec<bool>> =
                (0..new_len).map(|_| Vec::with_capacity(new_len)).collect();

            for two_row in 0..(old_len / 3) {
                let top = self.data.get(two_row * 3).unwrap();
                let mid = self.data.get(two_row * 3 + 1).unwrap();
                let bot = self.data.get(two_row * 3 + 2).unwrap();

                for cell_idx in 0..(old_len / 3) {
                    let lef = cell_idx * 3;

                    let row_item: ThreeCell = [
                        [top[lef], top[lef + 1], top[lef + 2]],
                        [mid[lef], mid[lef + 1], mid[lef + 2]],
                        [bot[lef], bot[lef + 1], bot[lef + 2]],
                    ];

                    let patt_match: FourCell = patterns
                        .iter()
                        .copied()
                        .flat_map(|pattern| row_item.try_match(pattern))
                        .next()
                        .expect("Something should match");

                    for y in 0..4 {
                        for x in 0..4 {
                            new_data[two_row * 4 + y].push(patt_match[y][x]);
                        }
                    }
                }
            }

            Grid { data: new_data }
        }
    }
}

use grid::Grid;

mod parse {
    use super::{ThreeCell, ThreePattern, TwoCell, TwoPattern};

    use crate::day21::FourCell;
    use std::convert::TryInto;

    fn to_bool(c: char) -> bool {
        match c {
            '.' => false,
            '#' => true,
            other => panic!("Unrecognized grid character '{}'", other),
        }
    }

    // panics aggressively on bad input
    fn two_cell(input: &str) -> TwoCell {
        input
            .split('/')
            .map(|row| {
                row.chars()
                    .map(to_bool)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn three_cell(input: &str) -> ThreeCell {
        input
            .split('/')
            .map(|row| {
                row.chars()
                    .map(to_bool)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn four_cell(input: &str) -> FourCell {
        input
            .split('/')
            .map(|row| {
                row.chars()
                    .map(to_bool)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    pub(super) fn parse(input: &str) -> (Vec<TwoPattern>, Vec<ThreePattern>) {
        let mut twos = Vec::new();
        let mut threes = Vec::new();

        for line in input.lines() {
            let [lhs, rhs]: [&str; 2] = line
                .split(" => ")
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();

            match lhs.len() {
                5 => {
                    // 2x2 is length 5, with the slash
                    assert_eq!(rhs.len(), 11, "Line {}", line);

                    let lhs_grid = two_cell(lhs);
                    let rhs_grid = three_cell(rhs);

                    twos.push(TwoPattern(lhs_grid, rhs_grid));
                }
                11 => {
                    // 3x3 is length 11, with the slash
                    assert_eq!(rhs.len(), 19, "Line {}", line);

                    let lhs_grid = three_cell(lhs);
                    let rhs_grid = four_cell(rhs);

                    threes.push(ThreePattern(lhs_grid, rhs_grid));
                }
                other => {
                    panic!("Impossible lhs length found: {}; line was {}", other, line);
                }
            }
        }

        (twos, threes)
    }
}

fn run_21a_with_input(input: &str, reps: usize) -> usize {
    let (two_patterns, three_patterns) = parse::parse(input);

    let mut grid = Grid::start();
    for _ in 0..reps {
        grid = grid.next(&two_patterns, &three_patterns);
    }

    grid.num_lights()
}

pub fn run_21a() -> usize {
    run_21a_with_input(INPUT, 5)
}

pub fn run_21b() -> usize {
    run_21a_with_input(INPUT, 18)
}

#[cfg(test)]
mod tests {
    use crate::day21::run_21a_with_input;

    const SAMPLE: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    #[test]
    fn sample_20a() {
        assert_eq!(run_21a_with_input(SAMPLE, 2), 12);
    }
}
