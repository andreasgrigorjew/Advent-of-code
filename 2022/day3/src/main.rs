use std::fs; 
use std::collections::HashMap;
use std::collections::HashSet;
 
fn val(c: char) -> u32 {
    let ret: u32 = (c as u32) - if c.is_uppercase() { ('A' as u32) - 27 } else { ('a' as u32) - 1 };
    return ret;
}

fn main() {
    let buf: &str = &fs::read_to_string("./3.in").expect("Error during reading");
    let backpacks: Vec<&str> = buf.split("\n").collect();
    let mut res: u32 = 0;
    let mut seen: HashMap<char, i32> = HashMap::new();
    let mut g = 0;
    for bp in backpacks {
        let mut done: HashSet<char> = HashSet::new();
        let n: usize = bp.len();
        let mut iter = bp.chars();
        for _i in 0..n {
            let c: char = iter.next().unwrap();
            if done.contains(&c) { continue; }
            done.insert(c);

            let upd = seen.entry(c).or_insert(0);
            *upd += 1;
        }

        g += 1;
        if g == 3 {
            g = 0;
            for (c, v) in &seen {
                if *v == 3 {
                    res += val(*c);
                }
            }
            seen = HashMap::new();
        }
    }
    println!("{}", res);
}
