fn main() {
    let input = include_str!("../../input-02.txt");
    println!("Result: {}", resolve(input));
}

fn resolve(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        result += resolve_line(line);
    }
    result
}

fn resolve_line(line: &str) -> u32 {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut positions = vec![];

    for (i, char) in line.chars().enumerate() {
        if let Some(digit) = char.to_digit(10) {
            positions.push((i, digit));
            continue;
        }

        for (num, word) in numbers.iter().enumerate() {
            if line[i..].starts_with(word) {
                positions.push((i, num as u32));
                break;
            }
        }
    }

    positions.sort_by_key(|&(pos, _)| pos);
    println!("{:?}", positions);


    match positions.as_slice() {
        // if just one num
        &[(_pos, num)] => format!("{0}{0}", num).parse().unwrap_or(0),
        //
        &[(first_pos, first), .., (last_pos, last)] if first_pos != last_pos => {
            format!("{}{}", first, last).parse().unwrap_or(0)
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let input1 = include_str!("../../test-input-02.txt");
        let result1 = resolve(input1);
        assert_eq!(result1, 281);
    }
}
