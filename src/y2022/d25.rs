const BASE: isize = 5;

pub fn from_snafu(num: &str) -> isize {
    num.chars().fold(0, |v, c| match c {
                '=' => -2,
                '-' => -1,
                _ => c.to_digit(10).unwrap() as isize,
            } + v*5)
}

pub fn to_snafu(mut value: isize) -> String {
    let mut result = String::new();
    while value > 0 {
        match value % BASE {
            3 => {
                result = format!("={}", result);
                value += BASE
            }
            4 => {
                result = format!("-{}", result);
                value += BASE
            }
            v => result = format!("{}{}", v, result),
        }
        value /= BASE;
    }
    result
}

pub fn part1(input: &str) -> String {
    to_snafu(input.split_ascii_whitespace().map(from_snafu).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_snafu() {
        assert_eq!(from_snafu("20"), 10);
        assert_eq!(from_snafu("2="), 8);
        assert_eq!(from_snafu("1=11-2"), 2022);

        assert_eq!(to_snafu(2022), "1=11-2")
    }

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2022/25.txt").unwrap();
        assert_eq!(part1(&input), "122-2=200-0111--=200");
    }
}
