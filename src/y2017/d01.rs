fn parse_input(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

pub fn part1(input: &str) -> usize {
    let digits = parse_input(&input);
    let n = digits.len();
    digits
        .iter()
        .enumerate()
        .filter(|&(i, d)| *d == digits[(i + 1) % n])
        .map(|(_, d)| *d as usize)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let digits = parse_input(&input);
    let n = digits.len();
    digits
        .iter()
        .enumerate()
        .filter(|&(i, d)| *d == digits[(i + n / 2) % n])
        .map(|(_, d)| *d as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2017/01.txt").unwrap();
        assert_eq!(part1(&input), 1341);
        assert_eq!(part2(&input), 1348);
    }
}
