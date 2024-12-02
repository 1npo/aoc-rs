use itertools::Itertools;

fn parse(input: String) -> Vec<Vec<i8>> {
    let mut reports: Vec<Vec<i8>> = Vec::new();

    for line in input.lines() {
        reports.push(line.split(" ")
                         .map(|x| x.parse::<i8>().unwrap())
                         .collect());
    }

    reports
}

/// A line is safe when:
/// - All numbers in the line are sorted from largest to smallest, or smallest to largest
/// - The line doesn't contain any duplicate numbers
/// - The distance between two consecutive numbers is 3 or less
pub fn part1(input: String) -> String {
    let reports = parse(input);
    let mut safe_reports = 0;

    for report in reports {
        let mut sorted_asc = report.clone();
        let mut sorted_desc = report.clone();
        sorted_asc.sort();
        sorted_desc.sort();
        sorted_desc.reverse();
        
        if report == sorted_asc || report == sorted_desc {
            if report.windows(2).all(|w| (1..4).contains(&(w[0] - w[1]).abs())) {
                safe_reports += 1;
            }
        }
    }

    safe_reports.to_string()
}

pub fn part2(input: String) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
    ";
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(String::from("2"), part1(parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(parse(TEST_INPUT)));
    }

}
