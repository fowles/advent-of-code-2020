use std::collections::HashMap;

#[aoc_generator(day23)]
fn gen(input: &str) -> Vec<usize> {
    input.chars().map(|c| c as usize - '0' as usize).collect()
}

struct Node {
    v: usize,
    next: *mut Node,
}

fn do_move(n: *mut Node, nodes: &HashMap<usize, *mut Node>) {
    unsafe {
        let mut dest_val = (*n).v - 1;
        if dest_val == 0 {
            dest_val = nodes.len();
        }

        let slice = (*n).next;

        let mut pos;
        'rescan: loop {
            pos = slice;
            for _ in 0..3 {
                if (*pos).v == dest_val {
                    dest_val -= 1;
                    if dest_val == 0 {
                        dest_val = nodes.len();
                    }
                    continue 'rescan;
                }
                pos = (*pos).next;
            }
            break;
        }
        (*n).next = pos;


        let pos: *mut Node = *nodes.get(&dest_val).unwrap();
        let pv = (*pos).next;
        (*pos).next = slice;
        (*(*(*slice).next).next).next = pv;
    }
}

fn build_nodes(nums: &Vec<usize>, len: usize) -> (Vec<Node>, HashMap<usize, *mut Node>) {
    let mut nodes = Vec::<Node>::new();
    for n in nums {
        nodes.push(Node {
            v: *n,
            next: std::ptr::null_mut(),
        });
    }

    for i in nodes.len()..len {
        nodes.push(Node {
            v: i+1,
            next: std::ptr::null_mut(),
        });
    }

    for i in 0..nodes.len() - 1 {
        nodes[i].next = &mut nodes[i + 1];
    }
    nodes.last_mut().unwrap().next = &mut nodes[0];

    let mut m = HashMap::<usize, *mut Node>::new();
    for n in &mut nodes {
        m.insert(n.v, &mut *n);
    }
    (nodes, m)
}

#[allow(dead_code)]
fn print(n: *mut Node) {
    unsafe {
        let mut pos = n;
        loop {
            print!("{} ", (*pos).v);
            if (*pos).next == n {
                break;
            }
            pos = (*pos).next;
        }
        println!("");
    }
}

#[aoc(day23, part1)]
fn part1(nums: &Vec<usize>) -> usize {
    let (mut nodes, map) = build_nodes(nums, nums.len());
    let mut cup: *mut Node = nodes.first_mut().unwrap();
    for _ in 0..100 {
        do_move(cup, &map);
        unsafe {
            cup = (*cup).next;
        }
    }

    let mut acc = 0;
    let one = *map.get(&1).unwrap();
    unsafe {
        cup = one;
        loop {
            cup = (*cup).next;
            if cup == one {
                break;
            }
            acc = 10*acc + (*cup).v;
        }
    }
    acc
}

#[aoc(day23, part2)]
fn part2(nums: &Vec<usize>) -> usize {
    let (mut nodes, map) = build_nodes(nums, 1000000);
    let mut cup: *mut Node = nodes.first_mut().unwrap();
    for _ in 0..10000000 {
        do_move(cup, &map);
        unsafe {
            cup = (*cup).next;
        }
    }

    let one = *map.get(&1).unwrap();
    unsafe {
        println!("{}", (*one).v);
        println!("{}", (*(*one).next).v);
        println!("{}", (*(*(*one).next).next).v);
        return (*(*one).next).v * (*(*(*one).next).next).v;
    }
}

#[test]
fn test_small() {
    assert_eq!(part1(&gen("389125467")), 67384529);
    // assert_eq!(part2(&gen("389125467")), 149245887792);
}
