use std::fs;
use std::collections::HashMap;
use std::iter::Cycle;
use std::str::Chars;
use num_integer::lcm;

fn main() {
    let content = fs::read_to_string("day08/input.txt")
        .expect("Could not read file");

    let mut it = content.split("\n\n");

    let path = it.next().unwrap();

    let nodes = Nodes::from_string(it.next().unwrap());

    // let result = nodes.traverse_loop(&mut path, &String::from("22A"));
    // println!("Result: {}", result);

    let result = nodes.traverse_ghost(path);
    println!("Ghost result: {}", result);

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

    fn traverse_loop(&self, path: &str, start: &String) -> u64 {
        let mut path = path.chars().cycle();
        let mut current: &String = start;
        let mut result = 0;

        while !current.ends_with("Z") {
            let (left, right) = self.0.get(current).unwrap();
            current = match path.next() {
                Some('L') => left,
                _ => right,
            };
            result += 1;
        }

        result
    }

    fn traverse_ghost(&self, path: &str) -> u64 {
        let mut result = 1;
        let starts: Vec<&String> = self.0.keys().filter(|x| x.ends_with("A")).collect();

        for start in starts {
            println!("Looking for {}", start);
            let res = self.traverse_loop(path, start);
            println!("For {} result {}", start, res);
            result = lcm(result, res);
        }

        result
    }
}
