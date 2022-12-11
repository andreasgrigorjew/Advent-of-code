use std::fs;
use std::collections::HashMap;
use std::string::String;

struct Graph {
    edges: HashMap<String, Vec<String>>,
    score: HashMap<String, i32>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            score: HashMap::new(),
        }
    }

    fn parse(&mut self, inp: String) {
        let lines = inp.split("\n");

        let mut stack: Vec<String> = vec!["/".to_string()];

        for line in lines {
            let chs: Vec<&str> = line.split(" ").collect();

            let cur_dir: String = match stack.last() {
                Some(x) => (*x).clone(),
                None => "/".to_string(),
            };

            match chs[0] {
                "$" => {
                    if chs[1] == "ls" {
                        continue;
                    }

                    if chs[2] == ".." {
                        stack.pop();
                    } else {
                        stack.push(chs[2].to_string());
                    }
                },

                "dir" => {
                    let out: &mut Vec<String> = self.edges.entry(cur_dir.clone()).or_insert(Vec::<String>::new());
                    (*out).push(chs[1].to_string());
                },

                _ => {
                    let s: &mut i32 = self.score.entry(cur_dir.clone()).or_insert(0);
                    *s += chs[0].parse::<i32>().unwrap();
                },
            }
        }
    }

    fn fill(&mut self, node: &String) {
        for nxt in &std::mem::take(&mut self.edges[node]) {
            self.fill(nxt);
            let s: &i32 = self.score.entry((*node).clone()).or_insert(0);
            *s += self.score[nxt];
        }
    }
}

fn main() {
    let inp = fs::read_to_string("./7.ex").expect("Error reading file");
    
    let g = Graph::new();
    g.parse(inp);
    g.fill(&"/".to_string());

    let mut res: i32 = 0;
    for (_, size) in g.score {
        if size > 100000 {
            continue;
        }

        res += size;
    }
    println!("{}", res);
}
