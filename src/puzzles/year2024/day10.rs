use std::collections::{HashMap, HashSet};

use crate::tools::cursor::{Point, Cursor};
use crate::tools::dfs::find_all_paths;

// trailheads key = trailhead location, value = score
//struct Hiker {
// }

fn parse(input: String) -> Vec<Vec<i8>> {
    let mut topography: Vec<Vec<i8>> = Vec::new();

    for line in input.lines() {
        topography.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        );
    }

    topography
}

pub fn part1(input: String) -> String {
    let topography = parse(input);
    let mut trailheads: HashSet<(x, y)> = HashSet::new();
    let mut 
    for x in 0..topography.len() {
        for y in 0..topography[0].len() {
            if topography[x][y] == 0 {
                trailheads.insert((x, y));
            }
        }
    }
    
    for trailhead in trailheads {
        
    }
    
    println!("{:?}", topography);
    // println!("{:?}", trailheads);

    String::from("")
}

pub fn part2(input: String) -> String {
    String::from("")
}

// fn find_trailheads(input: ) -> Vec<Vec<i8>> {
//     0
// }

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "";
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(String::from(""), part1(parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(String::from(""), part2(parse(TEST_INPUT)));
    }

}
