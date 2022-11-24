fn seat_id(seat: &str) -> usize {
    seat.chars().fold(0, |res, c| match c {
        'B' | 'R' => res << 1 | 1,
        _ => res << 1,
    })
}

pub fn part1(input: &str) -> usize {
    input.lines().map(seat_id).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut seats: Vec<usize> = input.lines().map(seat_id).collect();
    seats.sort();
    seats.windows(2).find(|w| w[1] - w[0] != 1).unwrap()[0] + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2020/05.txt").unwrap();
        assert_eq!(part1(&input), 938);
        assert_eq!(part2(&input), 696);
    }
}
