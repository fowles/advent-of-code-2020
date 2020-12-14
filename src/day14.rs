use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Mask { and: usize, or: usize },
    Assign { loc: String, val: usize },
}

fn parse_mask(mask: &str) -> Instruction {
    let mut and = (2 as usize).pow(36) - 1;
    let mut or = 0;
    for (idx, c) in mask.chars().rev().enumerate() {
        match c {
            '0' => {
                and &= !(1 << idx);
            },
            '1' => {
                or |= 1 << idx;
            },
            _ => {},
        }
    }
    Instruction::Mask {
        and: and,
        or: or,
    }
}

#[aoc_generator(day14)]
fn gen(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|l| l.split(" = ").collect::<Vec<&str>>())
        .map(|v| (v[0], v[1]))
        .map(|(key, val)| {
            if key == "mask" {
                return parse_mask(val);
            } else {
                return Instruction::Assign {
                    loc: key.to_string(),
                    val: val.parse().unwrap(),
                };
            }
        }).collect()
}


#[aoc(day14, part1)]
fn part1(p: &Vec<Instruction>) -> usize {
    let mut mem = HashMap::<String, usize>::new();

    let mut a = (2 as usize).pow(36) - 1;
    let mut o = 0;
    for i in p {
        match i {
            Instruction::Mask{and, or} => {
                a = *and;
                o = *or;
            },
            Instruction::Assign{loc, val} => {
                *mem.entry(loc.to_string()).or_default() = (val & a) | o;
            },
        }
    }
    mem.values().sum()
}

#[test]
fn test_small() {
    let input = "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
".trim();
    assert_eq!(part1(&gen(input)), 165);
}

