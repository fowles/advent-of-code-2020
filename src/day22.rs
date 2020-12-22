use std::collections::HashSet;
use std::collections::VecDeque;

fn parse_deck(deck: &str) -> VecDeque<usize> {
    let mut lines = deck.lines();
    lines.next(); // burn the header
    lines.map(|card| card.parse().unwrap()).collect()
}

#[aoc_generator(day22)]
fn gen(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut p = input.split("\n\n");
    (
        p.next().map_or(VecDeque::new(), parse_deck),
        p.next().map_or(VecDeque::new(), parse_deck),
    )
}

fn play(decks: &mut (VecDeque<usize>, VecDeque<usize>)) {
    let c = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());
    if c.0 > c.1 {
        decks.0.push_back(c.0);
        decks.0.push_back(c.1);
    } else if c.0 < c.1 {
        decks.1.push_back(c.1);
        decks.1.push_back(c.0);
    } else {
        panic!();
    }
}

fn score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, v)| (idx + 1) * v)
        .sum()
}

#[aoc(day22, part1)]
fn part1(decks: &(VecDeque<usize>, VecDeque<usize>)) -> usize {
    let mut d = decks.clone();
    while !d.0.is_empty() && !d.1.is_empty() {
        play(&mut d);
    }
    if d.0.is_empty() {
        score(&d.1)
    } else {
        score(&d.0)
    }
}

fn play_recursive(decks: &mut (VecDeque<usize>, VecDeque<usize>)) {
    let c = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());

    let p0_win;
    if c.0 <= decks.0.len() && c.1 <= decks.1.len() {
        let mut rd = (
            decks.0.iter().take(c.0).cloned().collect(),
            decks.1.iter().take(c.1).cloned().collect(),
        );
        run_recursive(&mut rd);
        p0_win = rd.1.is_empty();
    } else {
        p0_win = c.0 > c.1
    };

    if p0_win {
        decks.0.push_back(c.0);
        decks.0.push_back(c.1);
    } else {
        decks.1.push_back(c.1);
        decks.1.push_back(c.0);
    }
}

fn run_recursive(decks: &mut (VecDeque<usize>, VecDeque<usize>)) {
    let mut seen = HashSet::new();
    loop {
        if decks.0.is_empty() || decks.1.is_empty() {
            break;
        }
        if seen.contains(&decks.0) {
            decks.1.clear();
            break;
        }

        let old = decks.clone();
        play_recursive(decks);
        seen.insert(old.0);
    }
}

#[aoc(day22, part2)]
fn part2(decks: &(VecDeque<usize>, VecDeque<usize>)) -> usize {
    let mut d = decks.clone();
    run_recursive(&mut d);

    if d.0.is_empty() {
        score(&d.1)
    } else {
        score(&d.0)
    }
}

#[test]
fn test_small() {
    let input = "
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
"
    .trim();
    assert_eq!(part1(&gen(input)), 306);
}
