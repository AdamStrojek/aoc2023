use std::fs;

fn main() {
    solution("example.txt", seed_parser_1);
    solution("input.txt", seed_parser_1);
    solution("example.txt", seed_parser_2);
    solution("input.txt", seed_parser_2);
}

fn solution<F>(filename: &str, seeds_parser: F)
where
    F: Fn(&str) -> Vec<u64>,
{
    println!("Solving for file {}", filename);

    let mut locations = Vec::new();

    let file_content = fs::read_to_string(filename).expect("Could not read file");

    let mut it = file_content.split("\n\n");

    // seeds: 79 14 55 13
    // 012345678901234567
    //        ^ safely can skip here
    let seeds = seeds_parser(it.next().unwrap());
    println!("Seeds calculated {}", seeds.len());

    // Parse mappings
    let mut mappers = Vec::<Mapper>::new();

    while let Some(mapping_line) = it.next() {
        let mapper = Mapper::parse_from_str(mapping_line);
        mappers.push(mapper);
    }

    for seed in seeds {
        let mut value = seed;
        for mapper in mappers.iter() {
            value = mapper.map(value);
        }

        locations.push(value);
    }

    locations.sort();
    println!("Solution 1: {}", locations.first().unwrap());
}

fn seed_parser_1(s: &str) -> Vec<u64> {
    s.strip_prefix("seeds: ").unwrap()
     .split_whitespace().map(|seed| seed.parse::<u64>().unwrap())
     .collect::<Vec<u64>>()
}

fn seed_parser_2(s: &str) -> Vec<u64> {
    let mut result = Vec::new();

    for chunk in s.strip_prefix("seeds: ").unwrap()
        .split_whitespace().map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<u64>>().chunks(2) {
        for i in 0..chunk[1] {
            result.push(chunk[0] + i);
        }
    }

    result
}

#[derive(Debug, PartialEq)]
struct MappingItem {
    source: u64,
    target: u64,
    length: u64,
}

impl MappingItem {
    fn new(source: u64, target: u64, length: u64) -> MappingItem {
        MappingItem { source, target, length }
    }

    fn in_range(&self, source: u64) -> bool {
        self.source <= source && source < self.source + self.length
    }
}

struct Mapper {
    source: String,
    target: String,
    mapping_items: Vec<MappingItem>,
}

impl Mapper {
    fn parse_from_str(s: &str) -> Self {
        let mut items: Vec<MappingItem> = Vec::new();

        let mut it = s.lines();

        // seed-to-soil map:
        //             ^ strip suffix
        let mut title = it.next().unwrap().strip_suffix(" map:").unwrap().split("-to-");
        let source = title.next().unwrap().to_string();
        let target = title.next().unwrap().to_string();

        //  45  77  23
        // dst src len
        while let Some(line) = it.next() {
            let mut parts = line.split_whitespace();

            items.push(MappingItem {
                target: parts.next().unwrap().parse().unwrap(),
                source: parts.next().unwrap().parse().unwrap(),
                length: parts.next().unwrap().parse().unwrap(),
            });
        }

        // Sort everything for branch prediction to work better.
        items.sort_by_key(|mi| mi.source);

        Self { source, target, mapping_items: items }
    }

    fn map(&self, source: u64) -> u64 {
        let mapping_item = self.mapping_items.iter().find(|mi| mi.in_range(source));
        match mapping_item {
            Some(mi) => mi.target + source - mi.source,
            None => source,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = "seed-to-soil map:\n50 98 2\n52 50 48";

        let mapper = Mapper::parse_from_str(s);

        assert_eq!(mapper.mapping_items, vec![MappingItem::new(50, 52, 48), MappingItem::new(98, 50, 2)]);
    }

    #[test]
    fn test_mapping() {
        let s = "seed-to-soil map:\n50 98 2\n52 50 48";

        let mapper = Mapper::parse_from_str(s);

        assert_eq!(mapper.map(10), 10);
        assert_eq!(mapper.map(55), 57);
        assert_eq!(mapper.map(99), 51);
    }
}
