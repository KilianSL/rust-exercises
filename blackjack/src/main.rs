use std::fmt::Display;

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

impl Value {
    fn get_numeric_value(&self) -> u32 {
        match self {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ace => 11,
            _ => 10,
        }
    }
    fn get_string_value(&self) -> &str {
        match self {
            Value::Two => "Two",
            Value::Three => "Three",
            Value::Four => "Four",
            Value::Five => "Five",
            Value::Six => "Six",
            Value::Seven => "Seven",
            Value::Eight => "Eight",
            Value::Nine => "Nine",
            Value::Ten => "Ten",
            Value::Jack => "Jack",
            Value::Queen => "Queen",
            Value::King => "King",
            Value::Ace => "Ace",
        }
    }
}

enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    fn get_string_value(&self) -> &str {
        match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        }
    }
}

struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    fn new(value: Value, suit: Suit) -> Card {
        Card { suit, value }
    }
    fn get_string_value(&self) -> String {
        format!(
            "{} of {}",
            self.value.get_string_value(),
            self.suit.get_string_value()
        )
    }
}

impl Display for Card {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("{}", self.get_string_value());
        Ok(())
    }
}

struct Deck {
    cards: [Card; 52],
}

impl Deck {
    fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in Suit {}
    }
}

fn main() {
    let card = Card::new(Value::Ace, Suit::Spades);
    println!("Your card is the {}", card);
}
