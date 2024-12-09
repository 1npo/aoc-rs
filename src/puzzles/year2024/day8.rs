use std::collections::HashMap;
use itertools::Itertools;

struct Grid {
    grid: Vec<Vec<Point>>,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
    value: Option<char>,
    has_antenna: Option<bool>,
    has_antinode: Option<bool>,
}

impl Grid {
    pub fn from(raw_grid: &Vec<Vec<char>>) -> Grid {
        let mut grid: Vec<Vec<Point>> = Vec::new();
        for x in 0..raw_grid.len() {
            grid.push(Vec::new());
            for y in 0..raw_grid[0].len() {
                let is_antenna = if ['.', '#'].contains(&raw_grid[x][y]) {
                    false
                } else {
                    true
                };

                grid[x].push(Point {
                    x,
                    y,
                    value: Some(raw_grid[x][y]),
                    has_antenna: Some(is_antenna),
                    has_antinode: None,
                });

            }
        }

        Grid { grid }
    }

    pub fn find_antenna_locations(&self) -> HashMap<char, Vec<Point>> {
        let mut locations: HashMap<char, Vec<Point>> = HashMap::new();
        let (len_x, len_y) = self.extents();
        
        for x in 0..len_x {
            for y in 0..len_y {
                if self.grid[x][y].has_antenna.unwrap() {
                    locations
                        .entry(self.grid[x][y].value.unwrap())
                        .or_insert(Vec::new())
                        .push(self.grid[x][y].clone());
                }
            }
        }

        locations
    }

    pub fn count_antinodes(&mut self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|point| {
                match point.has_antinode {
                    Some(value) => value,
                    None => false
                }
            } == true)
            .count()
    }

    fn point(&mut self, x: usize, y: usize) -> &mut Point {
        &mut self.grid[x][y]
    }

    fn extents(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    fn within_grid(&self, x: i16, y: i16) -> bool {
        (0..self.grid.len()).contains(&(x as usize)) &&
        (0..self.grid[0].len()).contains(&(y as usize))
    }
}

fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(input: String) -> String {
    let mut map = Grid::from(&parse(input));
    let antennas = map.find_antenna_locations();
    for antenna in antennas.keys() {
        for pair in antennas.get(antenna).unwrap().iter().combinations(2) {
            let antenna1 = pair[0];
            let antenna2 = pair[1];

            let distance = (
                antenna1.x as i16 - antenna2.x as i16,
                antenna1.y as i16 - antenna2.y as i16
            );

            let antinode1 = (
                antenna1.x as i16 + distance.0,
                antenna1.y as i16 + distance.1,
            );
            let antinode2 = (
                antenna2.x as i16 - distance.0,
                antenna2.y as i16 - distance.1,
            );
            
            if map.within_grid(antinode1.0, antinode1.1) {
                map.point(antinode1.0 as usize, antinode1.1 as usize)
                    .has_antinode = Some(true);
            }

            if map.within_grid(antinode2.0, antinode2.1) {
                map.point(antinode2.0 as usize, antinode2.1 as usize)
                    .has_antinode = Some(true);
            }
        }
    }

    map.count_antinodes().to_string()
}

pub fn part2(input: String) -> String {
    let mut map = Grid::from(&parse(input));
    let antennas = map.find_antenna_locations();

    for antenna in antennas.keys() {   
        if antennas.get(antenna).unwrap().len() == 1 {
            println!("antenna len 1: {:?}", antennas.get(antenna).unwrap());
        }
        for pair in antennas.get(antenna).unwrap().iter().combinations(2) {
            let antenna1 = (pair[0].x as i16, pair[0].y as i16);
            let antenna2 = (pair[1].x as i16, pair[1].y as i16);
            let distance = (
                antenna1.0 - antenna2.0,
                antenna1.1 - antenna2.1
            );

            let mut antinode1 = (antenna1.0, antenna1.1);
            while map.within_grid(antinode1.0, antinode1.1) {
                map.point(antinode1.0 as usize, antinode1.1 as usize)
                    .has_antinode = Some(true);
                antinode1 = (
                    antinode1.0 + distance.0,
                    antinode1.1 + distance.1
                );

            }

            let mut antinode2 = (antenna2.0, antenna2.1);
            while map.within_grid(antinode2.0, antinode2.1) {
                map.point(antinode2.0 as usize, antinode2.1 as usize)
                    .has_antinode = Some(true);
                antinode2 = (
                    antinode2.0 - distance.0,
                    antinode2.1 - distance.1
                );
            }
        }
    }

    map.count_antinodes().to_string()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(String::from("14"), part1(parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(String::from("34"), part2(parse(TEST_INPUT)));
    }

}
