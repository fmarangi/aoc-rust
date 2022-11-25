fn slope(grid: &[char], col: usize, row: usize) -> usize {
    let w = 1 + grid.iter().position(char::is_ascii_whitespace).unwrap();
    (0..grid.len())
        .step_by(w * row)
        .filter(|&i| grid[i + (i / w * col / row) % (w - 1)] == '#')
        .count()
}

pub fn part1(input: &str) -> usize {
    let grid: Vec<char> = input.chars().collect();
    slope(&grid, 3, 1)
}

pub fn part2(input: &str) -> usize {
    let grid: Vec<char> = input.chars().collect();
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(a, b)| slope(&grid, a, b))
        .reduce(|a, b| a * b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2020/03.txt").unwrap();
        assert_eq!(part1(&input), 280);
        assert_eq!(part2(&input), 4355551200);
    }
}
