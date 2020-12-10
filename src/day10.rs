#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<usize> {
    let mut v: Vec<usize> = input.lines().map(|s| s.parse().unwrap()).collect();
    v.push(0);
    v.push(v.iter().max().unwrap() + 3);
    v.sort_unstable();
    v
}


#[aoc(day10, part1)]
fn part1(nums: &[usize]) -> usize {
    let mut steps: [usize; 4] = [0, 0, 0, 0];
    for w in nums.windows(2) {
        if let &[a, b] = w {
            steps[b - a] += 1;
        }
    }
    return steps[1] * steps[3];
}

#[aoc(day10, part2)]
fn part2(nums: &[usize]) -> usize {
    let mut ways = vec![0; nums.len()];
    ways[0] = 1;
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[j] - nums[i] <= 3 {
                ways[j] += ways[i];
            }
        }
    }
    ways[ways.len() - 1]
}

#[test]
fn test_small() {
    let input = "
16
10
15
5
1
11
7
19
6
12
4
".trim();
    assert_eq!(part1(&gen(input)), 35);
    assert_eq!(part2(&gen(input)), 8);
}

