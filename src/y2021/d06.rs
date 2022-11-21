fn parse_input(input: &str) -> [usize; 9] {
    let mut fish = [0usize; 9];
    for n in input.trim().split(",").map(|n| n.parse::<usize>().unwrap()) {
        fish[n] += 1;
    }
    fish
}

fn process(fish: &[usize; 9]) -> [usize; 9] {
    let mut next = [0usize; 9];
    for i in 0..9 {
        match i {
            6 => next[i] = fish[i + 1] + fish[0],
            8 => next[i] = fish[0],
            _ => next[i] = fish[i + 1],
        }
    }
    next
}

pub fn part1(input: &str) -> usize {
    let mut fish = parse_input(&input);
    for _ in 0..80 {
        fish = process(&fish);
    }
    fish.iter().sum()
}

pub fn part2(input: &str) -> usize {
    let mut fish = parse_input(&input);
    for _ in 0..256 {
        fish = process(&fish);
    }
    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2021/06.txt").unwrap();
        assert_eq!(part1(&input), 350917);
        assert_eq!(part2(&input), 1592918715629);
    }
}
