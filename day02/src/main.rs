use std::fs;

fn main() {
    let input = fs::read_to_string("in.txt").unwrap();
    let mut p1: u64 = 0;
    let mut p2: u64 = 0;

    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        for num in start..=end {
            let s = num.to_string();

            if s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..] {
                p1 += num;
            }

            let doubled = s.repeat(2);
            if doubled[1..doubled.len() - 1].contains(&s) {
                p2 += num;
            }
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}