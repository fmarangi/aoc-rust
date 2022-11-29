pub fn part1(input: &str) -> i32 {
    input
        .trim()
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    1 + input
        .trim()
        .chars()
        .scan(0, |floor, c| {
            *floor += if c == '(' { 1 } else { -1 };
            Some(*floor)
        })
        .enumerate()
        .find(|&(_, floor)| floor == -1)
        .unwrap()
        .0 as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2015/01.txt").unwrap();
        assert_eq!(part1(&input), 232);
        assert_eq!(part2(&input), 1783);
    }
}
