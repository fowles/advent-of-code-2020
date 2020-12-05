#[aoc_generator(day5)]
fn gen(input: &str) -> Vec<i64> {
    input.split_whitespace().map(|s| {
        s.chars().map(|c| {
            match c {
                'F' => '0',
                'B' => '1',
                'L' => '0',
                'R' => '1',
                _ => panic!("Unable to parse {} invalid char {}", s, c)
            }
        }).collect()
    }).map(|b: String| {
        let row = i64::from_str_radix(&b[..7], 2).unwrap();
        let column = i64::from_str_radix(&b[7..], 2).unwrap();
        row*8 + column
    })
    .collect()
}

#[test]
fn parsing() {
    // BFFFBBFRRR: row 70, column 7, seat ID 567.
    // FFFBBBFRRR: row 14, column 7, seat ID 119.
    // BBFFBBFRLL: row 102, column 4, seat ID 820
    let input = "BFFFBBFRRR FFFBBBFRRR BBFFBBFRLL";
    assert_eq!(gen(input), vec![567, 119, 820])
}

#[aoc(day5, part1)]
fn part1(boarding: &[i64]) -> i64 {
    *boarding.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn part2(boarding: &[i64]) -> i64 {
    let mut seats = boarding.to_vec();
    seats.sort_unstable();
    for i in 1..seats.len() {
        if seats[i] != seats[i - 1] + 1{
            return seats[i] - 1;
        }
    }
    return -1;
}
