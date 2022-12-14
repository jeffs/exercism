use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CardRank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for CardRank {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "J" => Ok(CardRank::Jack),
            "Q" => Ok(CardRank::Queen),
            "K" => Ok(CardRank::King),
            "A" => Ok(CardRank::Ace),
            _ => Ok(CardRank::Number(s.parse().map_err(|_| "bad rank")?)),
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl FromStr for Suit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Suit::Club),
            "D" => Ok(Suit::Diamond),
            "H" => Ok(Suit::Heart),
            "S" => Ok(Suit::Spade),
            _ => Err("bad suit"),
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Card {
    rank: CardRank,
    suit: Suit,
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            0 => Err("card is empty; needs rank and suit"),
            1 => Err("card is only one char, but should be two or three"),
            2 => Ok(Card {
                rank: s[..1].parse()?,
                suit: s[1..].parse()?,
            }),
            3 => Ok(Card {
                rank: s[..2].parse()?,
                suit: s[2..].parse()?,
            }),
            _ => Err("card should be no more than three chars"),
        }
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: HashSet<Card>,
}

impl Hand {
    /// Returns the rank of this hand, as defined on Wikipedia:
    /// https://en.wikipedia.org/wiki/List_of_poker_hands
    fn rank(&self) -> u8 {
        todo!()
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Result<HashSet<Card>, &'static str> =
            s.split_ascii_whitespace().map(|s| s.parse()).collect();
        Ok(Hand { cards: cards? })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, _rhs: &Self) -> Option<Ordering> {
        todo!()
    }
}

fn count_winners<T: PartialOrd>(items: &[T]) -> usize {
    for i in 1..items.len() {
        if let Some(Ordering::Greater) = items[i - 1].partial_cmp(&items[i]) {
            return i;
        }
    }
    items.len()
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut pairs: Vec<(usize, Hand)> = hands
        .iter()
        .map(|s| s.parse().expect("bad hand"))
        .enumerate()
        .collect();
    pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
    pairs
        .iter()
        .map(|&(index, _)| hands[index])
        .take(count_winners(&pairs))
        .collect()
}

fn main() {}
