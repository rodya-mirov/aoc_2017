use std::ops::BitXor;

const INPUT: &str = "70,66,255,2,48,0,54,48,80,141,244,254,160,108,1,41";

fn parse_input_10a(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|token| token.parse::<usize>().unwrap())
        .collect()
}

fn run_10a_with_input(input: &str, knot_length: usize) -> usize {
    let input = parse_input_10a(input);

    let mut data: Vec<usize> = (0..knot_length).collect();

    let mut pos: usize = 0;
    let mut skip_size: usize = 0;
    for length in input {
        for i in 0..length / 2 {
            let j = length - i - 1;

            let a_ind = (pos + i) % knot_length;
            let b_ind = (pos + j) % knot_length;

            let temp = data[a_ind];
            data[a_ind] = data[b_ind];
            data[b_ind] = temp;
        }

        // reverse data[pos .. pos+length]
        pos = (pos + length + skip_size) % knot_length;
        skip_size += 1;
    }

    data[0] * data[1]
}

pub fn run_10a() -> usize {
    run_10a_with_input(INPUT, 256)
}

fn parse_input_10b(input: &str) -> Vec<u8> {
    let mut out = input.as_bytes().to_vec();
    // Add the silly suffix
    [17, 31, 73, 47, 23]
        .iter()
        .copied()
        .for_each(|u| out.push(u));
    out
}

fn run_10b_with_input(input: &str) -> String {
    let input = parse_input_10b(input);

    let mut data: Vec<u8> = (0..=255).collect();

    let mut pos: usize = 0;
    let mut skip_size: usize = 0;

    for _round in 0..64 {
        for length in input.iter().copied() {
            for i in 0..length / 2 {
                let j = length - i - 1;

                let a_ind = (pos + i as usize) % 256;
                let b_ind = (pos + j as usize) % 256;

                let temp = data[a_ind];
                data[a_ind] = data[b_ind];
                data[b_ind] = temp;
            }

            // reverse data[pos .. pos+length]
            pos = (pos + (length as usize) + skip_size) % 256;
            skip_size += 1;
        }
    }

    let sparse_hash = data;
    let mut dense_hash = Vec::with_capacity(16);

    for block in 0..16 {
        let mut hashed = 0;
        for b in block * 16..(block + 1) * 16 {
            hashed = hashed.bitxor(sparse_hash[b]);
        }
        dense_hash.push(hashed);
    }

    let mut out = String::new();

    for hashed in dense_hash {
        out.push_str(&format!("{:02x}", hashed));
    }

    out
}

pub fn run_10b() -> String {
    run_10b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_10a() {
        let actual = run_10a_with_input("3,4,1,5", 5);

        assert_eq!(actual, 12);
    }

    #[test]
    fn test_parse_10b() {
        assert_eq!(
            parse_input_10b("1,2,3"),
            vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23]
        );
    }

    #[test]
    fn sample_10b() {
        assert_eq!(run_10b_with_input(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(
            run_10b_with_input("AoC 2017"),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(
            run_10b_with_input("1,2,3"),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        assert_eq!(
            run_10b_with_input("1,2,4"),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
