use std::fs;

fn main() {
    let input = fs::read_to_string("in.txt").unwrap();
    let lines: Vec<Vec<u32>> = input.lines().map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect()).collect();

    println!("Part 1: {}", solve(&lines, 2));
    println!("Part 2: {}", solve(&lines, 12));
}

fn solve(lines: &[Vec<u32>], n: usize) -> u64 {
    let mut total = 0;

    for digits in lines {
        if digits.len() < n { continue; }

        let mut val = 0;
        let mut start = 0;

        for i in 0..n {
            let end = digits.len() - (n - 1 - i);
            let slice = &digits[start..end];
            
            let &max = slice.iter().max().unwrap();
            let pos = slice.iter().position(|&x| x == max).unwrap();

            val = val * 10 + max as u64;
            start += pos + 1;
        }
        total += val;
    }

    total
}