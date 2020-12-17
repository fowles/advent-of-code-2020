use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::Range;
use regex::Regex;

#[derive(Default, Clone, Debug)]
struct Tickets {
    rules: HashMap<String, (Range<usize>, Range<usize>)>,
    mine: Vec<usize>,
    nearby: Vec<Vec<usize>>,
}

#[aoc_generator(day16)]
fn gen(input: &str) -> Tickets {
    let re = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    let mut tickets: Tickets = Default::default();

    let mut iter = input.split("\n\n");
    for rule in iter.next().unwrap().lines() {
        let caps = re.captures(rule).unwrap();

        tickets.rules.insert(
            caps.get(1).unwrap().as_str().to_string(),
            (Range {
                start: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                end: 1+caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            },
            Range {
                start: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                end: 1+caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
            }));
    }

    let mut mine_iter = iter.next().unwrap().lines();
    tickets.mine = mine_iter.nth(1).unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut nearby_iter = iter.next().unwrap().lines();
    nearby_iter.next();  // pour one out
    for n in nearby_iter {
        tickets.nearby.push(n.split(",")
                            .map(|s| s.parse().unwrap())
                            .collect());
    }

    tickets
}

fn is_valid(tickets: &Tickets, t: &Vec<usize>) -> bool {
    'valid: for v in t {
        for (r1, r2) in tickets.rules.values() {
            if r1.contains(&v) {
                continue 'valid;
            }
            if r2.contains(&v) {
                continue 'valid;
            }
        }

        return false;
    }
    return true;
}

#[aoc(day16, part1)]
fn part1(tickets: &Tickets) -> usize {
    let mut r = 0;
    for near in &tickets.nearby {
        'valid: for v in near {
            for (r1, r2) in tickets.rules.values() {
                if r1.contains(&v) {
                    continue 'valid;
                }
                if r2.contains(&v) {
                    continue 'valid;
                }
            }

            r += v;
        }
    }
    r
}

#[aoc(day16, part2)]
fn part2(t: &Tickets) -> usize {
    let mut tickets = t.clone();
    tickets.nearby = tickets.nearby.into_iter()
        .filter(|n| is_valid(t, n))
        .collect();

    let num_fields = tickets.rules.len();

    let mut options = HashMap::<String, Vec<usize>>::new();
    for i in 0..num_fields {
        for (key, (r1, r2)) in &tickets.rules {
            let valid = tickets.nearby.iter().all(
                |n| r1.contains(&n[i]) || r2.contains(&n[i]));
            if valid {
                options.entry(key.clone()).or_default().push(i);
            }
        }
    }

    let mut priority = options.into_iter().collect::<Vec<_>>();
    priority.sort_unstable_by_key(|(_, v)| v.len());

    let mut seen = HashSet::<usize>::new();
    let mut mapping = HashMap::<String, usize>::new();
    for (k, options) in priority {
        for o in options {
            if seen.insert(o) {
                mapping.insert(k, o);
                break;
            }
        }
    }

    let mut prod = 1;
    for (key, idx) in mapping {
        if key.starts_with("departure") {
            prod *= tickets.mine[idx];
        }
    }
    prod
}

#[test]
fn test_small() {
    // assert_eq!(part1(&gen("0,3,6")), 436);
    // assert_eq!(part2(&gen("0,3,6")), 175594);
}

