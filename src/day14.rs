use std::collections::HashMap;
use bit_vec::BitVec;
use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Mask { zeros: usize, ones: usize, exes: BitVec },
    Assign { loc: usize, val: usize },
}

fn parse_mask(mask: &str) -> Instruction {
    let mut ones = 0;
    let mut zeros = 0;
    let mut exes = BitVec::from_elem(36, false);
    for (idx, c) in mask.chars().rev().enumerate() {
        match c {
            '0' => {
                zeros |= 1 << idx;
            },
            '1' => {
                ones |= 1 << idx;
            },
            'X' => {
                exes.set(idx, true);
            },
            _ => panic!(),
        }
    }
    Instruction::Mask {
        ones: ones,
        zeros: zeros,
        exes: exes,
    }
}

#[aoc_generator(day14)]
fn gen(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|l| {
             if let Ok(mask) = scan_fmt::scan_fmt!(l, "mask = {}", String) {
                 return parse_mask(mask.as_str());
             }
             if let Ok((loc, val)) = scan_fmt::scan_fmt!(l, "mem[{d}] = {d}", usize, usize) {
                return Instruction::Assign {
                    loc: loc,
                    val: val,
                };
             }
             panic!();
        }).collect()
}


#[aoc(day14, part1)]
fn part1(p: &Vec<Instruction>) -> usize {
    let mut mem = HashMap::<usize, usize>::new();

    let mut and = 0;
    let mut or = 0;
    for i in p {
        match i {
            Instruction::Mask{ones, zeros, exes: _} => {
                and = !*zeros;
                or = *ones;
            },
            Instruction::Assign{loc, val} => {
                *mem.entry(*loc).or_default() = (val & and) | or;
            },
        }
    }
    mem.values().sum()
}

#[aoc(day14, part2)]
fn part2(p: &Vec<Instruction>) -> usize {
    let mut mem = HashMap::<usize, usize>::new();

    let mut or = 0;
    let mut float = BitVec::from_elem(36, false);
    for i in p {
        match i {
            Instruction::Mask{ones, zeros: _, exes} => {
                or = *ones;
                float = exes.clone();
            },
            Instruction::Assign{loc, val} => {
                let mut addr = loc | or;
                for (idx, b) in float.iter().enumerate() {
                    if b {
                        addr &= !(1 << idx);
                    }
                }

                for i in 0..=float.len() {
                    let masks = float.iter()
                        .enumerate()
                        .filter(|(_, b)| *b)
                        .map(|(idx, _)| idx)
                        .combinations(i);
                    for mask_indexes in masks {
                        let mut mask = 0;
                        for i in &mask_indexes {
                            mask |= 1 << i;
                        }
                        *mem.entry(addr | mask).or_default() = *val;
                    }
                }
            },
        }
    }
    mem.values().sum()
}

#[test]
fn test_part1() {
    let input = "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
".trim();
    assert_eq!(part1(&gen(input)), 165);
}

#[test]
fn test_part2() {
    let input = "
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
".trim();
    assert_eq!(part2(&gen(input)), 208);
}

