fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(input: String) -> String {
    let mut guard_map = parse(input);
    let (mut guard_symbol, mut pos_x, mut pos_y) = find_guard(&guard_map);
    let mut distinct_positions: i32 = 1;

    while move_guard(
        &mut guard_map,
        &mut guard_symbol,
        &mut pos_x,
        &mut pos_y) { }
    
    distinct_positions += guard_map
        .iter()
        .flat_map(|v| v.iter())
        .filter(|&c| *c == 'x')
        .count() as i32;
    
    distinct_positions.to_string()
}

pub fn part2(input: String) -> String {
    input
}

const GUARD_SYMBOLS: [char; 4] = ['^', 'v', '<', '>'];

fn find_guard(guard_map: &Vec<Vec<char>>) -> (char, usize, usize) {
    for x in 0..guard_map.len() {
        for y in 0..guard_map.len() {
            if GUARD_SYMBOLS.contains(&guard_map[x][y]) {
                return (guard_map[x][y], x, y)
            }
        }
    }
    ('-', 0, 0)
}

fn move_guard(
    guard_map: &mut Vec<Vec<char>>,
    guard_symbol: &mut char,
    pos_x: &mut usize,
    pos_y: &mut usize
) -> bool {
    let (symbol_ahead, next_x, next_y) = look_ahead(
        guard_map,
        guard_symbol,
        pos_x,
        pos_y);

    // println!("{} @ {:?}, {:?}", guard_symbol, pos_x, pos_y);

    match symbol_ahead {
        '.' => {
            guard_map[*pos_x][*pos_y] = 'x';
            guard_map[next_x][next_y] = *guard_symbol;
            *pos_x = next_x;
            *pos_y = next_y;
            return true;
        },
        '#' => {
            let rotated_guard_symbol = rotate_guard(*guard_symbol);
            guard_map[*pos_x][*pos_y] = rotated_guard_symbol;
            *guard_symbol = rotated_guard_symbol;
            return true;
        },
        'x' => {
            guard_map[*pos_x][*pos_y] = 'x';
            guard_map[next_x][next_y] = *guard_symbol;
            *pos_x = next_x;
            *pos_y = next_y;
            return true;
        },
        '$' => {
            return false;
        },
        _ => panic!("The next character should always be one of ['.', '#', 'x', '$'], \
            but it wasn't!"),
    }
}

// get rotated idiot
fn rotate_guard(guard_symbol: char) -> char {
    match guard_symbol {
        '^' => return '>',
        '>' => return 'v',
        'v' => return '<',
        '<' => return '^',
        _ => panic!("Guard symbol should always be one of {:?}, but it wasn't!",
                GUARD_SYMBOLS),
    }
}

fn look_ahead(
    guard_map: &mut Vec<Vec<char>>,
    guard_symbol: &char,
    pos_x: &usize,
    pos_y: &usize
) -> (char, usize, usize) {
    match guard_symbol {
        '^' => {
            if *pos_x as i16 - 1 < 0 {
                return ('$', 0, 0);
            } else {
                return (
                    guard_map[*pos_x-1][*pos_y],
                    *pos_x-1,
                    *pos_y
                );
            }
        },
        '>' => {
            if *pos_y as i16 + 1 >= (guard_map[0].len() as i16).abs() {
                return ('$', 0, 0);
            } else {
                return (
                    guard_map[*pos_x][*pos_y+1],
                    *pos_x,
                    *pos_y+1
                );
            }
        },
        'v' => {
            if *pos_x as i16 + 1 >= (guard_map[0].len() as i16).abs() {
                return ('$', 0, 0);
            } else {
                return (
                    guard_map[*pos_x+1][*pos_y],
                    *pos_x+1,
                    *pos_y
                );
            }
        },
        '<' => {
            if *pos_y as i16 - 1 < 0 {
                return ('$', 0, 0);
            } else {
                return (
                    guard_map[*pos_x][*pos_y-1],
                    *pos_x,
                    *pos_y-1
                );
            }
        },
        _ => panic!("Guard symbol should always be one of {:?}, but it wasn't!",
                GUARD_SYMBOLS)
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(String::from("41"), part1(parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(String::from("6"), part2(parse(TEST_INPUT)));
    }

}
