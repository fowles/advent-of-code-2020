use std::collections::HashSet;

#[derive(Default, Clone, Debug)]
struct State {
    active: HashSet<(i64, i64, i64, i64)>,
}

impl State {
    fn adjacent<F>(&self, p: (i64, i64, i64, i64), mut f: F)
        where F : FnMut((i64, i64, i64, i64)) {
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for h in -1..=1 {
                        if i != 0 || j != 0 || k != 0 || h != 0 {
                            f((p.0 + i, p.1 + j, p.2 + k, p.3 + h));
                        }
                    }
                }
            }
        }
    }

    fn count(&self, p: (i64, i64, i64, i64)) -> usize {
        let mut r = 0;
        self.adjacent(p, |o| {
            if self.active.contains(&o) {
                r += 1;
            }
        });
        r
    }

    fn step(&self) -> State {
        let mut next: State = Default::default();
        for a in &self.active {
            self.adjacent(*a, |p| {
                let c = self.count(p);
                if self.active.contains(&p) {
                    if c == 2 || c == 3 {
                        next.active.insert(p);
                    }
                } else {
                    if c == 3 {
                        next.active.insert(p);
                    }
                }
            });
        }
        next
    }
}

#[aoc_generator(day17)]
fn gen(input: &str) -> State {
    let n = input.lines().count() as i64;

    let mut s: State = Default::default();
    for (idx, b) in input.chars().filter(|s| !s.is_whitespace()).enumerate() {
        let i = idx as i64;
        assert!(i < n*n);
        if b == '#' {
            s.active.insert((i/n, i%n, 0, 0));
        }
    }
    s
}



#[aoc(day17, part2)]
fn part1(orig: &State) -> usize {
    let mut s = orig.clone();
    for _ in 0..6 {
        s = s.step();
    }
    s.active.len()
}

