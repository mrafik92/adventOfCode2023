use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, PartialOrd, PartialEq)]
enum Rank {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Card {
    _2 = 13,
    _3 = 12,
    _4 = 11,
    _5 = 10,
    _6 = 9,
    _7 = 8,
    _8 = 7,
    _9 = 6,
    _T = 5,
    _J = 4,
    _Q = 3,
    _K = 2,
    _A = 1,
}

#[derive(Debug, PartialOrd, PartialEq)]
enum CardWithJoker {
    _J = 13,
    _2 = 12,
    _3 = 11,
    _4 = 10,
    _5 = 9,
    _6 = 8,
    _7 = 7,
    _8 = 6,
    _9 = 5,
    _T = 4,
    _Q = 3,
    _K = 2,
    _A = 1,
}

fn char_to_card(c: char) -> Card {
    match c {
        '2' => Card::_2,
        '3' => Card::_3,
        '4' => Card::_4,
        '5' => Card::_5,
        '6' => Card::_6,
        '7' => Card::_7,
        '8' => Card::_8,
        '9' => Card::_9,
        'T' => Card::_T,
        'J' => Card::_J,
        'Q' => Card::_Q,
        'K' => Card::_K,
        'A' => Card::_A,
        _ => panic!("cannot convert unknown card"),
    }
}

fn char_to_card_with_jocker(c: char) -> CardWithJoker {
    match c {
        '2' => CardWithJoker::_2,
        '3' => CardWithJoker::_3,
        '4' => CardWithJoker::_4,
        '5' => CardWithJoker::_5,
        '6' => CardWithJoker::_6,
        '7' => CardWithJoker::_7,
        '8' => CardWithJoker::_8,
        '9' => CardWithJoker::_9,
        'T' => CardWithJoker::_T,
        'J' => CardWithJoker::_J,
        'Q' => CardWithJoker::_Q,
        'K' => CardWithJoker::_K,
        'A' => CardWithJoker::_A,
        _ => panic!("cannot convert unknown card"),
    }
}

#[derive(Debug)]
struct Hand {
    cards: String,
    rank: Rank,
    bid: u64,
    joker_used: bool,
}

impl Hand {
    fn new(cards: &str, is_joker: bool, bid: u64) -> Hand {
        Hand {
            cards: cards.to_owned(),
            bid,
            rank: match is_joker {
                false => Hand::calculate_rank(cards),
                true => Hand::calculate_rank_with_joker(cards),
            },
            joker_used: is_joker,
        }
    }

    fn calculate_rank(cards: &str) -> Rank {
        let used = cards.chars().counts_by(|x| x);

        match used.len() {
            1 => Rank::FiveOfKind,
            2 => {
                if used.values().contains(&4) {
                    Rank::FourOfKind
                } else {
                    Rank::FullHouse
                }
            }
            3 => {
                if used.values().contains(&3) {
                    Rank::ThreeOfKind
                } else {
                    Rank::TwoPair
                }
            }
            4 => Rank::OnePair,
            5 => Rank::HighCard,
            _ => panic!("un-expected value"),
        }
    }

