struct Keys {
    card: usize,
    door: usize,
}

#[aoc_generator(day25)]
fn gen(input: &str) -> Keys {
    let mut l = input.split_whitespace();
    Keys {
        card: l.next().unwrap().parse().unwrap(),
        door: l.next().unwrap().parse().unwrap(),
    }
}

fn xform(v: usize, subject_number: usize) -> usize {
    (v * subject_number) % 20201227
}

fn apply_loop(num: usize, subject_number: usize) -> usize {
    let mut v = 1;
    for _ in 0..num {
        v = xform(v, subject_number);
    }
    v
}

fn crack_loop_size(key: usize) -> usize {
    let mut i = 0;
    let mut v = 1;
    loop {
        i += 1;
        v = xform(v, 7);
        if v == key {
            return i;
        }
    }
}


#[aoc(day25, part1)]
fn part1(keys: &Keys) -> usize {
    let s = crack_loop_size(keys.card);
    apply_loop(s, keys.door)
}


#[test]
fn test_small() {
    assert_eq!(crack_loop_size(5764801), 8);
    assert_eq!(crack_loop_size(17807724), 11);
    assert_eq!(apply_loop(8, 17807724), 14897079);
    assert_eq!(apply_loop(11, 5764801), 14897079);
}

