struct Policy {
    from: usize,
    to: usize,
    letter: char,
    password: String,
}

impl Policy {
    fn from(input: &str) -> Policy {
        let (rule, pwd) = input.split_once(": ").unwrap();
        let (rng, c) = rule.split_once(" ").unwrap();
        let (from, to) = rng.split_once("-").unwrap();
        Policy {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
            letter: c.chars().nth(0).unwrap(),
            password: pwd.to_string(),
        }
    }

    fn valid(&self) -> bool {
        let n = self.password.matches(self.letter).count();
        n >= self.from && n <= self.to
    }

    fn valid_positions(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let a = chars[self.from - 1] == self.letter;
        let b = chars[self.to - 1] == self.letter;
        a != b
    }
}

pub fn part1(input: &str) -> usize {
    input.lines().filter(|&l| Policy::from(l).valid()).count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|&l| Policy::from(l).valid_positions())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2020/02.txt").unwrap();
        assert_eq!(part1(&input), 614);
        assert_eq!(part2(&input), 354);
    }
}
