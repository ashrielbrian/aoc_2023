use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn go_left(&self) -> &String {
        &self.left
    }

    fn go_right(&self) -> &String {
        &self.right
    }
}

fn make_node(left_and_right: &str) -> Node {
    let left_and_right_parsed: Vec<&str> = left_and_right
        .strip_prefix("(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split(",")
        .collect();

    Node {
        left: left_and_right_parsed[0].trim().to_string(),
        right: left_and_right_parsed[1].trim().to_string(),
    }
}

fn parse_maze(maze: String) -> (Vec<Direction>, HashMap<String, Node>) {
    let mut maze_lines = maze.lines();
    let directions = maze_lines.next().unwrap();

    let directions: Vec<Direction> = directions
        .chars()
        .map(|c| match c {
            'R' => Direction::Right,
            _ => Direction::Left,
        })
        .collect();

    let mut nodes = HashMap::new();

    for line in maze_lines.filter(|line| !line.is_empty()) {
        let values: Vec<&str> = line.split(" = ").collect();
        nodes.insert(values[0].trim().to_string(), make_node(values[1].trim()));
    }

    (directions, nodes)
}

fn part_one(directions: &Vec<Direction>, nodes: &HashMap<String, Node>) {
    let mut i: usize = 0;
    let mut num_steps = 0;
    let mut curr_node = &"AAA".to_string();

    while curr_node != "ZZZ" {
        if i >= directions.len() {
            i = 0;
        }

        let node = nodes.get(curr_node).unwrap();
        curr_node = match directions[i] {
            Direction::Right => node.go_right(),
            Direction::Left => node.go_left(),
        };

        i += 1;
        num_steps += 1;
    }

    println!("Part one: {}", num_steps);
}

fn get_steps_to_end(
    start_node: &String,
    directions: &Vec<Direction>,
    nodes: &HashMap<String, Node>,
) -> i32 {
    let mut i: usize = 0;
    let mut num_steps = 0;
    let mut curr_node = start_node;
    while !curr_node.ends_with("Z") {
        if i >= directions.len() {
            i = 0;
        }

        let node = nodes.get(curr_node).unwrap();
        curr_node = match directions[i] {
            Direction::Right => node.go_right(),
            Direction::Left => node.go_left(),
        };

        i += 1;
        num_steps += 1;
    }

    return num_steps;
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcm_of_list(numbers: Vec<i32>) -> u64 {
    if numbers.is_empty() {
        panic!("List is empty. LCM is undefined.")
    }

    let mut result = numbers[0] as u64;

    for &num in &numbers[1..] {
        result = lcm(result as u64, num as u64) as u64;
    }

    result
}
fn part_two(directions: &Vec<Direction>, nodes: &HashMap<String, Node>) {
    let num_steps = nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|node| get_steps_to_end(node, &directions, &nodes))
        .collect::<Vec<i32>>();

    println!("Part two: {:?}", lcm_of_list(num_steps));
}

fn main() {
    let nodes_content =
        fs::read_to_string("maze.txt").expect("Should be able to read maze text file.");

    let (directions, nodes) = parse_maze(nodes_content);

    part_one(&directions, &nodes);
    part_two(&directions, &nodes);
}
