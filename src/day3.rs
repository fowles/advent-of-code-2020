#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect()
}

fn test_slope(trees: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
    let mut collisions = 0;
    let mut x = 0;
    let mut y = 0;
    while y < trees.len() {
        if trees[y][x] {
            collisions += 1;
        }
        y += dy;
        x += dx;
        x %= trees[0].len();
    }

    return collisions;
}

#[aoc(day3, part1)]
fn part1(trees: &Vec<Vec<bool>>) -> usize {
    return test_slope(trees, 3, 1);
}

#[aoc(day3, part2)]
fn part2(trees: &Vec<Vec<bool>>) -> usize {
    return test_slope(trees, 1, 1)
        * test_slope(trees, 3, 1)
        * test_slope(trees, 5, 1)
        * test_slope(trees, 7, 1)
        * test_slope(trees, 1, 2);
}
