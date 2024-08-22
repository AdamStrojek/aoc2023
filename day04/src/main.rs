use std::fs;
use std::collections::HashSet;


fn main() {
    println!("Hello, world!");
}

struct ScratchCard {
    no: i32,
    winning: HashSet<i8>,
    yours: HashSet<i8>,
}

impl ScratchCard {
    fn parse_line(line: &str) -> Self {
        // "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        //  012345678901234567890123456789012345678901234567890
        //       ^ Safely can skip first 5 characters
        let mut it = line[5..].chars().peekable();

        let mut part: u8 = 0;

        let mut no: i32 = 0;
        let mut winning: HashSet<i8> = HashSet::new();
        let mut yours: HashSet<i8> = HashSet::new();

        let mut buf = String::new();

        while let Some(ch) = it.next() {
            if ch.is_ascii_digit() {
                buf.push(ch)
            }

            let peek_ch = it.peek();
            if peek_ch.is_some_and(|peek_ch| !peek_ch.is_ascii_digit()) || peek_ch.is_none() {
                if !buf.is_empty() {
                    match part {
                        0 => {no = buf.parse().unwrap();},
                        1 => {winning.insert(buf.parse().unwrap());},
                        2 => {yours.insert(buf.parse().unwrap());},
                        _ => panic!("Not allowed part {}", part),
                    }
                    buf.clear();
                }

                part =  match peek_ch {
                    Some(':') => 1,
                    Some('|') => 2,
                    Some(' ')|None => continue,
                    _ => panic!("Not possible"),
                }
            }
        }

        Self {
            no,
            winning,
            yours,
        }
    }

    fn matching(&self) -> u32 {
        self.winning.intersection(&self.yours).count() as u32
    }

    fn score(&self) -> u8 {
        let matching = self.matching();

        if matching > 0 {
            return 2u32.pow(matching - 1) as u8
        }

        0u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_1() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let card = ScratchCard::parse_line(line);

        assert_eq!(card.no, 1);
        assert_eq!(card.winning, HashSet::from([41, 48, 83, 86, 17]));
        assert_eq!(card.yours, HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]));

        assert_eq!(card.matching(), 4);

        assert_eq!(card.score(), 8);
    }

    #[test]
    fn test_parse_line_2() {
        let line = "Card 201: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";

        let card = ScratchCard::parse_line(line);

        assert_eq!(card.no, 201);
        assert_eq!(card.winning, HashSet::from([13, 32, 20, 16, 61]));
        assert_eq!(card.yours, HashSet::from([61, 30, 68, 82, 17, 32, 24, 19]));

        assert_eq!(card.matching(), 2);

        assert_eq!(card.score(), 2);
    }
}