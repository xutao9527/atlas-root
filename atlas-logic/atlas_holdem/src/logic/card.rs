use std::fmt;
use rs_poker::core::{Card, Suit, Value};
use crate::model::card::{AtlasCard, AtlasSuit, AtlasValue};

impl fmt::Display for AtlasSuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            // AtlasSuit::Heart   => "♥️",
            // AtlasSuit::Diamond => "♦️",
            // AtlasSuit::Club    => "♣️",
            // AtlasSuit::Spade   => "♠️",
            AtlasSuit::Heart   => "♥",
            AtlasSuit::Diamond => "♦",
            AtlasSuit::Club    => "♣",
            AtlasSuit::Spade   => "♠",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for AtlasValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AtlasValue::Two   => "2",
            AtlasValue::Three => "3",
            AtlasValue::Four  => "4",
            AtlasValue::Five  => "5",
            AtlasValue::Six   => "6",
            AtlasValue::Seven => "7",
            AtlasValue::Eight => "8",
            AtlasValue::Nine  => "9",
            AtlasValue::Ten   => "T",
            AtlasValue::Jack  => "J",
            AtlasValue::Queen => "Q",
            AtlasValue::King  => "K",
            AtlasValue::Ace   => "A",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for AtlasCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 输出例子： "K♣" 或 "10♦"
        write!(f, "{}{}", self.suit, self.value)
    }
}

impl From<&AtlasCard> for Card{
    fn from(c: &AtlasCard) -> Self {
        let suit = match c.suit {
            AtlasSuit::Spade => Suit::Spade,
            AtlasSuit::Club => Suit::Club,
            AtlasSuit::Heart => Suit::Heart,
            AtlasSuit::Diamond => Suit::Diamond,
        };
        let value = match c.value {
            AtlasValue::Two => Value::Two,
            AtlasValue::Three => Value::Three,
            AtlasValue::Four => Value::Four,
            AtlasValue::Five => Value::Five,
            AtlasValue::Six => Value::Six,
            AtlasValue::Seven => Value::Seven,
            AtlasValue::Eight => Value::Eight,
            AtlasValue::Nine => Value::Nine,
            AtlasValue::Ten => Value::Ten,
            AtlasValue::Jack => Value::Jack,
            AtlasValue::Queen => Value::Queen,
            AtlasValue::King => Value::King,
            AtlasValue::Ace => Value::Ace,
        };
        Card {
            suit,
            value,
        }
    }
}

