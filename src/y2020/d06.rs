use std::collections::HashMap;
use std::collections::HashSet;

type Group = Vec<String>;

fn parse_input(input: &str) -> Vec<Group> {
    input
        .split("\n\n")
        .map(|g| g.split_whitespace().map(str::to_string).collect())
        .collect()
}

fn answers(group: &Group) -> usize {
    let mut ans: HashSet<char> = HashSet::new();
    for c in group.concat().chars() {
        ans.insert(c);
    }
    ans.len()
}

fn same_answers(group: &Group) -> usize {
    let n = group.len();
    let mut ans: HashMap<char, usize> = HashMap::new();
    for c in group.concat().chars() {
        let a = ans.entry(c).or_insert(0);
        *a += 1;
    }
    ans.values().filter(|&&a| a == n).count()
}

pub fn part1(input: &str) -> usize {
    parse_input(&input).iter().map(answers).sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(&input).iter().map(same_answers).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2020/06.txt").unwrap();
        assert_eq!(part1(&input), 7027);
        assert_eq!(part2(&input), 3579);
    }
}
