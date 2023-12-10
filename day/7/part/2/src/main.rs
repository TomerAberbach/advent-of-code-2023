use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, prelude::*, BufReader},
    str::FromStr,
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let mut hands_and_bids: Vec<HandAndBid> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    hands_and_bids.sort_by_key(|hand_and_bid| hand_and_bid.hand);

    let winnings: u32 = hands_and_bids
        .iter()
        .map(|hand_and_bid| hand_and_bid.bid)
        .enumerate()
        .map(|(index, bid)| (index + 1) as u32 * bid)
        .sum();
    println!("{}", winnings);

    Ok(())
}

struct HandAndBid {
    hand: Hand,
    bid: u32,
}

impl FromStr for HandAndBid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = s.split_once(" ").ok_or(())?;
        Ok(HandAndBid {
            hand: hand_str.parse()?,
            bid: bid_str.parse().map_err(|_| ())?,
        })
    }
}

#[derive(Eq, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 5 {
            return Err(());
        }

        let cards: Result<Vec<Card>, ()> = chars.iter().map(|&c| c.try_into()).collect();
        Ok(Hand {
            cards: cards?[..5].try_into().unwrap(),
        })
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut card_counts = self.cards.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });

        if let Some(&joker_count) = card_counts.get(&Card::Joker) {
            card_counts.remove(&Card::Joker);
            match card_counts.iter().max_by_key(|(_, &count)| count) {
                Some((&card, _)) => {
                    card_counts
                        .entry(card)
                        .and_modify(|count| *count += joker_count);
                }
                None => return HandType::FiveOfAKind,
            }
        }

        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_counts.values().any(|&count| count == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_counts.values().any(|&count| count == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_ordering = *&self.get_type().cmp(&other.get_type());
        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        self.cards
            .iter()
            .zip(other.cards.iter())
            .map(|(card1, card2)| card1.cmp(card2))
            .find(|ordering| *ordering != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type()
            && self
                .cards
                .iter()
                .zip(other.cards.iter())
                .all(|(card1, card2)| card1 == card2)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'J' => Ok(Card::Joker),
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(()),
        }
    }
}
