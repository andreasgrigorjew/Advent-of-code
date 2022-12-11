use std::fs;

fn main() {
    let buffer = fs::read_to_string("./inp1.txt").expect("Error during reading");
    let v = buffer.split("\n\n");
    let mut xs = vec![];
    for s in v {
        let t = s.split("\n");
        let mut total = 0;
        for k in t {
            if k == "" {
                continue;
            }
            let nxt = k.parse::<i32>().unwrap();
            total += nxt;
        }
        xs.push(total);
    }
    xs.sort_by(|a, b| b.cmp(a));
    println!("{}", xs[0] + xs[1] + xs[2]);
}
