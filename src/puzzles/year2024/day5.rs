use std::collections::{HashMap, HashSet};

fn parse(input: String) -> (Vec<Vec<i8>>, Vec<Vec<i8>>) {
    let mut rules: Vec<Vec<i8>> = Vec::new();
    let mut order: Vec<Vec<i8>> = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            rules.push(line
                .split('|')
                .filter_map(|s| s.trim().parse::<i8>().ok())
                .collect());
        } else if line.contains(',') {
            order.push(line
                .split(',')
                .filter_map(|s| s.trim().parse::<i8>().ok())
                .collect());
        }
    }
    (rules, order)
}

pub fn part1(input: String) -> String {
    let (rules, order) = parse(input);
    let mut middle_pages: i16 = 0;

    for pages in order {
        let applicable_rules: Vec<Vec<i8>> = rules
            .iter()
            .filter(|rule| pages.contains(&rule[0]) || pages.contains(&rule[1]))
            .cloned()
            .collect();
        
        if is_correct_order(&applicable_rules, &pages) {
            middle_pages += pages[&pages.len() / 2] as i16;
        }
    }

    middle_pages.to_string()
}

pub fn part2(input: String) -> String {
    let (rules, order) = parse(input);
    let mut middle_pages: i16 = 0;

    for pages in order {
        let applicable_rules: Vec<Vec<i8>> = rules
            .iter()
            .filter(|rule| pages.contains(&rule[0]) && pages.contains(&rule[1]))
            .cloned()
            .collect();
        
        if !is_correct_order(&applicable_rules, &pages) {
            let new_pages = rearrange_pages(&applicable_rules, pages);
            middle_pages += new_pages[&new_pages.len() / 2] as i16;
        }
    }

    middle_pages.to_string()
}

// Assume the order of `pages` doesn't follow the given `rules`. For each of the given
// rules, find for the position of the "before" and "after" number in pages. Keep swapping
// the position of the numbers until the "before" number is positioned before the "after"
// number.
fn rearrange_pages(rules: &Vec<Vec<i8>>, pages: Vec<i8>) -> Vec<i8> {
    let mut pages = pages;
    let mut rearranged = true;

    while rearranged {
        rearranged = false;

        for rule in rules {
            let before: i8 = rule[0];
            let after: i8 = rule[1];

            if let (Some(before_pos), Some(after_pos)) = (
                pages.iter().position(|&p| p == before),
                pages.iter().position(|&p| p == after)) {

                if before_pos > after_pos {
                    pages.swap(before_pos, after_pos);
                    rearranged = true;
                }
            }
        }
    }
    
    pages
}

fn is_correct_order(rules: &Vec<Vec<i8>>, pages: &Vec<i8>) -> bool {
    // Create a mapping of each page number to that number's position in the list of pages
    let page_positions: HashMap<i8, usize> = pages
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i)).collect();
        
        // For each rule in the list of applicable rules, check if the position of the
        // "before" number is greater than (comes after) the "after" number. If it does,
        // the given pages don't adhere to the rules.
        for rule in rules {
            let before: i8 = rule[0];
            let after: i8 = rule[1];
            
            if page_positions.contains_key(&before) 
                && page_positions.contains_key(&after) {
                if page_positions[&before] > page_positions[&after] {
                    return false;
                }
            }
        }
    true
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
