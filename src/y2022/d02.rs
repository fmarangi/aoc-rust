use std::str::FromStr;

#[derive(Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match &self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play(&self, opponent: &Self) -> i32 {
        let result = self.score() - opponent.score();
        self.score()
            + (1 + match result.abs() {
                2 => result / -2,
                _ => result,
            }) * 3
    }

    fn mine(&self, outcome: &str) -> Self {
        match outcome {
            "X" => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
            "Z" => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            _ => self.clone(),
        }
    }
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("invalid".to_string()),
        }
    }
}

pub fn part1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .map(|round: Vec<Shape>| round[1].play(&round[0]))
        .sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(a, b)| {
            let o = a.parse::<Shape>().unwrap();
            o.mine(b).play(&o)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2022/02.txt").unwrap();
        assert_eq!(part1(&input), 12156);
        assert_eq!(part2(&input), 10835);
    }
}
