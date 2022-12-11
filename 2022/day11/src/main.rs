use std::collections::VecDeque;

// Example:
/*
let mut items = vec![VecDeque::from([79, 98]),
            VecDeque::from([54, 65, 75, 74]),
            VecDeque::from([79, 60, 97]),
            VecDeque::from([74])];
let ops: Vec<Box<dyn Fn(i64) -> i64>> = vec![Box::new(|x| x*19), Box::new(|x| x+6), Box::new(|x| x*x), Box::new(|x| x+3)];
let throws = vec![(2, 3), (2, 0), (1, 3), (0, 1)];
let check = vec![23, 19, 13, 17];
*/

fn main() {
    let mut items: Vec<VecDeque<i64>> = vec![VecDeque::from([65, 58, 93, 57, 66]),
                VecDeque::from([76, 97, 58, 72, 57, 92, 82]),
                VecDeque::from([90, 89, 96]),
                VecDeque::from([72, 63, 72, 99]),
                VecDeque::from([65]),
                VecDeque::from([97, 71]),
                VecDeque::from([83, 68, 88, 55, 87, 67]),
                VecDeque::from([64, 81, 50, 96, 82, 53, 62, 92])];
    let ops: Vec<Box<dyn Fn(i64) -> i64>> = vec![Box::new(|x| x*7), Box::new(|x| x+4), Box::new(|x| x*5), Box::new(|x| x*x), Box::new(|x| x+1), Box::new(|x| x+8), Box::new(|x| x+2), Box::new(|x| x+5)];
    let throws = vec![(6, 4), (7, 5), (5, 1), (0, 4), (6, 2), (7, 3), (2, 1), (3, 0)];
    let check = vec![19, 3, 13, 17, 2, 11, 5, 7];

    let mut inspected: Vec<i64> = vec![0; 8];
    for _ in 0..10_000 {
        for monkey in 0..8 {
            while let Some(mut w) = items[monkey].pop_front() {
                w %= 9699690; // lcm of check
                w = ops[monkey](w);
                w %= 9699690;
                //w = w / 3;
                if w % check[monkey] == 0 {
                    items[throws[monkey].0].push_back(w);
                } else {
                    items[throws[monkey].1].push_back(w);
                }
                inspected[monkey] += 1;
            }
        }
    }
    inspected.sort();
    println!("{}", inspected[6]*inspected[7] );
}
