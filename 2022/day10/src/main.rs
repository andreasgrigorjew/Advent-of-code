use std::env;
use std::fs;

fn parse(inp: &String) -> Vec<i32> {
    let mut ops = Vec::<i32>::new();
    for line in inp.split("\n") {
        ops.push(0);
        let p = line.split(" ").collect::<Vec::<&str>>();
        if p[0] == "addx" {
            ops.push(p[1].parse::<i32>().unwrap());
        }
    }
    return ops;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let inp = fs::read_to_string(&args[1]).expect("Error reading file");
    let ops = parse(&inp);

    let mut x = 1;
    let mut res1 = 0;
    for i in 1..ops.len()+1 {
        if [20, 60, 100, 140, 180, 220].contains(&i) {
            res1 += x * (i as i32);
        }
        x += ops[i-1];
    }
    println!("{}", res1);

    let mut x = 0;
    for i in 0..ops.len() {
        let p: i32 = (i % 40).try_into().unwrap();
        if p == 0 {
            println!();
        }
        let c = if p >= x && p < x + 3 {"#"} else {"."};
        print!("{}", c);

        x += ops[i];
    }
}
