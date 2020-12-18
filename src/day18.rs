use std::iter::Iterator;
use std::iter::Peekable;

#[aoc_generator(day18)]
fn gen(input: &str) -> Vec<String> {
    input.lines().map(|l| {
        l.replace('(', "( ").replace(')', " )")
    }).collect()
}

fn term(e: &mut dyn Iterator<Item = &str>) -> usize {
    match e.next() {
        Some("(") => expr(e),
        Some(d) => d.parse().unwrap(),
        _ => panic!(),
    }
}

fn expr(e: &mut dyn Iterator<Item = &str>) -> usize {
    let mut lhs = term(e);
    while let Some(op) = e.next() {
        match op {
            "+" => lhs += term(e),
            "*" => lhs *= term(e),
            ")" => break,
            _ => panic!(),
        }
    }
    lhs
}

#[aoc(day18, part1)]
fn part1(lines: &[String]) -> usize {
    lines.iter()
        .map(|l| expr(&mut l.as_str().split_ascii_whitespace()))
        .sum()
}

fn term2<'a, I>(e: &mut Peekable<I>) -> usize
where I: Iterator<Item = &'a str> {
    match e.next() {
        Some("(") => {
            let v = expr2(e);
            let p = e.next().unwrap();
            assert!(p == ")");
            return v;
        },
        Some(d) => {
            return d.parse().unwrap();
        },
        _ => panic!(),
    }
}

fn expr2<'a, I>(e: &mut Peekable<I>) -> usize 
where I: Iterator<Item = &'a str> {
    let mut lhs = term2(e);
    while let Some(op) = e.peek() {
        match *op {
            "+" => {
                e.next();
                lhs += term2(e);
            },
            "*" => {
                e.next();
                lhs *= expr2(e);
            },
            ")" => {
                return lhs;
            },
            _ => panic!(),
        }
    }
    lhs
}

fn eval2(line: &str) -> usize {
    expr2(&mut line.split_ascii_whitespace().peekable())
}

#[aoc(day18, part2)]
fn part2(lines: &[String]) -> usize {
    lines.iter().map(|l| eval2(l)).sum()
}

#[test]
fn test_small() {
    assert_eq!(eval2(&mut "1 + ( 2 * 3 ) + ( 4 * ( 5 + 6 ) )"), 51);
    assert_eq!(eval2(&mut "2 * 3 + ( 4 * 5 )"), 46);
    assert_eq!(eval2(&mut "5 + ( 8 * 3 + 9 + 3 * 4 * 3 )"), 1445);
    assert_eq!(eval2(&mut "5 * 9 * ( 7 * 3 * 3 + 9 * 3 + ( 8 + 6 * 4 ) )"), 669060);
    assert_eq!(eval2(&mut "( ( 2 + 4 * 9 ) * ( 6 + 9 * 8 + 6 ) + 6 ) + 2 + 4 * 2"), 23340);
    assert_eq!(eval2(&mut "( 2 * ( 2 * 2 ) + 1 ) + 1"), 11);
    
}

