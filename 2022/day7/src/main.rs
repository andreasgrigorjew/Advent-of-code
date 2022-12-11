use std::cmp;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::RefCell;
use std::string::String;

struct Graph {
    id: HashMap<String, i32>,
    edges: HashMap<i32, HashSet<i32>>,
    score: HashMap<i32, RefCell::<i32>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            id: HashMap::new(),
            edges: HashMap::new(),
            score: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: &String, idx: &mut i32) -> i32 {
        return match self.id.get(node) {
            Some(x) => *x,
            None => {
                *idx += 1;
                self.id.insert((*node).clone(), *idx);
                self.edges.insert(*idx, HashSet::new());
                self.score.insert(*idx, RefCell::new(0));
                *idx
            }
        };
    }

    fn parse(&mut self, inp: String) {
        let mut idx = -1;
        self.add_node(&"/".to_string(), &mut idx);
        let lines = inp.split("\n");

        let mut stack: Vec<i32> = vec![0];
        let mut name_stack: Vec<String> = vec!["/".to_string()];

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let chs: Vec<&str> = line.split(" ").collect();

            let cur_dir = match stack.last() {
                Some(x) => *x,
                None => 0,
            };

            match chs[0] {
                "$" => {
                    if chs[1] == "ls" {
                        continue;
                    }

                    if chs[2] == ".." {
                        stack.pop();
                        name_stack.pop();
                    } else if chs[2] == "/" {
                        stack = vec![0];
                        name_stack = vec!["/".to_string()];
                    } else {
                        let nv_name = name_stack.join("/");
                        let nv = self.add_node(&[nv_name, chs[2].to_string()].concat(), &mut idx);
                        self.edges.get_mut(&cur_dir).unwrap().insert(nv);
                        stack.push(nv);
                        name_stack.push(chs[2].to_string());
                    }
                },

                "dir" => {
                    let nv_name = name_stack.join("/");
                    let nv = self.add_node(&[nv_name, chs[1].to_string()].concat(), &mut idx);
                    self.edges.get_mut(&cur_dir).unwrap().insert(nv);
                },

                _ => {
                    let mut s = self.score.get_mut(&cur_dir).unwrap().borrow_mut();
                    *s += chs[0].parse::<i32>().unwrap();
                },
            }
        }
    }

    fn fill(&self, node: i32) {
        for nxt in self.edges.get(&node).unwrap() {
            self.fill(*nxt);
            let mut s = self.score.get(&node).unwrap().borrow_mut();
            *s += *self.score.get(nxt).unwrap().borrow();
        }
    }
}

fn main() {
    let inp = fs::read_to_string("./7.in").expect("Error reading file");
    
    let mut g = Graph::new();
    g.parse(inp);
    g.fill(0);

    let mut res1 = 0;
    for (_, v) in &g.score {
        let vb = *v.borrow();
        if vb > 100_000 {
            continue;
        }
        res1 += vb;
    }
    println!("{}", res1);

    let total = 70_000_000;
    let need = 30_000_000;
    let occ = *g.score.get(&0).unwrap().borrow();
    let mut res2 = total + 100;
    for (_, v) in g.score {
        let vb = *v.borrow();
        if total - occ + vb >= need {
            res2 = cmp::min(res2, vb);
        }
    }
    println!("{}", res2);
}
