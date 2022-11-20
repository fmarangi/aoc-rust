fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '1').collect())
        .collect()
}

fn counts(report: &Vec<Vec<bool>>) -> Vec<usize> {
    (0..report[0].len()).map(|i| count_at(&report, i)).collect()
}

fn count_at(report: &Vec<Vec<bool>>, i: usize) -> usize {
    report.iter().fold(0, |a, b| a + (b[i] as usize))
}

fn gamma(cnt: &Vec<usize>, n: usize) -> usize {
    cnt.iter().fold(0, |a, &b| (a << 1) + (b > n) as usize)
}

fn epsilon(cnt: &Vec<usize>, n: usize) -> usize {
    cnt.iter().fold(0, |a, &b| (a << 1) + (b < n) as usize)
}

fn oxigen_rating(report: &Vec<Vec<bool>>) -> usize {
    let mut remaining = report.clone();
    let n = remaining[0].len();
    let mut q = (remaining.len() + 1) / 2;
    for i in 0..n {
        let on = count_at(&remaining, i) >= q;
        remaining = remaining.into_iter().filter(|x| x[i] == on).collect();
        if remaining.len() == 1 {
            return num_value(&remaining[0]);
        }
        q = (remaining.len() + 1) / 2;
    }
    0
}

fn co2_rating(report: &Vec<Vec<bool>>) -> usize {
    let mut remaining = report.clone();
    let n = remaining[0].len();
    let mut q = (remaining.len() - 1) / 2;
    for i in 0..n {
        let on = count_at(&remaining, i) <= q;
        remaining = remaining.into_iter().filter(|x| x[i] == on).collect();
        if remaining.len() == 1 {
            return num_value(&remaining[0]);
        }
        q = (remaining.len() - 1) / 2;
    }
    0
}

fn num_value(num: &Vec<bool>) -> usize {
    num.iter().fold(0, |a, &b| (a << 1) + (b as usize))
}

pub fn part1(input: &str) -> usize {
    let report = parse_input(&input);
    let cnt = counts(&report);
    let s = report.len() / 2;
    gamma(&cnt, s) * epsilon(&cnt, s)
}

pub fn part2(input: &str) -> usize {
    let report = parse_input(&input);
    oxigen_rating(&report) * co2_rating(&report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2021/03.txt").unwrap();
        assert_eq!(part1(&input), 2003336);
        assert_eq!(part2(&input), 1877139);
    }
}
