#[aoc_generator(day9)]
fn gen(input: &str) -> Vec<i64> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

fn find_sum(nums: &[i64], val: i64) -> bool {
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == val {
                return true;
            }
        }
    }
    return false;
}

#[aoc(day9, part1)]
fn part1(nums: &[i64]) -> i64 {
    for i in 25..nums.len() {
        if !find_sum(&nums[i-25..i], nums[i]) {
            return nums[i];
        }
    }
    return 0;
}

#[aoc(day9, part2)]
fn part2(nums: &[i64]) -> i64 {
    let target = part1(nums);
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            let range = &nums[i..j];
            if range.iter().sum::<i64>() == target {
                return range.iter().min().unwrap() + 
                       range.iter().max().unwrap();
            }
        }
    }
    return 0;
}
