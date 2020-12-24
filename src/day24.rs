use regex::Regex;
use std::collections::HashSet;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Hex {
    x: i64,
    y: i64,
    z: i64,
}

impl Hex {
    fn step(&mut self, dir: &str) {
        match dir {
            "ne" => {
                self.x += 1;
                self.y += 0;
                self.z -= 1;
            },
            "nw" => {
                self.x += 0;
                self.y += 1;
                self.z -= 1;
            },
            "se" => {
                self.x += 0;
                self.y -= 1;
                self.z += 1;
            },
            "sw" => {
                self.x -= 1;
                self.y += 0;
                self.z += 1;
            },
            "e" => {
                self.x += 1;
                self.y -= 1;
                self.z += 0;
            },
            "w" => {
                self.x -= 1;
                self.y += 1;
                self.z += 0;
            },
            _ => panic!(),
        }
    }
}

fn adjacent<F>(floor: &HashSet<Hex>, p: &Hex, mut f: F)
    where F : FnMut(&Hex, bool) {
        for dir in &["ne", "e", "se", "sw", "w", "nw"] {
            let mut h = p.clone();
            h.step(dir);
            f(&h, floor.contains(&h));
        }
    }

fn count(floor: &HashSet<Hex>, h: &Hex) -> usize {
    let mut c = 0;
    adjacent(&floor, &h, |_, b| {
        if b {
            c += 1;
        }
    });
    c
}

fn advance(floor: &HashSet<Hex>) -> HashSet<Hex> {
    let mut next = HashSet::<Hex>::new();
    for h in floor {
        adjacent(&floor, &h, |tile, color| {
            // Any black tile with zero or more than 2 black tiles immediately adjacent to it is
            // flipped to white.
            //
            // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to
            // black.
            let c = count(&floor, &tile);
            if color {
                if 1 <= c && c <= 2 {
                    next.insert(*tile);
                } 
            } else {
                if c == 2 {
                    next.insert(*tile);
                }
            }
        });
    }
    next
}

#[aoc_generator(day24)]
fn gen(input: &str) -> HashSet<Hex> {
    let re = Regex::new("[ns]?[ew]").unwrap();

    let tiles = input.lines().map(|line| {
        let mut h: Hex = Default::default();
        for m in re.find_iter(line) {
            h.step(m.as_str());
        }
        h
    }).collect::<Vec<Hex>>();

    let mut floor = HashSet::new();
    for t in tiles {
        if !floor.insert(t) {
            floor.remove(&t);
        }
    }
    floor
}


#[aoc(day24, part1)]
fn part1(floor: &HashSet<Hex>) -> usize {
    floor.len()
}

#[aoc(day24, part2)]
fn part2(floor: &HashSet<Hex>) -> usize {
    let mut state = floor.clone();
    for _ in 0..100 {
        state = advance(&state);
    }
    state.len()
}

#[test]
fn test_small() {
    let input = "
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
".trim();
    assert_eq!(part1(&gen(input)), 10);
    assert_eq!(part2(&gen(input)), 2208);
}

