use std::fmt::{Display, Formatter};
use crate::deck::{Card, Suit, Value};

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::deck::Value::*;
        write!(f, "{}", match self {
            Two   => "2",
            Three => "3",
            Four  => "4",
            Five  => "5",
            Six   => "6",
            Seven => "7",
            Eight => "8",
            Nine  => "9",
            Ten   => "X",
            Jack  => "J",
            King  => "K",
            Queen => "Q",
            Ace   => "A",
        })
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::deck::Suit::*;
        write!(f, "{}", match self {
            Spades => "\u{2660}",
            Hearts => "\u{2665}",
            Diamonds => "\u{2666}",
            Clubs => "\u{2663}",
        })
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{value}{suit}", value=self.value, suit=self.suit)
    }
}