#[derive(Default)]
struct PasswordRecord {
    password: String,
    min: usize,
    max: usize,
    required: String,
}

impl std::str::FromStr for PasswordRecord {
    type Err = Box<text_io::Error>;

    fn from_str(s: &str) -> Result<PasswordRecord, Self::Err> {
        let mut r: PasswordRecord = Default::default();
        text_io::try_scan!(
                s.bytes() => "{}-{} {}: {}",
                r.min, r.max, r.required, r.password);
        Ok(r)
    }
}

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<PasswordRecord> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(passwords: &[PasswordRecord]) -> usize {
    let mut clean: usize = 0;
    for p in passwords {
        let c = p.password.matches(&p.required).count();
        if p.min <= c && c <= p.max {
            clean += 1;
        }
    }
    return clean;
}

#[aoc(day2, part2)]
fn part2(passwords: &[PasswordRecord]) -> usize {
    let mut clean: usize = 0;
    for p in passwords {
        let p1 = &p.password[p.min - 1..p.min];
        let p2 = &p.password[p.max - 1..p.max];
        let r = &p.required;
        if p1 == r && p2 == r {
            continue;
        }
        if p1 == r || p2 == r {
            clean += 1;
        }
    }
    return clean;
}
