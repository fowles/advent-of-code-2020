#![allow(dead_code)]

use text_io::scan;

fn main() {
    day2();
}

fn day1() {
    let expenses: Vec<i64> = std::fs::read_to_string("data/day1.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    for i in &expenses {
        for j in &expenses {
            for k in &expenses {
                if i + j + k == 2020 {
                    println!("{}", i*j*k);
                }
            }
        }
    }
}

#[derive(Default)]
struct PasswordRecord {
    password: String,
    min: usize,
    max: usize,
    required: String,
}

fn day2() {
    let passwords: Vec<PasswordRecord> = std::fs::read_to_string("data/day2.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let mut r: PasswordRecord = Default::default();
            scan!(s.bytes() => "{}-{} {}: {}", r.min, r.max, r.required, r.password);
            r
        })
        .collect();
    let mut clean: usize = 0;
    for p in &passwords {
        let p1 = &p.password[p.min - 1 .. p.min];
        let p2 = &p.password[p.max - 1 .. p.max];
        let r = &p.required;
        if p1 == r && p2 == r {
            continue;
        }
        if p1 == r || p2 == r {
            clean += 1;
        }
    }
    println!("{}", clean);
}
