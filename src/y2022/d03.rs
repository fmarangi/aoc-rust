use std::collections::HashSet;

fn in_both_compartments(input: &str) -> char {
    let first: HashSet<char> = input[..input.len() / 2].chars().collect();
    input[input.len() / 2..]
        .chars()
        .find(|c| first.contains(c))
        .unwrap()
}

fn priority(c: char) -> u32 {
    match c as u32 {
        65..=90 => c as u32 - ('A' as u32) + 27,
        97..=122 => c as u32 - ('a' as u32) + 1,
        _ => 0,
    }
}

fn badge(group: &[&str]) -> char {
    group
        .iter()
        .map(|&g| g.chars().collect::<HashSet<char>>())
        .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<char>>())
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

pub fn part1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(in_both_compartments)
        .map(priority)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .windows(3)
        .step_by(3)
        .into_iter()
        .map(badge)
        .map(priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2022/03.txt").unwrap();
        assert_eq!(part1(&input), 7817);
        assert_eq!(part2(&input), 2444);
    }
}
