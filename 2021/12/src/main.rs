use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Cave {
    fn parse(st: &str) -> Cave {
        use Cave::*;
        match st {
            "start" => Start,
            "end" => End,
            a => {
                let ch = a.chars().next().unwrap();
                if ch.is_uppercase() {
                    Big(st.to_string())
                } else {
                    Small(st.to_string())
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Edge {
    source: Cave,
    dest: Cave,
}

fn find_paths(edges: &[Edge]) -> Vec<Vec<Cave>> {
    use Cave::*;

    let mut cave_to_edges: HashMap<Cave, HashSet<Edge>> = HashMap::new();
    let mut visited: HashSet<Cave> = HashSet::new();

    for edge in edges {
        let source_cave_edges = cave_to_edges.entry(edge.source.clone()).or_default();
        source_cave_edges.insert(Edge {
            source: edge.source.clone(),
            dest: edge.dest.clone(),
        });

        let dest_cave_edges = cave_to_edges.entry(edge.dest.clone()).or_default();
        dest_cave_edges.insert(Edge {
            dest: edge.source.clone(),
            source: edge.dest.clone(),
        });
    }

    let mut q: VecDeque<Vec<Cave>> = VecDeque::from([vec![Cave::Start]]);
    let mut paths = vec![];

    while !q.is_empty() {
        let path = q.pop_front().unwrap();
        let cave = path.last().unwrap();

        if *cave == End {
            paths.push(path);
            continue;
        }

        for edge in cave_to_edges.get(cave).unwrap_or(&HashSet::new()) {
            let mut new_path = path.clone();
            new_path.push(edge.dest.clone());

            match edge.dest {
                Big(_) => q.push_back(new_path),
                Small(_) => {
                    if !path.contains(&edge.dest) {
                        q.push_back(new_path)
                    }
                }
                End => q.push_back(new_path),
                Start => (),
            }
        }
    }

    paths
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());

    let edges: Vec<Edge> = lines
        .map(|line| {
            let mut sp = line.split("-");
            println!("{:?}", line);
            let start = sp.next().unwrap();
            let end = sp.next().unwrap();

            Edge {
                source: Cave::parse(start),
                dest: Cave::parse(end),
            }
        })
        .collect();

    //println!("{:?}", edges);
    let p = find_paths(&edges);
    println!("{:?}", p.len());
}
