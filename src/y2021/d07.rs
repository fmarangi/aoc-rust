pub fn part1(input: &str) -> i32 {
    let mut crabs: Vec<i32> = input
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    crabs.iter().map(|c| (c - median).abs()).sum()
}

pub fn part2(input: &str) -> i32 {
    let crabs: Vec<i32> = input
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let sum: i32 = crabs.iter().sum();
    let n = crabs.len() as i32;
    let mean = |m: i32| crabs.iter().map(|c| (c - m).abs()).map(factorial).sum();
    let fuel: [i32; 2] = [sum / n, (sum + n - 1) / n].map(mean);

    fuel[0].min(fuel[1])
}

fn factorial(n: i32) -> i32 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2021/07.txt").unwrap();
        assert_eq!(part1(&input), 335271);
        assert_eq!(part2(&input), 95851339);
    }
}
