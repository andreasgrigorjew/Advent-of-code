use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let inp = fs::read_to_string(file).expect("Error reading from file");

    let key = 811589153;

    let mut vals: Vec<(usize, i64)> = Vec::new();
    for (i, vs) in inp.split("\n").enumerate() {
        if vs == "" {
            break;
        }
        let v: i64 = vs.parse().unwrap();
        vals.push((i, v*key));
    }

    let n = vals.len();
    let ni = n as i64 - 1;

    let mut pos: HashMap<(usize, i64), usize> = HashMap::new();
    for (i, v) in &vals {
        pos.insert((*i, *v), *i);
    }
    let mut decr = vals.clone();

    for _ in 0..10 {
        for (i, v) in &vals {
            let p = pos[&(*i, *v)];
            let dest = (((v+p as i64)%ni+ni)%ni) as usize;
            let mut current_pos = p;
            while current_pos != dest {
                let nxt = if dest < p { current_pos-1 } else { current_pos+1 };
                decr.swap(nxt, current_pos);
                *pos.entry(decr[current_pos]).or_insert(current_pos) = current_pos;
                *pos.entry(decr[nxt]).or_insert(nxt) = nxt;
                current_pos = nxt;
            }
        }
    }

    let z = decr.iter().position(|x| x.1 == 0).unwrap();
    let i = (z+1000) % n;
    let j = (z+2000) % n;
    let k = (z+3000) % n;
    println!("{}", decr[i].1 + decr[j].1 + decr[k].1);
}
