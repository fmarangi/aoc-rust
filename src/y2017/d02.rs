fn parse_row(input: &str) -> Vec<usize> {
    let mut nums: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    nums.sort();
    return nums;
}

fn find_divisible(nums: Vec<usize>) -> Result<usize, ()> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[j] % nums[i] == 0 {
                return Ok(nums[j] / nums[i]);
            }
        }
    }
    Err(())
}

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parse_row)
        .map(|n| n[n.len() - 1] - n[0])
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parse_row)
        .map(|nums| find_divisible(nums).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2017/02.txt").unwrap();
        assert_eq!(part1(&input), 36174);
        assert_eq!(part2(&input), 244);
    }
}