    fn calculate_rank_with_joker(cards: &str) -> Rank {
        let mut used = cards.chars().counts_by(|x| x);
        if !used.keys().contains(&'J') {
            Hand::calculate_rank(cards)
        } else {
            let joker_counts = *used.get(&'J').unwrap();
            used.remove(&'J');

            println!("{:?}, joker counts = {}", used, joker_counts);

            match (joker_counts, used.len()) {
                (5, _) => Rank::FiveOfKind,
                (4, _) => Rank::FiveOfKind,
                (3, 1) => Rank::FiveOfKind,
                (3, 2) => Rank::FourOfKind,
                (2, 1) => Rank::FiveOfKind,
                (2, 2) => Rank::FourOfKind,
                (2, 3) => Rank::ThreeOfKind,
                (1, 4) => Rank::OnePair,
                (1, 3) => Rank::ThreeOfKind,
                (1, 2) => {
                    if used.values().contains(&3) {
                        Rank::FourOfKind
                    } else {
                        Rank::FullHouse
                    }
                }
                (1, 1) => Rank::FiveOfKind,
                _ => panic!("cannot happen"),
            }
        }
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank == other.rank {
            let z: Vec<_> = self.cards.chars().zip(other.cards.chars()).collect();

            match self.joker_used {
                true => {
                    for x in z {
                        if char_to_card_with_jocker(x.1)
                            .partial_cmp(&char_to_card_with_jocker(x.0))
                            .unwrap()
                            != Ordering::Equal
                        {
                            return char_to_card_with_jocker(x.1)
                                .partial_cmp(&char_to_card_with_jocker(x.0))
                                .unwrap();
                        }
                    }
                }
                false => {
                    for x in z {
                        if char_to_card(x.1).partial_cmp(&char_to_card(x.0)).unwrap()
                            != Ordering::Equal
                        {
                            return char_to_card(x.1).partial_cmp(&char_to_card(x.0)).unwrap();
                        }
                    }
                }
            };

            Ordering::Equal
        } else {
            self.rank.partial_cmp(&other.rank).unwrap()
        }
    }
}

fn main() {
    // read file
    let input = std::fs::read_to_string("inputs/input07").unwrap();
    let hands = input
        .lines()
        .map(|x| {
            let splits = x.split(' ').collect::<Vec<_>>();
            Hand::new(splits[0], false, splits[1].parse::<u64>().unwrap())
        })
        .sorted()
        .collect::<Vec<Hand>>();

    let hands_with_hands = input
        .lines()
        .map(|x| {
            let splits = x.split(' ').collect::<Vec<_>>();
            Hand::new(splits[0], true, splits[1].parse::<u64>().unwrap())
        })
        .sorted()
        .collect::<Vec<Hand>>();

    let mut c: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        c += (i as u64 + 1) * hand.bid;
    }
    println!("part1: {}", c);

    let mut c_2: u64 = 0;
    for (i, hand) in hands_with_hands.iter().enumerate() {
        c_2 += (i as u64 + 1) * hand.bid;
    }
    println!("part1: {}", c_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_order() {
        assert_eq!(Rank::FiveOfKind, Rank::FiveOfKind);
        assert_eq!(Rank::FiveOfKind, Rank::FiveOfKind);
        assert!(Rank::FourOfKind < Rank::FiveOfKind);
        assert!("KK677" < "KTJJT");
    }

    #[test]
    fn test_rank_with_jocker() {
        assert_eq!(Rank::OnePair, Hand::calculate_rank_with_joker("11234"));

        assert_eq!(Rank::OnePair, Hand::calculate_rank_with_joker("J1234"));
        assert_eq!(Rank::ThreeOfKind, Hand::calculate_rank_with_joker("J2234"));
        assert_eq!(Rank::FourOfKind, Hand::calculate_rank_with_joker("J2224"));
        assert_eq!(Rank::FiveOfKind, Hand::calculate_rank_with_joker("J2222"));

        assert_eq!(Rank::FiveOfKind, Hand::calculate_rank_with_joker("JJ222"));
        assert_eq!(Rank::FiveOfKind, Hand::calculate_rank_with_joker("JJ333"));

        assert_eq!(Rank::FourOfKind, Hand::calculate_rank_with_joker("JJ433"));
        assert_eq!(Rank::FourOfKind, Hand::calculate_rank_with_joker("JJ343"));

        assert_eq!(Rank::ThreeOfKind, Hand::calculate_rank_with_joker("JJ345"));

        assert_eq!(Rank::FourOfKind, Hand::calculate_rank_with_joker("JJJ45"));

        assert_eq!(Rank::FiveOfKind, Hand::calculate_rank_with_joker("JJJ44"));

        assert_eq!(Rank::FiveOfKind, Hand::calculate_rank_with_joker("JJJJ4"));

        assert_eq!(Rank::FiveOfKind, Hand::calculate_rank_with_joker("JJJJJ"));
    }
}
