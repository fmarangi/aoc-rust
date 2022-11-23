fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn find_sum(sum: usize, nums: &[usize]) -> bool {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == sum {
                return true;
            }
        }
    }
    false
}

pub fn part1(input: &str) -> usize {
    let nums = parse_input(&input);
    let mut n = 0;
    for i in 25..nums.len() {
        n = nums[i];
        if !find_sum(n, &nums[(i - 25)..i]) {
            break;
        }
    }
    n
}

fn part2(input: &str) -> usize {
    let nums = parse_input(&input);
    let target = part1(&input);
    for i in 0..nums.len() {
        let mut sum = nums[i];
        for j in i + 1..nums.len() {
            sum += nums[j];
            if sum == target {
                let vals = &nums[i..j];
                let min = vals.iter().min().unwrap();
                let max = vals.iter().max().unwrap();
                return min + max;
            }

            if sum > target {
                break;
            }
        }
    }
    unreachable!("no result was found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2020/09.txt").unwrap();
        assert_eq!(part1(&input), 25918798);
        assert_eq!(part2(&input), 3340942);
    }
}
