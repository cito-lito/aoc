fn main() {
    let test_time_dist = vec![(7, 9), (15, 40), (30, 200)];
    let time_dist = vec![(52, 426), (94, 1374), (75, 1279), (94, 1216)];
    let result = resolve1(test_time_dist);
    println!("result: {}", result);
    let result = resolve1(time_dist);
    println!("result: {}", result);
    let result = resolve2((71530,940200));
    println!("result: {}", result);
    let result = resolve2((52947594,426137412791216));
    println!("result: {}", result);
}

fn resolve1(time_dist: Vec<(i32, i32)>) -> u64 {
    let mut total_win_ways = 1;
    for (time, distance) in time_dist {
        let mut wins = 0;
        for i in 1..time {
            let d = i * (time - i);
            if d > distance {
                wins += 1;
            }
        }
        total_win_ways *= wins;
    }
    total_win_ways
}


fn resolve2 (time_dist:(u64, u64)) -> u64 {
    let (time, distance) = time_dist;
    let mut wins = 0;
    for i in 1..time {
        let d = i * (time - i);
        if d > distance {
            wins += 1;
        }
        println!("i: {} d: {} w: {}", i, d, wins)
    }
    wins
}