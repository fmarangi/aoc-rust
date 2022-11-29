use std::str::FromStr;

struct Present(usize, usize, usize);

impl Present {
    fn wrapping_paper(&self) -> usize {
        3 * self.0 * self.1 + 2 * (self.1 * self.2 + self.2 * self.0)
    }

    fn ribbon(&self) -> usize {
        2 * (self.0 + self.1) + self.0 * self.1 * self.2
    }
}

impl FromStr for Present {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dims: Vec<usize> = s
            .splitn(3, 'x')
            .map(|d| d.parse().unwrap_or_default())
            .collect();

        dims.sort();
        match dims.len() {
            3 => Ok(Self(dims[0], dims[1], dims[2])),
            _ => Err(String::from("not a present")),
        }
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<Present>().unwrap())
        .map(|p| p.wrapping_paper())
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<Present>().unwrap())
        .map(|p| p.ribbon())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2015/02.txt").unwrap();
        assert_eq!(part1(&input), 1606483);
        assert_eq!(part2(&input), 3842356);
    }
}
