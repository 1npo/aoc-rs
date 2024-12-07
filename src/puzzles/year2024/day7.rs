use itertools::Itertools;

struct Calibration {
    test_result: u128,
    test_values: Vec<u128>,
}

fn parse(input: String) -> Vec<Calibration> {
    let mut calibrations: Vec<Calibration> = Vec::new();

    for line in input.lines() {
        let mut split_line = line.split(':');
        calibrations.push(Calibration {
            test_result: split_line
                .nth(0)
                .unwrap()
                .trim()
                .parse::<u128>()
                .ok()
                .unwrap(),
            test_values: split_line
                .nth(0)
                .unwrap()
                .split(' ')
                .filter_map(|v| v.trim().parse::<u128>().ok())
                .collect(),
        });
    }

    calibrations
}

pub fn part1(input: String) -> String {
    let calibrations: Vec<Calibration> = parse(input);
    count_calibrations(calibrations, vec!['*', '+'])
}

pub fn part2(input: String) -> String {
    let calibrations: Vec<Calibration> = parse(input);
    count_calibrations(calibrations, vec!['*', '+', '|'])
}

fn count_calibrations(calibrations: Vec<Calibration>, operators: Vec<char>) -> String{
    let mut calibration_results: u128 = 0;

    for calibration in calibrations {
        if let Some(result) = eval_calibration(calibration, &operators) {
            calibration_results += result;
        }
    }
    
    calibration_results.to_string()
}

fn eval_calibration(
    calibration: Calibration,
    selected_operators: &Vec<char>
) -> Option<u128> {
    let operators: Vec<_> = (0..calibration.test_values.len()-1)
        .map(|_| selected_operators.into_iter())
        .multi_cartesian_product()
        .collect();
    
    for operator in operators {
        let mut result = calibration.test_values[0];
        for (i, &op) in operator.iter().enumerate() {
            match op {
                '+' => result += calibration.test_values[i + 1],
                '*' => result *= calibration.test_values[i + 1],
                '|' => result = format!("{}{}", result, calibration.test_values[i + 1])
                    .parse::<u128>()
                    .unwrap(),
                _ => unreachable!("Only '+', '*', and '|' were expected!"),
            }
        }
        if result == calibration.test_result {
            return Some(calibration.test_result);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(String::from("3749"), part1(parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(String::from("11387"), part2(parse(TEST_INPUT)));
    }

}
