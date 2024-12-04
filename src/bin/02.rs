advent_of_code::solution!(2);

#[derive(PartialEq)]
enum Direction {
    Increment,
    Decrement,
    NoChange
}
enum Status{
    Safe(Direction),
    Unsafe(Direction)
}

fn calculate_safety(previous: &u32, previous_direction: &Direction, current: &u32) -> Status {
    let mut current_direction = Direction::NoChange;
    let mut report: bool = false;
    if previous > current {
        current_direction = Direction::Decrement;
    } else if previous < current {
        current_direction = Direction::Increment;
    }else{
        current_direction = Direction::NoChange;
    }
    if *previous_direction != Direction::NoChange && current_direction != *previous_direction {
        report = true;
    }
    let diff = previous.abs_diff(*current);
    //println!("previous: {} current: {} diff: {}", previous, current, diff);
    if diff < 1 || diff > 3 {
        report = true;
    }
    if report {
        Status::Unsafe(current_direction)
    } else {
        Status::Safe(current_direction)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_level_count :u32 = 0;
    input.split("\n").for_each(|report|{
        let mut current_direction = Direction::NoChange;
        let mut unsafe_report: bool = false;
        let mut cursor = report.split(" ");
        let mut previous :u32 = 0;
        let mut previous_direction = Direction::NoChange;

        while let level = cursor.next() {
            if level == None {break;}
            let current :u32 = level.unwrap().parse().unwrap_or(0);
            if previous == 0{
                previous = current;
                continue;
            }
            match calculate_safety(&previous, &previous_direction, &current) {
                Status::Safe(direction) => {
                    unsafe_report = false;
                    previous_direction = direction;
                },
                Status::Unsafe(_direction) => {
                    unsafe_report = true;
                    //current_direction = direction;
                    break;
                }
            }
            
            previous = current;
            //previous_direction = current_direction;
        }
        //println!("report: {} unsafe: {}", report, unsafe_report);
        if !unsafe_report && report.len() > 0 {
            safe_level_count += 1;
        }
    });
    Some(safe_level_count)
}



pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_level_count :u32 = 0;
    input.split("\n").for_each(|report|{
        
        let mut unsafe_report: bool = false;
        let mut cursor = report.split(" ");
        let mut previous :u32 = 0;
        let mut previous_direction = Direction::NoChange;
        let mut bypass :bool = false;
        while let level = cursor.next() {
            if level == None {break;}
            let current :u32 = level.unwrap().parse().unwrap_or(0);
            if previous == 0{
                previous = current;
                continue;
            }
            
            match calculate_safety(&previous, &previous_direction, &current) {
                Status::Safe(direction) => {
                    unsafe_report = false;
                    previous_direction = direction;
                },
                Status::Unsafe(_direction) => {
                    if bypass {
                        unsafe_report = true;
                        break;
                    }
                    bypass = true;
                }
            }
            if !bypass{
                previous = current;
                //previous_direction = current_direction;
            }
        }
        println!("report: {} unsafe: {}", report, unsafe_report);
        if !unsafe_report && report.len() > 0 {
            safe_level_count += 1;
        }
    });
    Some(safe_level_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
