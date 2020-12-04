#[derive(Default)]
struct PasswordRecord {
    min: usize,
    max: usize,
    required: String,
    password: String,
}

impl std::str::FromStr for PasswordRecord {
    type Err = scan_fmt::parse::ScanError;

    fn from_str(s: &str) -> Result<PasswordRecord, Self::Err> {
        scan_fmt::scan_fmt!(s, "{}-{} {}: {}", usize, usize, String, String).map(
            |(min, max, required, password)| PasswordRecord{
                min: min,
                max: max,
                required: required,
                password: password,
            })

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
