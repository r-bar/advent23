use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

lazy_static! {
    static ref NODE_REGEX: regex::Regex =
        regex::Regex::new(r"^(?<name>\w+) = \((?<left>\w+), (?<right>\w+)\)$").unwrap();
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matches = NODE_REGEX
            .captures(s)
            .ok_or(anyhow::anyhow!("Invalid node line: {s}"))?;
        let name = matches.name("name").unwrap().as_str().to_string();
        let left = matches.name("left").unwrap().as_str().to_string();
        let right = matches.name("right").unwrap().as_str().to_string();
        Ok(Node { name, left, right })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Map {
    nodes: HashMap<String, Node>,
    first_node: String,
    directions: Vec<Direction>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();
        let mut lines = s.lines();
        let direction_line = lines.next().ok_or(anyhow::anyhow!("No direction line."))?;
        let mut directions = Vec::new();
        for c in direction_line.chars() {
            match c {
                'L' => directions.push(Direction::Left),
                'R' => directions.push(Direction::Right),
                _ => continue,
            }
        }
        let _ = lines.next().ok_or(anyhow::anyhow!("No direction speerator line."))?;
        let mut first_node = None;
        for line in lines {
            let node: Node = line.parse()?;
            if first_node.is_none() {
                first_node = Some(node.name.clone());
            }
            nodes.insert(node.name.clone(), node);
        }
        let first_node = first_node.ok_or(anyhow::anyhow!("No first nodes found."))?;
        Ok(Map {
            nodes,
            directions,
            first_node,
        })
    }
}

fn solve(map: &Map) -> anyhow::Result<usize> {
    let mut steps = 0;
    let start_node = String::from("AAA");
    let mut node_ptr = start_node.as_str();
    let mut dir_ptr = map.directions.iter().cycle();

    while node_ptr != "ZZZ" {
        let node = map.nodes.get(node_ptr).ok_or(anyhow::anyhow!("Invalid node."))?;
        let direction = dir_ptr.next().unwrap();
        match direction {
            Direction::Left => node_ptr = &node.left,
            Direction::Right => node_ptr = &node.right,
        }
        steps += 1;
    }

    Ok(steps)
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let map: Map = buffer.parse()?;
    let result = solve(&map)?;
    println!("Result: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const EXAMPLE: &str = include_str!("../../example.txt");
    const EXAMPLE2: &str = include_str!("../../example2.txt");

    #[test]
    fn test_parse_example() -> anyhow::Result<()> {
        let map: Map = EXAMPLE.parse()?;
        assert_eq!(&map.first_node, "AAA");
        assert_eq!(map.directions, vec![Direction::Right, Direction::Left]);
        assert_eq!(map.nodes.len(), 7);
        Ok(())
    }

    #[test]
    fn test_example() {
        let map: Map = EXAMPLE.parse().unwrap();
        assert_eq!(solve(&map).unwrap(), 2);
    }

    #[test]
    fn test_example2() {
        let map: Map = EXAMPLE2.parse().unwrap();
        assert_eq!(solve(&map).unwrap(), 6);
    }

    #[test]
    fn test_regex() {
        let node: Node = "AAA = (BBB, CCC)".parse().unwrap();
        assert_eq!(node.name, "AAA");
        assert_eq!(node.left, "BBB");
        assert_eq!(node.right, "CCC");
    }
}
