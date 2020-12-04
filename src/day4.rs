use std::collections::HashMap;

#[aoc_generator(day4)]
fn gen(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|record| {
            record
                .split_whitespace()
                .map(|entry| entry.split(":").collect::<Vec<&str>>())
                .map(|vec| (vec[0].to_string(), vec[1].to_string()))
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(passports: &[HashMap<String, String>]) -> usize {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    passports
        .iter()
        .filter(|p| {
            for f in &required_fields {
                if !p.contains_key(*f) {
                    return false;
                }
            }
            return true;
        })
        .count()
}

fn is_valid(p: &HashMap<String, String>) -> bool {
    p.get("byr").map_or(false, |s| {
        s.parse::<i64>().map_or(false, |v| 1920 <= v && v <= 2002)
    }) && // --
    p.get("iyr").map_or(false, |s| {
        s.parse::<i64>().map_or(false, |v| 2010 <= v && v <= 2020)
    }) && // --
    p.get("eyr").map_or(false, |s| {
        s.parse::<i64>().map_or(false, |v| 2020 <= v && v <= 2030)
    }) && // --
    p.get("hgt").map_or(false, |s| {
        scan_fmt::scan_fmt!(s, "{d}{}", i64, String).map_or(false, |(size, unit)| {
            if unit == "cm" && 150 <= size && size <= 193 {
                return true;
            }
            if unit == "in" && 59 <= size && size <= 76 {
                return true;
            }
            return false;
        })
    }) && // --
    p.get("hcl").map_or(false, |s| {
        scan_fmt::scan_fmt!(s, "#{x}", String).map_or(false, |s| s.len() == 6)
    }) && // --
    p.get("ecl").map_or(false, |s| {
        s == "amb"
            || s == "blu"
            || s == "brn"
            || s == "gry"
            || s == "grn"
            || s == "hzl"
            || s == "oth"
    }) && // --
    p.get("pid").map_or(false, |s| {
        scan_fmt::scan_fmt!(s, "{d}", String).map_or(false, |s| s.len() == 9)
    })
}

#[aoc(day4, part2)]
fn part2(passports: &[HashMap<String, String>]) -> usize {
    passports.iter().filter(|p| is_valid(p)).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invalid_passports() {
        let input = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154
";
        let passports = gen(input);
        for p in &passports {
            assert_eq!(is_valid(p), false);
        }
    }

    #[test]
    fn valid_passports() {
        let input = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
        let passports = gen(input);
        for p in &passports {
            assert_eq!(is_valid(p), true);
        }
    }
}
