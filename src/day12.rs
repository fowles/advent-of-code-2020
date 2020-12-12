#[aoc_generator(day12)]
fn gen(input: &str) -> Vec<(char, i64)> {
    input.lines().map(|l| {
        scan_fmt::scan_fmt!(l, "{[NSEWLRF]}{d}", char, i64).unwrap()
    }).collect()
}

fn rotate(h: char, d: i64) -> char {
    let headings = ['N', 'W', 'S', 'E'];
    let p = headings.iter().position(|c| *c == h).unwrap();
    let offset = (d.rem_euclid(360) / 90) as usize;
    headings[(p + offset) % headings.len()]
}

fn advance(h: char, p: (i64, i64), n: i64) -> (i64, i64) {
    match h {
        'N' => (p.0, p.1 + n),
        'S' => (p.0, p.1 - n),
        'E' => (p.0 + n, p.1),
        'W' => (p.0 - n, p.1),
        _ => panic!(),
    }
}

#[aoc(day12, part1)]
fn part1(cmds: &[(char, i64)]) -> usize {
    let mut p: (i64, i64) = (0, 0);
    let mut h = 'E';
    for (c, n) in cmds {
        match c {
            'L' => h = rotate(h, *n),
            'R' => h = rotate(h, -*n),
            'F' => p = advance(h, p, *n),
            _   => p = advance(*c, p, *n),
        }
    }
    (p.0.abs() + p.1.abs()) as usize
}

fn rotate_waypoint(w: (i64, i64), d: i64) -> (i64, i64) {
    match d.rem_euclid(360) {
        0   => w,
        90  => (-w.1, w.0),
        180 => (-w.0, -w.1),
        270 => (w.1, -w.0),
        _ => panic!(),
    }
}

#[aoc(day12, part2)]
fn part2(cmds: &[(char, i64)]) -> usize {
    let mut p: (i64, i64) = (0, 0);
    let mut w: (i64, i64) = (10, 1);
    for (c, n) in cmds {
        match c {
            'F' => {
                p.0 += w.0 * *n;
                p.1 += w.1 * *n;
            },
            'L' => w = rotate_waypoint(w, *n),
            'R' => w = rotate_waypoint(w, -*n),
            _   => w = advance(*c, w, *n),
        }
    }
    (p.0.abs() + p.1.abs()) as usize
}


#[test]
fn test_small() {
    let input = "
F10
N3
F7
R90
F11
".trim();
    assert_eq!(part1(&gen(input)), 25);
    assert_eq!(part2(&gen(input)), 286);
}

