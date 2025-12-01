use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let reader = BufReader::new(File::open("in.txt").unwrap());

    let mut rot: i32 = 50;
    let mut p1 = 0;
    let mut p2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (dir, num_str) = line.split_at(1);
        let num: i32 = num_str.parse().unwrap();
        let step = if dir == "R" { 1 } else { -1 };

        p2 += num / 100;

        for _ in 0..(num % 100) {
            rot = (rot + step).rem_euclid(100);
            if rot == 0 {
                p2 += 1;
            }
        }

        if rot == 0 {
            p1 += 1;
        }
    }

    println!("Password: {}", p1);
    println!("Zero crosses: {}", p2);
}