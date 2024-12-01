fn main() {
    let input = include_str!("../../input-01.txt");
    println!("Result: {}", resolve(input));
    println!("Result: {}", resolve2(input));
}

fn resolve(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let mut numbers = vec![];
        for char in line.chars() {
            if char.is_numeric() {
                numbers.push(char);
            }
        }
        if numbers.len() < 1 {
            continue;
        }
        let result = vec![numbers[0], numbers[numbers.len() - 1]];
        let result = format!("{}{}", result[0], result[1]);
        sum += result.parse::<u32>().unwrap();
    }
    return sum;
}
fn resolve2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut numbers = vec![];
            for char in line.chars() {
                if char.is_numeric() {
                    numbers.push(char);
                }
            }
            if numbers.len() < 1 {
                return None;
            }
            let result = vec![numbers[0], numbers[numbers.len() - 1]];
            let result = format!("{}{}", result[0], result[1]);
            result.parse::<u32>().ok()
        })
        .sum::<u32>()
}

//// 55029

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let input1 = include_str!("../../test-input-01.txt");
        let input2 = include_str!("../../input-01.txt");
        let result1 = resolve(input1);
        let result2 = resolve(input2);
        assert_eq!(result1, 142);
        assert_eq!(result2, 55029);
    }
    #[test]
    fn test_resolve2() {
        let input1 = include_str!("../../test-input-01.txt");
        let input2 = include_str!("../../input-01.txt");
        let result1 = resolve2(input1);
        let result2 = resolve2(input2);
        assert_eq!(result1, 142);
        assert_eq!(result2, 55029);
    }
}
