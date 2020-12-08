use std::collections::HashSet;

#[aoc_generator(day8)]
fn gen(input: &str) -> Vec<(String, i64)> {
    input.lines().map(|s| {
        scan_fmt::scan_fmt!(s, "{} {}",  String, i64).unwrap()
    }).collect()
}

fn eval(prog: &[(String, i64)]) -> (i64, i64) {
    let mut acc: i64 = 0;
    let mut ic: i64 = 0;
    let mut seen : HashSet<i64> = vec![prog.len() as i64].into_iter().collect();

    while seen.insert(ic) {
        let p = &prog[ic as usize];
        match p.0.as_str() {
            "nop" => ic += 1,
            "jmp" => ic += p.1,
            "acc" => {
                acc += p.1;
                ic += 1;
            },
            _ => panic!(),
        }
    }
    (acc, ic)
}

#[aoc(day8, part1)]
fn part1(prog: &[(String, i64)]) -> i64 {
    eval(prog).0
}

#[aoc(day8, part2)]
fn part2(prog: &[(String, i64)]) -> i64 {
    for i in 0..prog.len() {
        let mut p = prog.to_vec();
        p[i].0 = match p[i].0.as_str() {
            "nop" => "jmp",
            "jmp" => "nop",
            "acc" => "acc",
            _ => panic!(),
        }.to_string();
        let (acc, ic) = eval(p.as_slice());
        if ic == p.len() as i64 {
            return acc;
        }
    }
    -1
}
