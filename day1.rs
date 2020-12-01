
fn main() {
    let expenses: Vec<i64> = std::fs::read_to_string("day1-input.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    for i in &expenses {
        for j in &expenses {
            for k in &expenses {
                if i + j + k == 2020 {
                    println!("{}", i*j*k);
                }
            }
        }
    }
}
