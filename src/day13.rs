#[aoc_generator(day13)]
fn gen(input: &str) -> (usize, Vec<(usize, usize)>) {
    let mut iter = input.lines();
    let time = iter.next().unwrap().parse::<usize>().unwrap();
    let schedule = iter.next().unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, v)| *v != "x")
        .map(|(idx, v)| (idx, v.parse::<usize>().unwrap()))
        .collect();
    (time, schedule)
}


#[aoc(day13, part1)]
fn part1(s: &(usize, Vec<(usize, usize)>)) -> usize {
    let mut min = s.0;
    let mut r = 0;
    for (_, t) in &s.1 {
        let wait = (s.0 / t + 1) * t - s.0;
        if wait < min {
            min = wait;
            r = wait * *t;
        }
    }
    r
}

fn inv(v: usize, m: usize) -> usize {
    for i in 0..m {
        if (v * i) % m == 1 {
            return i;
        }
    }
    panic!();
}

#[aoc(day13, part2)]
fn part2(s: &(usize, Vec<(usize, usize)>)) -> usize {
    // Chinese Remainder Theorem:
    //
    // x =  ( ∑ (rem[i]*pp[i]*inv[i]) ) % prod
    // Where 0 <= i <= n-1
    //
    // rem[i] is given array of remainders
    //
    // prod is product of all given numbers
    // prod = num[0] * num[1] * ... * num[k-1]
    //
    // pp[i] is product of all divided by num[i]
    // pp[i] = prod / num[i]
    //
    // inv[i] = Modular Multiplicative Inverse of 
    //         pp[i] with respect to num[i]

    let prod = s.1.iter().map(|(_, v)| v).product::<usize>();
    let mut sum = 0;
    for (rem, val) in &s.1 {
        let pp = prod/val;
        sum += rem * pp * inv(pp, *val);
    }

    prod - sum % prod
}

#[test]
fn test_small() {
    let input = "
939
7,13,x,x,59,x,31,19
".trim();
    assert_eq!(part1(&gen(input)), 295);
    assert_eq!(part2(&gen(input)), 1068781);
    assert_eq!(part2(&gen("1\n17,x,13,19")), 3417);
    assert_eq!(part2(&gen("1\n67,7,59,61")), 754018);
    assert_eq!(part2(&gen("1\n67,x,7,59,61")), 779210);
    assert_eq!(part2(&gen("1\n67,7,x,59,61")), 1261476);
    assert_eq!(part2(&gen("1\n1789,37,47,1889")), 1202161486);
}

