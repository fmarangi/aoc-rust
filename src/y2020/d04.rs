type Passport = Vec<(String, String)>;

fn passport(input: &str) -> Passport {
    let mut p: Passport = vec![];
    for d in input.split_ascii_whitespace() {
        if let Some((k, v)) = d.split_once(":") {
            p.push((k.to_string(), v.to_string()));
        }
    }
    p
}

fn between(x: &str, from: usize, to: usize) -> bool {
    let v: usize = x.parse().unwrap_or_default();
    v >= from && v <= to
}

fn all_data(pass: &Passport) -> bool {
    pass.iter().filter(|(k, _)| k != "cid").count() == 7
}

fn valid(pass: &Passport) -> bool {
    pass.iter()
        .find(|(k, v)| !match k.as_str() {
            "byr" => between(v, 1920, 2002),
            "iyr" => between(v, 2010, 2020),
            "eyr" => between(v, 2020, 2030),
            "hgt" => match &v[v.len() - 2..] {
                "cm" => between(&v[..v.len() - 2], 150, 193),
                "in" => between(&v[..v.len() - 2], 59, 76),
                _ => false,
            },
            "hcl" => {
                v.len() == 7
                    && v.starts_with('#')
                    && v[1..].chars().find(|c| !c.is_digit(16)).is_none()
            }
            "ecl" => match v.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            "pid" => v.len() == 9 && v.chars().find(|&c| !c.is_digit(10)).is_none(),
            _ => true,
        })
        .is_none()
}

pub fn part1(input: &str) -> usize {
    input.split("\n\n").map(passport).filter(all_data).count()
}

pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(passport)
        .filter(all_data)
        .filter(valid)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_puzzle() {
        let input = read_to_string("input/2020/04.txt").unwrap();
        assert_eq!(part1(&input), 239);
        assert_eq!(part2(&input), 188);
    }
}
