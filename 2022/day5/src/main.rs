use std::fs;

fn main() {
    let inp = fs::read_to_string("./5.in").expect("Error during reading");
    let v: Vec<&str> = inp.split("\n\n").collect();

    let rows: Vec<&str> = v[0].split("\n").collect();
    assert_eq!((rows[0].len() + 1)%4, 0);
    let m = (rows[0].len() + 1)/4; // number of columns
    let mut stacks = vec![vec![]; m];
    for i in 0..rows.len() {
        let mut iter = rows[i].chars();
        for j in 0..m {
            let k = iter.nth(1).unwrap();
            if k.is_digit(10) {
                break;
            }
            iter.nth(1);
            if k != ' ' {
                stacks[j].push(k);
            }
        }
    }
    for j in 0..m {
        stacks[j].reverse();
    }

    let cmds: Vec<&str> = v[1].split("\n").collect();
    for i in 0..cmds.len() {
        let s: Vec<char> = cmds[i].chars().collect();
        if s.is_empty() { continue; }
        let mut amount = 0;
        let mut idx = "move ".len();
        while s[idx] != ' ' {
            amount *= 10;
            amount += s[idx].to_digit(10).unwrap();
            idx += 1;
        }
        idx += "from ".len() + 1;
        // assuming that from and to are < 10
        let mut from: usize = s[idx].to_digit(10).unwrap().try_into().unwrap();
        let mut to: usize = s[idx + 5].to_digit(10).unwrap().try_into().unwrap();
        from -= 1;
        to -= 1;

        let mut newv = vec![];
        while amount > 0 {
            let top = stacks[from].pop().unwrap();
            newv.push(top);
            amount -= 1;
        }
        newv.reverse();
        for top in newv {
            stacks[to].push(top);
        }
    }

    for i in 0..m {
        print!("{}", stacks[i][stacks[i].len() - 1])
    }
    print!("\n")
}
