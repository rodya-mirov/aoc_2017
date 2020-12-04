use std::ops::BitXor;

mod parsing {
    use nom::{
        branch::alt,
        character::complete::{char, digit1},
        combinator::map,
        sequence::pair,
        IResult,
    };

    pub fn parse_i64(input: &str) -> IResult<&str, i64> {
        alt((
            map(pair(char('-'), digit1), |(_, s): (_, &str)| {
                -s.parse::<i64>().unwrap()
            }),
            map(digit1, |s: &str| s.parse::<i64>().unwrap()),
        ))(input)
    }

    pub fn parse_i32(input: &str) -> IResult<&str, i32> {
        alt((
            map(pair(char('-'), digit1), |(_, s): (_, &str)| {
                -s.parse::<i32>().unwrap()
            }),
            map(digit1, |s: &str| s.parse::<i32>().unwrap()),
        ))(input)
    }

    pub fn parse_usize(input: &str) -> IResult<&str, usize> {
        map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
    }
}

pub use parsing::{parse_i32, parse_i64, parse_usize};

fn str_to_bytes_salted(input: &str) -> Vec<u8> {
    let mut out = input.as_bytes().to_vec();
    // Add the silly suffix
    [17, 31, 73, 47, 23]
        .iter()
        .copied()
        .for_each(|u| out.push(u));
    out
}

pub fn compute_knot_hash(input: &str) -> String {
    let input = str_to_bytes_salted(input);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_10b() {
        assert_eq!(
            str_to_bytes_salted("1,2,3"),
            vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23]
        );
    }
}
