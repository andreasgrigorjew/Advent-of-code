use std::fs;

fn inside(a: u32, b: u32, c: u32, d: u32) -> bool {
    return b >= c && a <= d;
}

fn main() {
    let inp = fs::read_to_string("./4.in").expect("Error reading");
    let sp = inp.split("\n");
    let mut res = 0;
    for line in sp {
        if line.is_empty() { continue; }
        let mut v = vec![0; 4];
        let mut vidx = 0;
        for c in line.chars() {
            match c {
                ',' => vidx += 1,
                '-' => vidx += 1,
                _ => {
                    v[vidx] *= 10;
                    v[vidx] += c.to_digit(10).unwrap();
                },
            }
        }
        assert!(vidx == 3);
        if inside(v[0], v[1], v[2], v[3]) || inside(v[2], v[3], v[0], v[1]) {
            res += 1;
        }
    }
    println!("{}", res);
}
