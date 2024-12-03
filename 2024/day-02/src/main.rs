fn main() {
    let input = include_str!("../input1.txt");
    let mut count1 = 0;
    let mut count2 = 0;

    for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i64>().ok())
            .collect();

        if is_safe(&numbers) {
            count1 += 1;
        }

        if is_safe_with_single_bad_level(&numbers) {
            count2 += 1;
        }
    }

    println!("part 1: --> {}", count1);
    println!("part 2: --> {}", count2);
}

fn is_safe(numbers: &[i64]) -> bool {
    if numbers.len() < 2 {
        return true; // Single or empty array is always safe
    }

    let mut is_ascending = true;
    let mut is_descending = true;

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if diff == 0 || diff.abs() > 3 {
            return false; // Unsafe if difference is invalid
        }
        if diff < 0 {
            is_ascending = false;
        } else if diff > 0 {
            is_descending = false;
        }
    }

    is_ascending || is_descending
}

fn is_safe_with_single_bad_level(numbers: &[i64]) -> bool {
    for i in 0..numbers.len() {
        let mut temp_numbers = numbers.to_vec();
        temp_numbers.remove(i);
        if is_safe(&temp_numbers) {
            return true;
        }
    }
    false
}
