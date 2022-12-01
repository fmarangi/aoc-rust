fn elf(input: &str) -> usize {
    input
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .sum()
}

pub fn part1(input: &str) -> usize {
    input.trim().split("\n\n").map(elf).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut elves: Vec<usize> = input.trim().split("\n\n").map(elf).collect();
    elves.sort();
    elves[elves.len() - 3..].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2022/01.txt").unwrap();
        assert_eq!(part1(&input), 69836);
        assert_eq!(part2(&input), 207968);
    }
}
