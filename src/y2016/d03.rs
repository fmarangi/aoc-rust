struct Triangle(usize, usize, usize);

impl Triangle {
    fn new(mut sides: Vec<usize>) -> Self {
        sides.sort();
        Self(sides[0], sides[1], sides[2])
    }

    fn is_valid(&self) -> bool {
        self.0 + self.1 > self.2
    }
}

fn sides(l: &str) -> Vec<usize> {
    l.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(sides)
        .map(Triangle::new)
        .filter(|t| t.is_valid())
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(sides)
        .collect::<Vec<_>>()
        .windows(3)
        .step_by(3)
        .flat_map(|w| (0..3).map(|n| w.iter().map(|t| t[n]).collect()))
        .map(Triangle::new)
        .filter(|t| t.is_valid())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2016/03.txt").unwrap();
        assert_eq!(part1(&input), 917);
        assert_eq!(part2(&input), 1649);
    }
}
