use std::fs;
use std::collections::HashMap;

fn solve(rnds: &mut std::str::Split<'_, &str>) -> i32 {
    let mut m = HashMap::<char, i32>::new();
    m.insert('A', 0);
    m.insert('B', 1);
    m.insert('C', 2);
    let mut res = 0;
    for r in rnds {
        if r == "" {
            continue;
        }
        let L = r.chars().nth(0).unwrap();
        let R = r.chars().nth(2).unwrap();
        if R == 'X' {
            res += (m[&L] + 2) % 3 + 1;
        } else if R == 'Y' {
            res += m[&L] + 1 + 3;
        } else {
            res += (m[&L] + 1) % 3 + 1 + 6;
        }
    }
    res
}

fn main() {
    let inp = fs::read_to_string("./2.in").expect("Error reading");
    let mut rnds = inp.split("\n");
    let mut res = 0;
    res = solve(&mut rnds);
    println!("{}", res);
}
