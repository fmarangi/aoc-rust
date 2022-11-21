fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn fuel(mass: usize) -> usize {
    let f = mass / 3;
    return if f > 2 { f - 2 } else { 0 };
}

fn total_fuel(mass: usize) -> usize {
    let mut tot = 0;
    let mut f = fuel(mass);
    while f > 0 {
        tot += f;
        f = fuel(f);
    }
    tot
}

pub fn part1(input: &str) -> usize {
    parse_input(input).iter().map(|&x| fuel(x)).sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input).iter().map(|&x| total_fuel(x)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_fuel() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn test_total_fuel() {
        assert_eq!(total_fuel(14), 2);
        assert_eq!(total_fuel(1969), 966);
        assert_eq!(total_fuel(100756), 50346);
    }

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2019/01.txt").unwrap();
        assert_eq!(part1(&input), 3381405);
        assert_eq!(part2(&input), 5069241);
    }
}
