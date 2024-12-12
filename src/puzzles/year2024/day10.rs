use std::collections::{HashMap, HashSet};

//#[derive(Debug)]
//struct Point<T> {
//    x: usize,
//    y: usize,
//    value: T,
//}
//
//#[derive(Debug)]
//struct Grid<T> {
//    grid: Vec<Point<T>>,
//}
//
//impl<T> Grid<T> {
//    fn from(raw_grid: Vec<Vec<i8>>) -> Grid<i8> {
//        let mut grid: Vec<Point<i8>> = Vec::new();
//        for x in 0..raw_grid.len() {
//            for y in 0..raw_grid[0].len() {
//                grid.push(Point { x, y, value: raw_grid[x][y] });
//            }
//        }
//        Grid { grid }
//    }
//}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct Cursor<T> {
    x: usize,
    y: usize,
    x_min: usize,
    y_min: usize,
    x_max: usize,
    y_max: usize,
    metadata: T,
}

impl<T> Cursor<T> {
    pub fn from(
        metadata: T,
        x: Option<usize>,
        y: Option<usize>,
        x_max: Option<usize>,
        y_max: Option<usize>,
        x_min: Option<usize>,
        y_min: Option<usize>
    ) -> Cursor<T> {
        let x = match x { Some(x) => x, None => 0 };
        let y = match y { Some(y) => y, None => 0 };
        let x_max = match x_max { Some (x_max) => x_max, None => usize::MAX };
        let y_max = match y_max { Some (y_max) => y_max, None => usize::MAX };
        let x_min = match x_min { Some (x_min) => x_min, None => 0 };
        let y_min = match y_min { Some (y_min) => y_min, None => 0 };
        
        Cursor {
            x,
            y,
            x_max,
            y_max,
            x_min,
            y_min,
            metadata,
        }
    }

    pub fn within_horizontal(&self, x: usize) -> bool {
        if self.y_min < x && self.x_max > x {
            return true
        }
        false
    }

    pub fn within_vertical(&self, y: usize) -> bool {
        if self.y_min < y && self.y_max > y {
            return true
        }
        false
    }

    pub fn within_grid(&self, x: usize, y: usize) -> bool {
        if self.within_horizontal(x) && self.within_vertical(y) {
            return true
        }
        false
    }

    pub fn relocate(&mut self, x: usize, y: usize) -> &mut Self {
        if self.within_grid(x, y) {
            self.x = x;
            self.y = y;
        }

        self
    }

    pub fn move_up(&mut self, by: Option<usize>) -> &mut Self {
        match by {
            Some(places) => {
                if self.within_vertical(self.x - places) {
                    self.x -= places; 
                }
            },
            None => {
                if self.within_vertical(self.x - 1) {
                    self.x -= 1;
                }
            },
        }
        self
    }

    pub fn move_down(&mut self, by: Option<usize>) -> &mut Self {
        match by {
            Some(places) => {
                if self.within_vertical(self.x + places) {
                    self.x += places; 
                }
            },
            None => {
                if self.within_vertical(self.x + 1) {
                    self.x += 1;
                }
            },
        }
        self
    }
    
    pub fn move_left(&mut self, by: Option<usize>) -> &mut Self {
        match by {
            Some(places) => {
                if self.within_horizontal(self.y - places) {
                    self.y -= places; 
                }
            },
            None => {
                if self.within_horizontal(self.y - 1) {
                    self.y -= 1;
                }
            },
        }
        self
    }

    pub fn move_right(&mut self, by: Option<usize>) -> &mut Self {
        match by {
            Some(places) => {
                if self.within_horizontal(self.y + places) {
                    self.y += places; 
                }
            },
            None => {
                if self.within_horizontal(self.y + 1) {
                    self.y += 1;
                }
            },
        }
        self
    }

}

// trailheads key = trailhead location, value = score
struct Hiker {
    trailheads: HashMap<Point, u8>,
}

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
    // let mut trailheads: HashSet<Point> = HashSet::new();
    // 
    // for x in 0..topography.len() {
    //     for y in 0..topography[0].len() {
    //         if topography[x][y] == 0 {
    //             // trailheads.push(Point { x, y });
    //         }
    //     }
    // }
    // 
    // for trailhead in trailheads {
    // }
    
    println!("{:?}", topography);
    // println!("{:?}", trailheads);

    String::from("")
}

pub fn part2(input: String) -> String {
    String::from("")
}

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
