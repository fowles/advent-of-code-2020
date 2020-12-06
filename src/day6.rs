use std::collections::HashSet;

#[aoc_generator(day6)]
fn gen1(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|family| {
            family.lines()
                .map(|member| member.chars().collect())
                .collect()
        })
        .collect()
}

// TODO figure out why I need clone/cloned in these methods
#[aoc(day6, part1)]
fn part1(answers: &[Vec<HashSet<char>>]) -> usize {
    answers.iter().map(|family| {
        let mut r = family[0].clone();
        for member in family {
            r = r.union(&member).cloned().collect();
        }
        r.len()
    }).sum()
}

#[aoc(day6, part2)]
fn part2(answers: &[Vec<HashSet<char>>]) -> usize {
    answers.iter().map(|family| {
        let mut r = family[0].clone();
        for member in family {
            r = r.intersection(&member).cloned().collect();
        }
        r.len()
    }).sum()
}

