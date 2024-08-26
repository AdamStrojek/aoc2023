use std::fs;
use std::collections::HashMap;
use std::iter::Cycle;
use std::str::Chars;

fn main() {
    let content = fs::read_to_string("day08/input.txt")
        .expect("Could not read file");

    let mut it = content.split("\n\n");

    let mut path = it.next().unwrap().chars().cycle();

    let nodes = Nodes::from_string(it.next().unwrap());

    let result = nodes.traverse_loop(&mut path);
    println!("Result: {}", result);
}

struct Nodes(HashMap<String, (String, String)>);

impl Nodes {
    fn from_string(input: &str) -> Self {
        let mut nodes = HashMap::new();

        for line in input.lines() {
            // AAA = (BBB, CCC)
            let key = line[0..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            nodes.insert(key.clone(), (left, right));
        }

        Nodes(nodes)
    }

    fn traverse(&self, path: &mut Cycle<Chars>, count: usize, current: &String) -> Option<usize> {
        if current == "ZZZ" {
            Some(count)
        } else {
            let (left, right) = self.0.get(current)?;
            match path.next() {
                Some('L') => self.traverse(path, count + 1, left),
                _ => self.traverse(path, count + 1, right),
            }
        }
    }

    fn traverse_loop(&self, path: &mut Cycle<Chars>) -> usize {
        let mut current: &String = &String::from("AAA");
        let mut result = 0;

        while current != "ZZZ" {
            let (left, right) = self.0.get(current).unwrap();
            current = match path.next() {
                Some('L') => left,
                _ => right,
            };
            result += 1;
        }

        result
    }
}
