use std::collections::HashMap;
use scan_fmt::scan_fmt;

#[derive(Debug)]
enum Rule {
    Literal(char),
    Sequence(Vec<usize>),
    Alternative(Vec<usize>, Vec<usize>),
}

#[derive(Debug)]
struct Input {
    rules: HashMap<usize, Rule>,
    lines: Vec<String>,
}

fn parse_seq(input: &str) -> Vec<usize> {
    input.split_ascii_whitespace()
        .map(|d| d.parse::<usize>().unwrap())
        .collect()
}

fn parse_rules(input: &str) -> HashMap<usize, Rule> {
    let mut rules = HashMap::new();
    for line in input.lines() {
        if line.contains('"') {
            let (rule, lit) = scan_fmt!(line, "{d}: \"{[a-z]}\"", usize, char).unwrap();
            rules.insert(rule, Rule::Literal(lit));
        } else {
            let mut parts = line.split(|c| c == ':' || c == '|');
            let rule = parts.next().unwrap().parse::<usize>().unwrap();
            let s1 = parse_seq(parts.next().unwrap_or(""));
            let s2 = parse_seq(parts.next().unwrap_or(""));
            if s2.is_empty() {
                rules.insert(rule, Rule::Sequence(s1));
            } else {
                rules.insert(rule, Rule::Alternative(s1, s2));
            }
        }
    }
    rules
}

#[aoc_generator(day19, part1)]
fn gen1(input: &str) -> Input {
    let mut stanzas = input.split("\n\n");
    Input {
        rules: parse_rules(stanzas.next().unwrap()),
        lines: stanzas.next().unwrap().lines().map(|l| l.to_string()).collect(),
    }
}

#[aoc_generator(day19, part2)]
fn gen2(input: &str) -> Input {
    let mut i = gen1(input);
    i.rules.insert(8, Rule::Alternative(vec![42], vec![42, 8]));
    i.rules.insert(11, Rule::Alternative(vec![42, 31], vec![42, 11, 31]));
    i
}

fn consume_seq(line: &str, seq: &[usize], rules: &HashMap<usize, Rule>) -> Vec<usize> {
    match seq.len() {
        0 => panic!(),
        1 => consume(line, seq[0], rules),
        _ => {
            let mut r = vec![];
            let head_options = consume(line, seq[0], rules);
            for head in head_options {
                let tail_options = consume_seq(&line[head..], &seq[1..], rules);
                for tail in tail_options {
                    r.push(head + tail);
                }
            }
            r
        }
    }
}

fn consume(line: &str, id: usize, rules: &HashMap<usize, Rule>) -> Vec<usize> {
    let mut r = vec![];
    match &rules[&id] {
        Rule::Literal(c) => {
            if line.starts_with(*c) {
                r.push(1);
            }
        },
        Rule::Sequence(seq) => {
            r.append(&mut consume_seq(line, seq, rules));
        },
        Rule::Alternative(lhs, rhs) => {
            r.append(&mut consume_seq(line, lhs, rules));
            r.append(&mut consume_seq(line, rhs, rules));
        }
    }
    r
}


#[aoc(day19, part1)]
fn part1(i: &Input) -> usize {
    let mut r = 0;
    for line in &i.lines {
        let matches = consume(line.as_str(), 0, &i.rules);
        if matches.iter().any(|m| *m == line.len()) {
            r += 1;
        }
    }
    r
}

#[aoc(day19, part2)]
fn part2(i: &Input) -> usize {
    part1(i)
}

#[test]
fn test_small() {
    let input = "
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
".trim();
    assert_eq!(part1(&gen1(input)), 2);
}


#[test]
fn test_big() {
    let input = "
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
".trim();
    // assert_eq!(part1(&gen1(input)), 3);
    assert_eq!(part2(&gen2(input)), 1);
}
