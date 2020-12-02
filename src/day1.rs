#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<i64> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(expenses: &[i64]) -> i64 {
    for i in expenses {
        for j in expenses {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    return -1;
}

#[aoc(day1, part2)]
fn part2(expenses: &[i64]) -> i64 {
    for i in expenses {
        for j in expenses {
            for k in expenses {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    return -1;
}
