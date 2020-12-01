use crate::lib::compute_knot_hash;
use nom::lib::std::collections::HashSet;

const INPUT: &str = "hxtvlmkl";

// 128x128 grid
fn make_grid(key: &str) -> Vec<Vec<bool>> {
    (0..128)
        .map(|row: u8| {
            let hash_input = format!("{}-{}", key, row);
            let hash = compute_knot_hash(&hash_input);

            let mut row_bits = Vec::new();
            for c in hash.chars() {
                let next_bits: [bool; 4] = match c {
                    '0' => [false, false, false, false],
                    '1' => [false, false, false, true],
                    '2' => [false, false, true, false],
                    '3' => [false, false, true, true],
                    '4' => [false, true, false, false],
                    '5' => [false, true, false, true],
                    '6' => [false, true, true, false],
                    '7' => [false, true, true, true],
                    '8' => [true, false, false, false],
                    '9' => [true, false, false, true],
                    'a' => [true, false, true, false],
                    'b' => [true, false, true, true],
                    'c' => [true, true, false, false],
                    'd' => [true, true, false, true],
                    'e' => [true, true, true, false],
                    'f' => [true, true, true, true],
                    _ => unreachable!(),
                };

                for b in next_bits.iter().copied() {
                    row_bits.push(b);
                }
            }

            assert_eq!(row_bits.len(), 128);

            row_bits
        })
        .collect()
}

fn run_14a_with_input(input: &str) -> usize {
    let grid = make_grid(input);

    grid.into_iter()
        .map(|row| row.into_iter().filter(|b| *b).count())
        .sum()
}

pub fn run_14a() -> usize {
    run_14a_with_input(INPUT)
}

fn run_14b_with_input(input: &str) -> usize {
    const GRID_WIDTH: usize = 128;

    let grid: Vec<Vec<bool>> = make_grid(input);

    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let mut num_groups = 0;

    // for each filled square, if we haven't seen it before, process its whole component
    for (y, row) in grid.iter().enumerate() {
        for (x, b) in row.iter().copied().enumerate() {
            if !b || seen.contains(&(x, y)) {
                continue;
            }

            num_groups += 1;

            let mut to_process: Vec<(usize, usize)> = Vec::new();
            to_process.push((x, y));

            while let Some((x, y)) = to_process.pop() {
                if !seen.insert((x, y)) {
                    continue;
                }

                if x > 0 && grid[y][x - 1] {
                    to_process.push((x - 1, y));
                }
                if y > 0 && grid[y - 1][x] {
                    to_process.push((x, y - 1));
                }
                if x + 1 < GRID_WIDTH && grid[y][x + 1] {
                    to_process.push((x + 1, y));
                }
                if y + 1 < GRID_WIDTH && grid[y + 1][x] {
                    to_process.push((x, y + 1));
                }
            }
        }
    }

    num_groups
}

pub fn run_14b() -> usize {
    run_14b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_14a() {
        assert_eq!(run_14a_with_input("flqrgnkx"), 8108);
    }

    #[test]
    fn sample_14b() {
        assert_eq!(run_14b_with_input("flqrgnkx"), 1242);
    }
}
