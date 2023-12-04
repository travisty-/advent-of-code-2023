advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    for line in input.lines() {
        let mut digits = line.chars().filter_map(|char| char.to_digit(10));
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        result += (first * 10) + last;
    }
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
