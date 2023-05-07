use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point(i32, i32);

impl Point {
    fn step(&mut self, dir: char) -> &Self {
        match dir {
            '^' => self.1 += 1,
            '>' => self.0 += 1,
            'v' => self.1 -= 1,
            _ => self.0 -= 1,
        };
        self
    }
}

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .chars()
        .scan(Point(0, 0), |p, c| Some(*p.step(c)))
        .collect::<HashSet<Point>>()
        .len()
}

pub fn part2(input: &str) -> usize {
    input
        .trim()
        .chars()
        .enumerate()
        .scan([Point(0, 0), Point(0, 0)], |[a, b], (i, c)| match i % 2 {
            0 => Some([*a, *b.step(c)]),
            _ => Some([*a.step(c), *b]),
        })
        .flatten()
        .collect::<HashSet<Point>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2015/03.txt").unwrap();
        assert_eq!(part1(&input), 2572);
        assert_eq!(part2(&input), 2631);
    }
}
