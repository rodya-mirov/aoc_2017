use std::collections::HashMap;

const INPUT: &str = "312051";

fn get_ring(input: u64) -> u64 {
    if input <= 1 {
        return 0;
    }

    let mut ring_num = 0;

    loop {
        ring_num += 1;

        let n = ring_num * 2 + 1;
        if n * n >= input {
            return ring_num;
        }
    }
}

/// Technically O(n) where n is input; this could be improved with More Math
/// but who cares, input is like 6 digits, it takes a millisecond
fn run_3a_with_input(input: u64) -> u64 {
    if input <= 1 {
        return 0;
    }

    let ring = get_ring(input);

    // starts on right edge, at the next-to-the-bottom square
    let dist = |x: i64, y: i64| (x.abs() + y.abs()) as u64;

    let iring = ring as i64;
    let ring_adj = ring * 2 - 1;

    let start = ring_adj * ring_adj + 1;

    let mut n = start;

    let mut x: i64 = iring;
    let mut y: i64 = -iring + 1;

    // go up the right edge
    while y < iring {
        y += 1;
        n += 1;

        if n == input {
            return dist(x, y);
        }
    }

    // go along the top
    while x > -iring {
        x -= 1;
        n += 1;

        if n == input {
            return dist(x, y);
        }
    }

    // go along the left side
    while y > -iring {
        y -= 1;
        n += 1;

        if n == input {
            return dist(x, y);
        }
    }

    // go along the bottom
    while x < iring {
        x += 1;
        n += 1;

        if n == input {
            return dist(x, y);
        }
    }

    panic!("Didn't find the input, this shouldn't happen");
}

pub fn run_3a() -> u64 {
    run_3a_with_input(INPUT.parse().unwrap())
}

/// Sadly 3a does not extend nicely to 3b
/// This is still O(n) where n is input, but it uses a hashmap because I can't be bothered with math
fn run_3b_with_input(input: u64) -> u64 {
    let mut running: HashMap<(i32, i32), u64> = HashMap::new();

    running.insert((0, 0), 1);

    let adder = |x: i32, y: i32, map: &mut HashMap<(i32, i32), u64>| {
        let left = map.get(&(x - 1, y)).copied().unwrap_or(0);
        let right = map.get(&(x + 1, y)).copied().unwrap_or(0);
        let up = map.get(&(x, y - 1)).copied().unwrap_or(0);
        let down = map.get(&(x, y + 1)).copied().unwrap_or(0);
        let dl = map.get(&(x - 1, y + 1)).copied().unwrap_or(0);
        let dr = map.get(&(x + 1, y + 1)).copied().unwrap_or(0);
        let ul = map.get(&(x - 1, y - 1)).copied().unwrap_or(0);
        let ur = map.get(&(x + 1, y - 1)).copied().unwrap_or(0);

        let total = left + right + up + down + dl + dr + ul + ur;

        map.insert((x, y), total);

        total
    };

    let mut ring: u32 = 0;

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    loop {
        // increment the ring
        x += 1;
        ring += 1;

        let next = adder(x, y, &mut running);
        if next > input {
            return next;
        }

        let iring = ring as i32;
        // up the right edge
        while y > -iring {
            y -= 1;

            let next = adder(x, y, &mut running);
            if next > input {
                return next;
            }
        }

        // left the top edge
        while x > -iring {
            x -= 1;

            let next = adder(x, y, &mut running);
            if next > input {
                return next;
            }
        }

        // down the left edge
        while y < iring {
            y += 1;

            let next = adder(x, y, &mut running);
            if next > input {
                return next;
            }
        }

        // right the bottom edge
        while x < iring {
            x += 1;

            let next = adder(x, y, &mut running);
            if next > input {
                return next;
            }
        }
    }
}

pub fn run_3b() -> u64 {
    run_3b_with_input(INPUT.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    #[test]
    fn test_rings() {
        assert_eq!(get_ring(1), 0);
        for i in 2..10 {
            assert_eq!(get_ring(i), 1);
        }
        for i in 10..26 {
            assert_eq!(get_ring(i), 2);
        }

        for i in (5 * 5 + 1)..(7 * 7 + 1) {
            assert_eq!(get_ring(i), 3);
        }

        for i in (7 * 7 + 1)..(9 * 9 + 1) {
            assert_eq!(get_ring(i), 4);
        }
    }

    #[test]
    fn test_3a_samples() {
        assert_eq!(run_3a_with_input(1), 0);
        assert_eq!(run_3a_with_input(12), 3);
        assert_eq!(run_3a_with_input(23), 2);
        assert_eq!(run_3a_with_input(1024), 31);
    }

    #[test]
    fn test_3b_samples() {
        let fixtures: [(u64, u64); 9] = [
            (0, 1),
            (1, 2),
            (2, 4),
            (3, 4),
            (10, 11),
            (11, 23),
            (15, 23),
            (23, 25),
            (100, 122),
        ];

        for (input, expected) in fixtures.iter() {
            assert_eq!(run_3b_with_input(*input), *expected);
        }
    }
}
