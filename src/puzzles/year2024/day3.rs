use regex::Regex;

fn parse(input: String) -> Vec<(String, i64, i64)> {
    let pat = r"(?<inst>(mul|do|don\'t))\((\)|(?<left>\d{1,3}),(?<right>\d{1,3})\))";
    let re: Regex = Regex::new(pat).unwrap();
    let caps = re.captures_iter(input.as_str());
    let mut instructions: Vec<(String, i64, i64)> = Vec::new();

    for cap in caps {
        instructions.push((
            cap["inst"].to_string(),
            cap.name("left").map_or(0, |m| m.as_str().parse::<i64>().unwrap()),
            cap.name("right").map_or(0, |m| m.as_str().parse::<i64>().unwrap()),
        ));
    }

    instructions
}

pub fn part1(input: String) -> String {
    let instructions = parse(input);
    let mut products = 0;

    for inst in instructions {
        if inst.0 == "mul" {
            products += inst.1 * inst.2;
        }
    }

    products.to_string()
}

pub fn part2(input: String) -> String {
    let instructions = parse(input);
    let mut products = 0;
    let mut enabled = 1;

    for inst in instructions {
        match inst.0.as_str() {
            "do" => enabled = 1,
            "don't" => enabled = 0,
            "mul" => {
                if enabled == 1 {
                    products += inst.1 * inst.2;
                }
            },
            &_ => {},
        }
        println!("{}({}, {})", inst.0, inst.1, inst.2);
    }

    products.to_string()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(String::from("161"), part1(parse(TEST_INPUT_1)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(String::from("48"), part2(parse(TEST_INPUT_2)));
    }

}
