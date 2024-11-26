use std::collections::HashMap;

fn parse_nodes(input: Vec<&str>) -> HashMap<&str, (&str, &str)> {
    let mut map = HashMap::new();
    for line in input {
        let splt: Vec<&str> = line.split([' ', '=', ',', '(', ')']).filter(|s| *s != "").collect();
        map.insert(splt[0], (splt[1], splt[2]));
    }
    map
}

fn part1(dirs: &str, nodes: HashMap<&str, (&str, &str)>) -> i32 {
    let dirs: Vec<char> = dirs.chars().collect();
    let mut hops = 0;
    let mut dir_idx: usize = 0;
    let mut node = "AAA";
    while node != "ZZZ" {
        let dir = dirs[dir_idx];
        let (left, right) = nodes[node];
        node = match dir {
            'L' => left,
            'R' => right,
            _ => node
        };

        dir_idx = (dir_idx + 1) % dirs.len();
        hops += 1;
    }
    hops
}

fn part2(dirs: &str, node_map: HashMap<&str, (&str, &str)>) -> i32 {
    let dirs: Vec<char> = dirs.chars().collect();
    let mut hops = 0;
    let mut dir_idx: usize = 0;
    let mut nodes: Vec<&str> = node_map.keys().filter(|k: &&&str| (**k).chars().nth(2).unwrap() == 'A').map(|k| *k).collect();
    loop {
        if nodes.iter().all(|n| (*n).as_bytes()[2] as char == 'Z') {
            break;
        }
        let dir = dirs[dir_idx];
        for node in nodes.iter_mut() {
            if node.as_bytes()[2] as char == 'Z' {
                continue
            }
            let (left, right) = node_map[node];
            if dir == 'L' {
                *node = left;
            } else if dir == 'R' {
                *node = right;
            }
        }
        dir_idx = (dir_idx + 1) % dirs.len();
        hops += 1;
    }
    hops
}

fn main() {
    let example_dirs = "LR";
    let example_nodes = vec![
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
    ];

    // println!("{:?}", parse_nodes(example_nodes));
    // println!("{}", part1(example_dirs, parse_nodes(example_nodes)));
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let dirs = lines[0];
    let nodes: Vec<&str> = lines.into_iter().skip(2).collect();
    // println!("{}", part1(dirs, parse_nodes(nodes)));

    println!("{}", part2(example_dirs, parse_nodes(example_nodes)));
    println!("{}", part2(dirs, parse_nodes(nodes)));
}
