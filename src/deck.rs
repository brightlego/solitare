use std::cmp::Ordering;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Value {
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

const VALUES: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
    Value::Ace,
];

const SUITS: [Suit; 4] = [Suit::Clubs, Suit::Hearts, Suit::Diamonds, Suit::Spades];

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Card {
    suit: Suit,
    value: Value,
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

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Deck<T, const N: usize> {
    cards: [T; N],
    top: usize,
}

impl<T, const N: usize> From<[T; N]> for Deck<T, N> {
    fn from(value: [T; N]) -> Self {
        Deck {
            cards: value,
            top: 0,
        }
    }
}

impl<T, const N: usize> Deck<T, N> {
    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let slice = &mut self.cards[self.top..];
        slice.shuffle(&mut rng);
    }

    pub fn draw_one(&mut self) -> Option<&T> {
        if self.top >= N {
            None
        } else {
            let card = &self.cards[self.top];
            self.top += 1;
            Some(card)
        }
    }

    pub fn draw_many(&mut self, n: usize) -> Vec<&T> {
        let mut vec: Vec<&T> = vec![];
        for _ in 0..n {
            if let Some(card) = self.cards.get(self.top) {
                vec.push(card);
                self.top += 1;
            } else {
                return vec;
            }
        }
        vec
    }

    pub fn get_one(&self) -> Option<&T> {
        self.cards.get(self.top)
    }

    pub fn get_many(&self, n: usize) -> Vec<&T> {
        let mut vec = Vec::with_capacity(n);
        for card in &self.cards[self.top..std::cmp::min(self.top + n, N)] {
            vec.push(card);
        }
        vec
    }

    pub fn reset(&mut self) {
        self.top = 0;
    }

    pub fn get_all(&self) -> Vec<&T> {
        self.get_many(N)
    }
}

pub fn standard() -> Deck<Card, 52> {
    let mut cards: [Card; 52] = [Card {
        suit: Suit::Clubs,
        value: Value::Ace,
    }; 52];
    for i in 0..SUITS.len() {
        for j in 0..VALUES.len() {
            cards[VALUES.len() * i + j] = Card {
                suit: SUITS[i],
                value: VALUES[j],
            };
        }
    }
    Deck::from(cards)
}

mod display;
#[cfg(test)]
mod tests;
