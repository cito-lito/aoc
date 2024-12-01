fn main() {
    let input = include_str!("../input1.txt");
    let mut vec1: Vec<i64> = Vec::new();
    let mut vec2: Vec<i64> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i64>().ok())
            .collect();

        vec1.push(numbers[0]);
        vec2.push(numbers[1]);
    }

    let part2 = similarity(vec1.clone(), vec2.clone());
    let part2: i64 = part2.iter().sum();
    println!("part2: {}", part2);

    vec1.sort_unstable();
    vec2.sort_unstable();

    let size = vec1.len();
    let mut total: Vec<u64> = Vec::new();
    if vec1.len() == vec2.len() {
        for i in 0..size {
            let diff = (vec1[i] - vec2[i]).abs() as u64;
            total.push(diff);
        }
    }

    let total: u64 = total.iter().sum();
    println!("part1: {}", total);
}

fn similarity(vec1: Vec<i64>, vec2: Vec<i64>) -> Vec<i64> {
    let mut counts: Vec<i64> = Vec::new();
    for i in vec1.iter() {
        let count = vec2.iter().filter(|&n| n == i).count() as i64;
        counts.push(i * count);
    }
    counts
}
