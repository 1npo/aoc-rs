fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(input: String) -> String {
    let matrix = parse(input);
    let len_x = matrix.len();
    let len_y = matrix.clone().first().unwrap().len();
    let mut xmas_occurrences: i32 = 0;

    let horizontal = search_horizontally(&len_x, &len_y, &matrix);
    let vertical = search_vertically(&len_x, &len_y, &matrix);
    let diagonal = search_diagonally(&len_x, &len_y, &matrix);

    xmas_occurrences += horizontal + vertical + diagonal;

    xmas_occurrences.to_string()
}

pub fn part2(input: String) -> String {
    let matrix = parse(input);
    let len_x = matrix.len();
    let len_y = matrix.clone().first().unwrap().len();
    let mut xmas_occurrences: i32 = 0;

    xmas_occurrences += search_cross(&len_x, &len_y, &matrix);

    xmas_occurrences.to_string()
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn search_vertically(len_x: &usize, len_y: &usize, matrix: &Vec<Vec<char>>) -> i32 {
    let mut occurrences = 0;

    for x in 0..*len_x-3 {
        for y in 0..*len_y {
            let mut vertical: Vec<char> = Vec::new();
            for z in 0..=3 {
                vertical.push(matrix[x + z][y]);
            }
            let reversed = vertical
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<char>>();
            if vertical == &XMAS {
                occurrences += 1;
            }
            if reversed == &XMAS {
                occurrences += 1;
            }
        }
    }

    occurrences
}

fn search_horizontally(len_x: &usize, len_y: &usize, matrix: &Vec<Vec<char>>) -> i32 {
    let mut occurrences = 0;

    for x in 0..*len_x {
        for y in 0..*len_y-3 {
            let reversed = &matrix[x][y..y+4]
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<char>>();
            if &matrix[x][y..y+4] == &XMAS {
                occurrences += 1;
            }
            if reversed == &XMAS {
                occurrences += 1;
            }
        }
    }

    occurrences
}

fn search_diagonally(len_x: &usize, len_y: &usize, matrix: &Vec<Vec<char>>) -> i32 {
    let mut occurrences = 0;

    // Traverse from left to right
    for x in 0..*len_x-3 {
        for y in 0..*len_x-3 {
            let mut diagonal: Vec<char> = Vec::new();
            for z in 0..=3 {
                diagonal.push(matrix[x + z][y + z]);
            }
            let reversed = diagonal
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<char>>();

            if diagonal == &XMAS {
                occurrences += 1;
            }

            if reversed == &XMAS {
                occurrences += 1;
            }
        }
    }

    // Traverse from right to left
    for x in 0..*len_x-3 {
        for y in 3..*len_y {
            let mut diagonal: Vec<char> = Vec::new();
            for z in 0..=3 {
                diagonal.push(matrix[x + z][y - z]);
            }
            let reversed = diagonal
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<char>>();

            if diagonal == &XMAS {
                occurrences += 1;
            }

            if reversed == &XMAS {
                occurrences += 1;
            }
        }
    }

    occurrences
}

fn search_cross(len_x: &usize, len_y: &usize, matrix: &Vec<Vec<char>>) -> i32 {
    let mut occurrences = 0;

    // Scan the matrix in 6x6 sections and check if each section contains a valid X-MAS
    for x in 0..*len_x-2 {
        for y in 0..*len_y-2 {
            let mut cube: Vec<Vec<char>> = Vec::new();
            for z in 0..=2 {
                cube.push(Vec::from(&matrix[x+z][y..y+3]));
            }
            
            if cube[1][1] == 'A' {
                if cube[0][0] == 'M' && cube[2][2] == 'S' ||
                    cube[0][0] == 'S' && cube[2][2] == 'M' {
                    if cube[0][2] == 'M' && cube[2][0] == 'S' ||
                        cube[0][2] == 'S' && cube[2][0] == 'M' {
                        occurrences += 1;
                    }
                }
            }
        }
    }
    occurrences    
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
