use std::collections::HashSet;

const INPUT: &str = include_str!("input/4.txt");

fn is_valid_4a(line: &str) -> bool {
    let mut seen: HashSet<String> = HashSet::new();

    for token in line.split_whitespace() {
        if !seen.insert(token.to_string()) {
            return false;
        }
    }

    true
}

fn is_valid_4b(line: &str) -> bool {
    let mut seen: HashSet<String> = HashSet::new();

    for token in line.split_whitespace() {
        let mut chars: Vec<char> = token.chars().collect();
        chars.sort();

        if !seen.insert(chars.into_iter().collect()) {
            return false;
        }
    }

    true
}

pub fn run_4a() -> usize {
    INPUT.lines().filter(|line| is_valid_4a(line)).count()
}

pub fn run_4b() -> usize {
    INPUT.lines().filter(|line| is_valid_4b(line)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn samples_4a() {
        assert!(is_valid_4a("aa bb cc dd ee"));
        assert!(!is_valid_4a("aa bb cc dd aa"));
        assert!(is_valid_4a("aa bb cc dd aaa"));
    }

    #[test]
    pub fn samples_4b() {
        assert!(is_valid_4b("abcde fghij"));
        assert!(!is_valid_4b("abcde xyz ecdab"));
        assert!(is_valid_4b("a ab abc abd abf abj"));
        assert!(is_valid_4b("iiii oiii ooii oooi oooo"));
        assert!(!is_valid_4b("oiii ioii iioi iiio"));
    }
}
