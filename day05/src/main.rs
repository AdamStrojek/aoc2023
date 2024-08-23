fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct MappingItem {
    source: u32,
    target: u32,
    length: u32,
}

impl MappingItem {
    fn new(source: u32, target: u32, length: u32) -> MappingItem {
        MappingItem { source, target, length }
    }

    fn in_range(&self, source: u32) -> bool {
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

    fn map(&self, source: u32) -> u32 {
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
