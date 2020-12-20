use std::collections::HashMap;

#[aoc_generator(day15)]
fn gen(input: &str) -> Vec<usize> {
    input.split(",").map(|s| s.parse().unwrap()).collect()
}

fn play(nums: &[usize], turns: usize) -> usize {
    let mut seen = HashMap::<usize, Vec<usize>>::new();
    let mut spoken = 0;

    for i in 0..nums.len() {
        spoken = nums[i];
        seen.entry(spoken).or_default().push(i);
    }
    for i in nums.len()..turns {
        let v = seen.get(&spoken).unwrap();
        if v.len() == 1 {
            spoken = 0;
        } else {
            spoken = v[v.len() - 1] - v[v.len() - 2];
        }
        seen.entry(spoken).or_default().push(i);
    }
    spoken
}


#[aoc(day15, part1)]
fn part1(nums: &[usize]) -> usize {
    play(nums, 2020)
}

#[aoc(day15, part2)]
fn part2(nums: &[usize]) -> usize {
    play(nums, 30000000)
}

#[test]
fn test_small() {
    assert_eq!(part1(&gen("0,3,6")), 436);
}

