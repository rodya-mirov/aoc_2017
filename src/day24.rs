use std::convert::TryInto;

const INPUT: &str = include_str!("input/24.txt");

fn parse(input: &str) -> Vec<[u32; 2]> {
    input
        .lines()
        .map(|line| {
            line.split("/")
                .map(|token| token.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn run_24a_with_input(input: &str) -> u32 {
    let bridges = parse(input);

    let mut used = vec![false; bridges.len()];

    fn dfs(opening_port: u32, bridges: &[[u32; 2]], used: &mut [bool]) -> u32 {
        let mut best = 0;
        for (i, [a, b]) in bridges.iter().copied().enumerate() {
            if used[i] {
                continue;
            }

            if a == opening_port {
                used[i] = true;
                let bridge = a + b + dfs(b, bridges, used);
                best = best.max(bridge);
                used[i] = false;
            }

            if b == opening_port {
                used[i] = true;
                let bridge = a + b + dfs(a, bridges, used);
                best = best.max(bridge);
                used[i] = false;
            }
        }
        best
    }

    dfs(0, &bridges, &mut used)
}

pub fn run_24a() -> u32 {
    run_24a_with_input(INPUT)
}

fn run_24b_with_input(input: &str) -> u32 {
    let bridges = parse(input);

    let mut used = vec![false; bridges.len()];

    fn dfs(opening_port: u32, bridges: &[[u32; 2]], used: &mut [bool]) -> (usize, u32) {
        let mut best = (0, 0);
        for (i, [a, b]) in bridges.iter().copied().enumerate() {
            if used[i] {
                continue;
            }

            if a == opening_port {
                used[i] = true;
                let (child_len, child_str) = dfs(b, bridges, used);
                let bridge = (child_len + 1, a + b + child_str);
                best = best.max(bridge);
                used[i] = false;
            }

            if b == opening_port {
                used[i] = true;
                let (child_len, child_str) = dfs(a, bridges, used);
                let bridge = (child_len + 1, a + b + child_str);
                best = best.max(bridge);
                used[i] = false;
            }
        }
        best
    }

    let (_len, strength) = dfs(0, &bridges, &mut used);
    strength
}

pub fn run_24b() -> u32 {
    run_24b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    #[test]
    fn sample_24a() {
        assert_eq!(run_24a_with_input(SAMPLE_INPUT), 31);
    }

    #[test]
    fn sample_24b() {
        assert_eq!(run_24b_with_input(SAMPLE_INPUT), 19);
    }
}
