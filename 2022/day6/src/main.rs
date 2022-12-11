use std::fs;
use std::collections::HashSet;

fn main() {
    let inp: Vec<char> = fs::read_to_string("./6.in").expect("Error reading").chars().collect();
    let n: usize = inp.len();
    let mut res: i32 = 0;
    let k = 14;
    for i in 0..n-4 {
        let mut s: HashSet<char> = HashSet::new();
        for j in 0..k {
            s.insert(inp[i+j]);
        }
        // res += if s.len() == 4 { 1 } else { 0 };
        if s.len() == k {
            println!("{}", i+k);
            return;
        }
    }
    //println!("{}", res);
}
