use scan_fmt::scan_fmt;
use std::collections::HashMap;


#[derive(Debug)]
struct Tile {
    id: usize,
    pixels: Vec<String>,
}

impl Tile {
    fn border(&self, b: usize) -> String {
        match b {
            0 => self.pixels.first().unwrap().clone(),
            1 => {
                let mut r = String::new();
                for p in &self.pixels {
                    r.push_str(&p[0..1]);
                }
                r
            },
            2 => self.pixels.last().unwrap().clone(),
            3 => {
                let mut r = String::new();
                for p in &self.pixels {
                    r.push_str(&p[p.len() - 1..p.len()]);
                }
                r
            },
            _ => panic!(),
        }
    }
}

fn parse_tile(input: &str) -> Tile {
    let mut lines = input.lines();
    Tile {
        id: scan_fmt!(lines.next().unwrap(), "Tile {d}:", usize).unwrap(),
        pixels: lines.map(|l| l.to_string()).collect(),
    }
}


#[aoc_generator(day20, part1)]
fn gen(input: &str) -> Vec<Tile> {
    input.split("\n\n").map(|t| parse_tile(t)).collect()
}


#[aoc(day20, part1)]
fn part1(tiles: &Vec<Tile>) -> usize {
    let mut pairings = HashMap::<String, usize>::new();
    for t in tiles {
        for i in 0..4 {
            let b = t.border(i);
            *pairings.entry(b.clone()).or_default() += 1;
            let r = b.chars().rev().collect::<String>();
            if r != b {
                *pairings.entry(r).or_default() += 1;
            }
        }
    }

    let mut answer = 1;
    let mut count = 0;
    for t in tiles {
        let mut unmatched = 0;
        for i in 0..4 {
            let b = t.border(i);
            let r = b.chars().rev().collect::<String>();
            let c = std::cmp::max(
                pairings.get(&b).unwrap(),
                pairings.get(&r).unwrap());
            match c {
                1 => {
                    unmatched += 1;
                },
                2 => {
                    // matched
                },
                _ => panic!(),  // our cheating caught up with us
            }
        }
        if unmatched == 2 {
            count += 1;
            answer *= t.id;
        }
    }
    assert!(count == 4);
    answer
}


