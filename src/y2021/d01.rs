fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .windows(4)
        .filter(|x| x[0] < x[3])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2021/01.txt").unwrap();
        assert_eq!(part1(&input), 1681);
        assert_eq!(part2(&input), 1704);
    }
}
