type Parsed = ();

fn parse(input: &str) -> Parsed {}

pub fn part1(data: &Parsed) -> u8 {
    0
}

pub fn part2(data: &Parsed) -> u8 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "";
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(0, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(&parse(TEST_INPUT)));
    }

}