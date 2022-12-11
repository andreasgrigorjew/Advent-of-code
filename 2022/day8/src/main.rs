use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let binding = fs::read_to_string(file).expect("Error reading file");
    let inp: Vec<&str> = binding.split("\n").collect();
    
    let mut a: Vec<Vec<char>> = inp.into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect();
    a.pop(); // remove empty line
    for k in &a {
        for c in k {
            print!("{}", c);
        }
        println!();
    }
    println!();
    let (n, m) = (a.len(), a[0].len());
    let mut b = vec![vec!['a'; n]; m];
    for j in 0..m {
        for i in 0..n {
            b[j][i] = a[i][j];
        }
        b[j].reverse();
    }

    for k in b {
        for c in k {
            print!("{}", c);
        }
        println!();
    }
}
