fn parse(input: String) -> Vec<Vec<i8>> {
    let mut reports: Vec<Vec<i8>> = Vec::new();
    
    for line in input.lines() {
        reports.push(
            line.split(" ")
                .map(|x| x.parse::<i8>().unwrap())
                .collect());
    }

    reports
}

pub fn part1(input: String) -> String {
    let reports = parse(input);
    let mut safe_reports = 0;

    for report in reports {
        if is_report_safe(&report) {
            safe_reports += 1;
        }
    }

    safe_reports.to_string()
}

pub fn part2(input: String) -> String {
    let reports = parse(input);
    let mut safe_reports = 0;

    for report in reports {
        if is_report_safe(&report) {
            safe_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut report_copy = report.clone();
                report_copy.remove(i);
                if is_report_safe(&report_copy) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    safe_reports.to_string()
}

/// A line is safe when:
/// - All numbers in the line are sorted from largest to smallest, or smallest to largest
/// - The line doesn't contain any duplicate numbers
/// - The distance between two consecutive numbers is 3 or less
fn is_report_safe(report: &Vec<i8>) -> bool {
    let mut sorted_asc = report.clone();
    let mut sorted_desc = report.clone();
    sorted_asc.sort();
    sorted_desc.sort();
    sorted_desc.reverse();

    if *report == sorted_asc || *report == sorted_desc {
        if report.windows(2).all(|w| (1..=3).contains(&(w[0] - w[1]).abs())) {
            return true;
        }
    }
    return false;
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
        assert_eq!(String::from("4"), part2(parse(TEST_INPUT)));
    }
}
