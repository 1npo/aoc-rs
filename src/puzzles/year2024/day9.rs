use std::iter::repeat;

fn parse(input: String) -> Vec<char> {
    input.chars().collect()
}

pub fn part1(input: String) -> String {
    let mut blocks = get_blocks(parse(input));
    let mut checksum: i128 = 0;

    println!("Compacting file blocks...");

    for i in (0..blocks.len()).rev() {
        if blocks[i] != -1 {
            let first_empty_space = blocks.iter().position(|c| *c == -1).unwrap();
            blocks.swap(first_empty_space, i);
        }
    }

    println!("Performing checksum...");

    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            checksum += (i * blocks[i] as usize) as i128;
        }
    }

    checksum.to_string()
}

// Skipping, spent too much time
pub fn part2(input: String) -> String {
    let mut blocks = get_blocks(parse(input));
    let mut checksum: i128 = 0;
    let mut contiguous: u8 = 0;
    let mut start_pos: usize = 0;

    // println!("{:?}", blocks);
    
    println!("Compacting file blocks...");

    loop {
        let last_file_block = find_last_file_block(&blocks);
        let first_empty_space = find_first_empty_space(&blocks, last_file_block.len());
        
        // println!("last_file_block @ {:?} (len = {:?})", last_file_block, last_file_block.len());
        // println!("first_empty_space @ {:?} (len = {:?})", first_empty_space, first_empty_space.len());
        // println!("===========================================");
        
        if first_empty_space.len() == 2540 {
            break;
        }

        for (first, last) in first_empty_space.into_iter().zip(last_file_block) {
            blocks.swap(first, last);
            // println!("swapped {:?} @ {:?} with {:?} @ {:?}", blocks[first], first, blocks[last], last);
        }
    }
    
    println!("Performing checksum...");
    
    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            checksum += (i * blocks[i] as usize) as i128;
        }
    }
    
    println!("{:?}", blocks);

    checksum.to_string()
}

fn get_blocks(disk_map: Vec<char>) -> Vec<i32> {
    let mut blocks: Vec<i32> = Vec::new();
    let mut file_id: i32 = 0;

    for (i, digit) in disk_map.into_iter().enumerate() {
        if let Some(num) = digit.to_digit(10) {
            if i % 2 == 0 {
                for n in 0..num {
                    blocks.push(file_id as i32);
                }
                file_id += 1;
            } else {
                for n in 0..num {
                    blocks.push(-1);
                }
            }
        }
    }
    
    blocks
}

fn find_first_empty_space(blocks: &Vec<i32>, width: usize) -> Vec<usize> {
    let mut items: Vec<usize> = Vec::new();
    let mut contiguous = false;

    for i in 0..blocks.len() {
        if width == 1 && blocks[i] == -1 {
            items.push(i);
            break;
        }
        if i > 0 {
            if blocks[i] == blocks[i-1] {
                if blocks[i] == -1 {
                    contiguous = true;
                    items.push(i-1);
                }
            } else {
                if contiguous == true {
                    items.push(i-1);
                    if items.len() == width {
                        break;
                    } else {
                        contiguous = false;
                        items.clear();
                    }
                }
            }
        }
    }
    
    items       
}

fn find_last_file_block(blocks: &Vec<i32>) -> Vec<usize> {
    let mut items: Vec<usize> = Vec::new();
    let mut contiguous = false;

    for i in (0..blocks.len()).rev() {
        if i < blocks.len()-1 {
            if blocks[i] != -1 && blocks[i+1] == -1 && blocks[i-1] == -1 {
                items.push(i);
                break;
            }
            if blocks[i] == blocks[i+1] {
                if blocks[i] != -1 {
                    contiguous = true;
                    items.push(i+1);
                }
            } else {
                if contiguous == true {
                    items.push(i+1);
                    break;
                }
            }
        }
    }

    items         
}

fn find_last_file(blocks: &Vec<i32>) -> (usize, i32) {
    for i in (0..blocks.len()).rev() {
        if blocks[i] != -1 {
            return (i, blocks[i]);
        }
    }
    
    (0, 0)
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
