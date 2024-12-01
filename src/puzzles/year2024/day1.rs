fn parse(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        if let Some((left, right)) = line.split_once("   ") {
            left_list.push(left.to_string().parse::<i32>().unwrap());
            right_list.push(right.to_string().parse::<i32>().unwrap());
        }
    }

    if left_list.len() != right_list.len() {
        panic!("Left list and right list aren't the same length. They should be!");
    }

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

pub fn part1(input: String) -> String {
    let (left_list, right_list) = parse(input);
    let mut distances = Vec::new();

    for i in 0..left_list.len() {
        distances.push((&left_list[i] - &right_list[i]).abs());    
    }

    let total_distance: i32 = distances.iter().sum();

    total_distance.to_string()
}

pub fn part2(input: String) -> String {
    let (left_list, right_list) = parse(input);
    let mut similarities = Vec::new();

    for i in 0..left_list.len() {
        let occurrences = right_list.iter().filter(|&n| *n == left_list[i]).count();
        similarities.push(left_list[i] * occurrences as i32);
    }

    let total_similarities: i32 = similarities.iter().sum();

    total_similarities.to_string()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(String::from("11"), part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(String::from("31"), part2(TEST_INPUT));
    }

 }
 