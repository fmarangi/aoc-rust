fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|n| n.parse().unwrap()).collect()
}

fn find_sum(nums: &[usize], sum: usize) -> Option<usize> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == sum {
                return Some(nums[i] * nums[j]);
            }
        }
    }
    None
}

pub fn part1(input: &str) -> usize {
    let mut solution = 0;
    if let Some(x) = find_sum(&parse_input(input), 2020) {
        solution = x;
    }
    solution
}

pub fn part2(input: &str) -> usize {
    let mut solution = 0;
    let nums = parse_input(&input);
    for &n in nums.iter() {
        if let Some(x) = find_sum(&nums, 2020 - n) {
            solution = n * x;
        }
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2020/01.txt").unwrap();
        assert_eq!(part1(&input), 731731);
        assert_eq!(part2(&input), 116115990);
    }
}
