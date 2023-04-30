fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1(input: &str) -> usize {
    let mut jolts = parse(input);
    jolts.sort();
    let (one, three) = jolts
        .windows(2)
        .map(|c| c[1] - c[0])
        .fold((1, 1), |(a, b), c| match c {
            3 => (a, b + 1),
            _ => (a + 1, b),
        });
    one * three
}

pub fn part2(input: &str) -> usize {
    let mut jolts = parse(input);
    jolts.push(0);
    jolts.sort();
    jolts.push(3 + jolts[jolts.len() - 1]);

    let groups = jolts
        .windows(2)
        .map(|w| w[1] - w[0])
        .fold((vec![], 0 as usize), |(mut v, c), n| match n {
            1 => (v, c + 1),
            _ => {
                if c > 0 {
                    v.push(c);
                }
                (v, 0)
            }
        })
        .0;

    groups
        .iter()
        .map(|n| match n {
            2 => 2,
            3 => 4,
            4 => 7,
            _ => 1,
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2020/10.txt").unwrap();
        assert_eq!(part1(&input), 2400);
        assert_eq!(part2(&input), 338510590509056);
    }
}
