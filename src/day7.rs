use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day7, part1)]
fn gen1(input: &str) -> HashMap<String, Vec<String>> {
    let re = regex::Regex::new(r"\w+ \w+ bag").unwrap();
    let mut r : HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let mut iter = re.find_iter(line);
        let key = iter.next().unwrap().as_str();
        for bag in iter {
            r.entry(bag.as_str().to_string())
                .or_default()
                .push(key.to_string())
        }
    }
    r
}

#[aoc_generator(day7, part2)]
fn gen2(input: &str) -> HashMap<String, Vec<(i64, String)>> {
    let re = regex::Regex::new(r"(\d*) ?(\w+ \w+ bag)").unwrap();
    let mut r = HashMap::new();
    for line in input.lines() {
        let mut iter = re.captures_iter(line);
        let key = iter.next().unwrap().get(2).unwrap().as_str();
        let v = r.entry(key.to_string()).or_insert(vec![]);
        for bag in iter {
            let n = bag.get(1).unwrap().as_str().parse::<i64>().unwrap_or(0);
            let b = bag.get(2).unwrap().as_str();
            v.push((n, b.to_string()));
        }
    }
    r
}

#[aoc(day7, part1)]
fn part1(rules: &HashMap<String, Vec<String>>) -> usize {
    let mut want = vec!["shiny gold bag"];
    let mut seen = HashSet::new();
    while let Some(bag) = want.pop() {
        if seen.insert(bag) {
            if let Some(b) = rules.get(bag) {
                for outer in b {
                    want.push(&outer)
                }
            }
        }
    }
    seen.len() - 1
}

#[aoc(day7, part2)]
fn part2(rules: &HashMap<String, Vec<(i64, String)>>) -> i64 {
    count_bags("shiny gold bag", rules) - 1
}

fn count_bags(bag: &str, rules: &HashMap<String, Vec<(i64, String)>>) -> i64 {
    let mut total = 1;
    if let Some(v) = rules.get(bag) {
        for (n, b) in v {
            total += n * count_bags(b, rules);
        }
    }
    total
}

#[test]
fn part2_test() {
    let input = "
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
".trim();
    assert_eq!(part2(&gen2(input)), 126);
}
