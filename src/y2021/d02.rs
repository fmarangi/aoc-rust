struct Submarine {
    position: usize,
    depth: usize,
    aim: usize,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<(&str, usize)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            (parts[0], parts[1].parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let mut sub = Submarine::new();
    for (dir, q) in parse_input(&input) {
        match dir {
            "forward" => sub.position += q,
            "down" => sub.depth += q,
            "up" => sub.depth -= q,
            _ => (),
        }
    }
    sub.depth * sub.position
}

pub fn part2(input: &str) -> usize {
    let mut sub = Submarine::new();
    for (dir, q) in parse_input(&input) {
        match dir {
            "down" => sub.aim += q,
            "up" => sub.aim -= q,
            "forward" => {
                sub.position += q;
                sub.depth += sub.aim * q;
            }
            _ => (),
        }
    }
    sub.depth * sub.position
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2021/02.txt").unwrap();
        assert_eq!(part1(&input), 1936494);
        assert_eq!(part2(&input), 1997106066);
    }
}
