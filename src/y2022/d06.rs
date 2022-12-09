use std::collections::HashSet;

fn find_marker(message: &str, length: usize) -> usize {
    message
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .windows(length)
        .map(|w| w.to_owned().into_iter().collect::<HashSet<char>>())
        .enumerate()
        .find(|(_, signal)| signal.len() == length)
        .unwrap()
        .0
        + length
}

pub fn part1(input: &str) -> usize {
    find_marker(&input, 4)
}

pub fn part2(input: &str) -> usize {
    find_marker(&input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2022/06.txt").unwrap();
        assert_eq!(part1(&input), 1235);
        assert_eq!(part2(&input), 3051);
    }
}
