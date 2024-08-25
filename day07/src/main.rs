use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    solution1("day07/example.txt");
    solution1("day07/input.txt");
}

fn solution1(filename: &str) {
    println!("Solving for file {}", filename);

    let mut hands: Vec<(Hand, u32)> = Vec::new();

    let file_content = fs::read_to_string(filename).expect("Could not read file");
    for line in file_content.lines() {
        let mut line = line.split_whitespace();
        let hand = Hand::new(line.next().unwrap());
        let bid = line.next().unwrap().parse::<u32>().unwrap();
        hands.push((hand, bid));
    }

    hands.sort_unstable_by(|s, o| s.0.partial_cmp(&o.0).unwrap());
    hands.reverse();

    let mut total_bid = 0;

    for (rank, (hand, bid)) in hands.iter().enumerate() {
        total_bid += (rank+1) as u32 * *bid;
        println!("{:?} bid {} rank {}", hand, bid, rank);
    }

    println!("Total winning: {}", total_bid);
}

const CARDS_ORDER: &str = "AKQJT98765432";

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_cards(cards: &str) -> HandType {
        let mut cards_count: HashMap<char, u8> = HashMap::new();

        for card in cards.chars() {
            cards_count.entry(card).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut sorted: Vec<&u8> = cards_count.values().collect();
        sorted.sort();

        match sorted.pop() {
            Some(5) => HandType::FiveOfKind,
            Some(4) => HandType::FourOfKind,
            Some(3) => {
                match sorted.pop() {
                    Some(2) => HandType::FullHouse,
                    _ => HandType::ThreeOfKind,
                }
            },
            Some(2) => {
                match sorted.pop() {
                    Some(2) => HandType::TwoPair,
                    _ => HandType::OnePair,
                }
            },
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug)]
struct Cards(String);

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (s, o) in self.0.chars().zip(other.0.chars()) {
            if s == o {
                continue;
            }

            let s = CARDS_ORDER.find(s).unwrap() as i32;
            let o = CARDS_ORDER.find(o).unwrap() as i32;
            if s < o {
                 return Some(Ordering::Less)
            } else {
                 return Some(Ordering::Greater)
            }
        }

        return Some(Ordering::Equal);
    }
}

impl PartialEq for Cards {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap().is_eq()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: Cards,
}

impl Hand {
    fn new(hand: &str) -> Self {
        let cards = Cards(hand.to_string());
        Hand {
            hand_type: HandType::from_cards(&hand),
            cards,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_of_hand_five() {
        let hand = "AAAAA";
        assert_eq!(HandType::from_cards(hand), HandType::FiveOfKind);
    }
    #[test]
    fn test_type_of_hand_four() {
        let hand = "AAAA4";
        assert_eq!(HandType::from_cards(hand), HandType::FourOfKind);
    }
    #[test]
    fn test_type_of_hand_full_house() {
        let hand = "23332";
        assert_eq!(HandType::from_cards(hand), HandType::FullHouse);
    }
    #[test]
    fn test_type_of_hand_three() {
        let hand = "TTT98";
        assert_eq!(HandType::from_cards(hand), HandType::ThreeOfKind);
    }
    #[test]
    fn test_type_of_hand_two_pairs() {
        let hand = "23432";
        assert_eq!(HandType::from_cards(hand), HandType::TwoPair);
    }
    #[test]
    fn test_type_of_hand_one_pair() {
        let hand = "A23A4";
        assert_eq!(HandType::from_cards(hand), HandType::OnePair);
    }
    #[test]
    fn test_type_of_hand_high_card() {
        let hand = "23456";
        assert_eq!(HandType::from_cards(hand), HandType::HighCard);
    }

    #[test]
    fn test_cards_order() {
        let hand1: Cards = Cards("A2345".to_string());
        let hand2: Cards = Cards("KQJT9".to_string());
        assert!(hand1 < hand2);
    }
    #[test]
    fn test_cards_order_eq() {
        // This are same cards when "scoring"
        let hand1: Cards = Cards("A2345".to_string());
        let hand2: Cards = Cards("A2345".to_string());
        assert_eq!(hand1, hand2);
    }

    #[test]
    fn test_order_of_hand() {
        let hand1 = Hand::new("AAAAA");
        let hand2 = Hand::new("AAAA4");
        assert!(hand1 < hand2);

        let hand1 = Hand::new("KK677");
        let hand2 = Hand::new("KK677");
        assert!(hand1 == hand2);

        let hand1 = Hand::new("T55J5");
        let hand2 = Hand::new("QQQJA");
        assert!(hand1 > hand2);
    }
}
