use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut group1: Vec<u32> = Vec::new();
    let mut group2: Vec<u32> = Vec::new();
    let mut distance: u32 = 0;
    input.split("\n").for_each(|line|{
        if line.is_empty() {return;}
        let mut numbers = line.split("   ");
        group1.push(numbers.next().unwrap().trim().parse().unwrap());
        group2.push(numbers.next().unwrap().trim().parse().unwrap());
    });
    group1.sort();
    group2.sort();
    for (i, j) in group1.iter().zip(group2.iter()) {
        distance += i.abs_diff(*j);
        //println!(" i={} j={} distance={}", i, j, distance);
    }
    Some(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut group1: Vec<u32> = Vec::new();
    let mut group2: Vec<u32> = Vec::new();
    let mut similarity_score: u32 = 0;
    input.split("\n").for_each(|line|{
        if line.is_empty() {return;}
        let mut numbers = line.split("   ");
        group1.push(numbers.next().unwrap().trim().parse().unwrap());
        group2.push(numbers.next().unwrap().trim().parse().unwrap());
    });

    let group2_counts: HashMap<_,_> = group2.iter().fold(HashMap::new(), |mut acc, i| {
        *acc.entry(*i).or_insert(0) += 1;
        acc
    });

    group1.iter().for_each(|i|{
        let count = group2_counts.get(i).unwrap_or(&0);
        similarity_score += i * count;
    });
    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
