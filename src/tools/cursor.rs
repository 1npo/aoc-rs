// solution
// ========
//
// 1. Identify the coordinates of all trailheads in the map
// 2. For each trailhead:
//  1. Save the trailhead coordinate to a temporary list
//  2. Search all 4 neighbors for the next available digit (if starting from trailhead, this is 1)
//  3. If any of the neighboring fields are the next number, save the current number
//  4. If 

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

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Cursor<T> {
    pub x: usize,
    pub y: usize,
    pub x_min: usize,
    pub y_min: usize,
    pub x_max: usize,
    pub y_max: usize,
    pub metadata: T,
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
