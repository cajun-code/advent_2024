advent_of_code::solution!(3);
use regex::Regex;
pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<left>\d*),(?<right>\d*)\)").unwrap();
    let mut total: u32 = 0;
    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()){
        total += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
    }
    Some(total)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, Some(48));
        assert_eq!(result, None);
    }
}
