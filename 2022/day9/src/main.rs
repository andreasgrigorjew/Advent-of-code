use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use scanf::scanf;

fn adj(h: (i32, i32), t: (i32, i32)) -> bool {
    return h.0.abs_diff(t.0) <= 1 && h.1.abs_diff(t.1) <= 1;
}

fn main() {
    assert_eq!(env::args().len(), 1, "Use std in instead.");

    let n = 10;

    let dir_to_step = HashMap::from([('R', (1, 0)), ('U', (0, 1)), ('L', (-1, 0)), ('D', (0, -1))]);
    let mut rope = vec![(0, 0); n];
    let mut t_pos = HashSet::from([(0, 0)]);

    let mut dir = 'D';
    let mut steps = 0;
    while scanf!("{} {}\n", dir, steps).is_ok() {
        while steps > 0 {
            for i in (0..n).rev() {
                if i == n - 1 {
                    rope[i].0 += dir_to_step[&dir].0;
                    rope[i].1 += dir_to_step[&dir].1;
                } else if !adj(rope[i], rope[i + 1]) {
                    rope[i].0 += (rope[i + 1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i + 1].1 - rope[i].1).signum();
                }
            }
            steps -= 1;
            t_pos.insert(rope[0]);
        }
    }

    println!("{}", t_pos.len());
}
