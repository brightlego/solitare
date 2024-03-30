use std::cmp::Ordering;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, Debug)]
struct Card {
    suit: Suit,
    value: Value
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.suit != self.suit {
            None
        } else {
            Some(self.value.cmp(&other.value))
        }
    }
}


#[cfg(test)]
mod tests;
mod display;