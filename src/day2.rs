use text_io::scan;

#[derive(Default)]
struct PasswordRecord {
    password: String,
    min: usize,
    max: usize,
    required: String,
}


#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<PasswordRecord> {
    input.lines()
        .map(|s| {
            let mut r: PasswordRecord = Default::default();
            scan!(s.bytes() => "{}-{} {}: {}", r.min, r.max, r.required, r.password);
            r
        })
        .collect()
}


fn day2() {
}

#[aoc(day2, part1)]
fn part1(expenses: &[PasswordRecord]) {
    let mut clean: usize = 0;
    for p in &passwords {
        let c = p.password.matches(p.required).count();
        if p.min <= c && c <= p.max {
            clean += 1;
        }
    }
    println!("{}", clean);
}

#[aoc(day2, part2)]
fn part2(expenses: &[PasswordRecord]) {
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

