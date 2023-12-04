use regex::Regex;
use std::collections::HashMap;

fn main() {
    let text = include_str!("../input-01.txt");
    resolve(text);
    resolve2(text);
}

fn resolve(text: &str) -> u32 {
    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut ids: u32 = 0;
    for (i, line) in text.lines().enumerate() {
        let sets = line.split(';');
        let mut valid_set = true;

        for set in sets {
            let game = get_game_count(set);
            let is_game = !bag
                .iter()
                .all(|(&key, &val1)| game.get(key).map_or(true, |&val2| val2 <= val1));
            if is_game {
                valid_set = false;
                break;
            }
        }

        if valid_set {
            ids += i as u32 + 1;
        }
    }
    println!("IDs: {}", ids);
    ids
}
fn resolve2(text: &str) -> u64 {
    let mut result = 0;
    for line in text.lines() {
        let max_values = get_max_value(line);
        let mut product = 1;
        for (_, &val) in max_values.iter() {
            product *= val as u64;
        }
        result += product;
    }
    println!("Result: {}", result);
    result
}

fn get_game_count(line: &str) -> HashMap<String, u32> {
    let re = Regex::new(r"(\d+)\s*(green|red|blue)").unwrap();
    let mut counts = HashMap::new();

    for cap in re.captures_iter(line) {
        let color = cap[2].to_string();
        let count: u32 = cap[1].parse().expect("Not a number");
        *counts.entry(color).or_insert(0) += count;
    }
    counts
}

fn get_max_value(line: &str) -> HashMap<String, u32> {
    let re = Regex::new(r"(\d+)\s*(green|red|blue)").unwrap();
    let mut max_values = HashMap::new();

    for cap in re.captures_iter(line) {
        let color = cap[2].to_string();
        let count: u32 = cap[1].parse().expect("Not a number");
        let entry = max_values.entry(color).or_insert(0);
        *entry = count.max(*entry);
    }
    max_values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let text = include_str!("../test-input-01.txt");
        assert_eq!(resolve(text), 8);
    }
}
