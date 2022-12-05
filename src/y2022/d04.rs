use std::str::FromStr;

#[derive(Clone, Copy)]
struct Elf(i32, i32);
struct Pair(Elf, Elf);

impl FromStr for Elf {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split('-')
            .map(|p| p.parse().unwrap())
            .collect::<Vec<i32>>();

        if parts.len() == 2 {
            return Ok(Elf(parts[0], parts[1]));
        }
        Err(format!("{} is not a valid elf", s))
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect::<Vec<Elf>>();

        if parts.len() == 2 {
            return Ok(Pair(parts[0], parts[1]));
        }
        Err(format!("{} is not a valid pair", s))
    }
}

impl Pair {
    fn fully_contains(&self) -> bool {
        (self.0 .0 <= self.1 .0 && self.0 .1 >= self.1 .1)
            || (self.1 .0 <= self.0 .0 && self.1 .1 >= self.0 .1)
    }

    fn overlaps(&self) -> bool {
        !(self.0 .1 < self.1 .0 || self.1 .1 < self.0 .0)
    }
}

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|p| p.parse::<Pair>().unwrap())
        .filter(|p| p.fully_contains())
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|p| p.parse::<Pair>().unwrap())
        .filter(|p| p.overlaps())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2022/04.txt").unwrap();
        assert_eq!(part1(&input), 556);
        assert_eq!(part2(&input), 876);
    }
}
