
#[derive(Clone, PartialEq)]
struct Seats {
    state: Vec<Vec<char>>,
}

impl Seats {
    fn total_occupied(&self) -> usize {
        let mut r = 0;
        for row in &self.state {
            for c in row {
                if *c == '#' {
                    r += 1;
                }
            }
        }
        r
    }

    fn occupied(&self, i: i64, j: i64) -> usize {
        if 0 <= i && i < self.state.len() as i64 &&
            0 <= j && j < self.state[i as usize].len() as i64 {
           if self.state[i as usize][j as usize] == '#' {
               return 1;
           }
        }
        0
    }

    fn occupied_vis(&self, mut i: i64, mut j: i64, di: i64, dj: i64) -> usize {
        i += di;
        j += dj;
        while 0 <= i && i < self.state.len() as i64 &&
              0 <= j && j < self.state[i as usize].len() as i64 {
            match self.state[i as usize][j as usize] {
                '.' => {
                    i += di;
                    j += dj;
                    continue;
                },
                '#' => return 1,
                'L' => return 0,
                _ => panic!(),
            }
        }
        0
    }

    fn count(&self, i: usize, j: usize) -> usize {
        let mut r = 0;
        for di in &[-1 as i64, 0, 1] {
            for dj in &[-1 as i64, 0, 1] {
                if *di != 0 || *dj != 0 {
                    r += self.occupied(i as i64 + *di, j as i64 + *dj);
                }
            }
        }
        r
    }

    fn count_vis(&self, i: usize, j: usize) -> usize {
        let mut r = 0;
        for di in &[-1, 0, 1] {
            for dj in &[-1, 0, 1] {
                if *di != 0 || *dj != 0 {
                    r += self.occupied_vis(i as i64, j as i64, *di, *dj);
                }
            }
        }
        r
    }

    #[allow(dead_code)]
    fn to_str(&self) -> String {
        let mut r: String = "".to_string();
        for row in &self.state {
            for c in row {
                r.push(*c);
            }
            r.push('\n');
        }
        r
    }

    fn step1(&self) -> Seats {
        let mut r = self.clone();
        for i in 0..self.state.len() {
            for j in 0..self.state[i].len() {
                match self.state[i][j] {
                    'L' => {
                        if self.count(i, j) == 0 {
                            r.state[i][j] = '#';
                        }
                    },
                    '#' => {
                        if self.count(i, j) >= 4 {
                            r.state[i][j] = 'L';
                        }
                    },
                    _ => {
                    },
                }
            }
        }
        r
    }

    fn step2(&self) -> Seats {
        let mut r = self.clone();
        for i in 0..self.state.len() {
            for j in 0..self.state[i].len() {
                match self.state[i][j] {
                    'L' => {
                        if self.count_vis(i, j) == 0 {
                            r.state[i][j] = '#';
                        }
                    },
                    '#' => {
                        if self.count_vis(i, j) >= 5 {
                            r.state[i][j] = 'L';
                        }
                    },
                    _ => {
                    },
                }
            }
        }
        r
    }
}

#[aoc_generator(day11)]
fn gen(input: &str) -> Seats {
    Seats {
        state: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

#[aoc(day11, part1)]
fn part1(orig: &Seats) -> usize {
    let mut s = orig.clone();
    for _ in 0.. {
        let next = s.step1();
        if s == next {
            return s.total_occupied();
        }
        s = next.clone();
    }
    0
}

#[aoc(day11, part2)]
fn part2(orig: &Seats) -> usize {
    let mut s = orig.clone();
    for _ in 0.. {
        let next = s.step2();
        if s == next {
            return s.total_occupied();
        }
        s = next.clone();
    }
    0
}

#[test]
fn test_small() {
    let input = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
".trim();
    assert_eq!(part1(&gen(input)), 37);
    assert_eq!(part2(&gen(input)), 26);
}

