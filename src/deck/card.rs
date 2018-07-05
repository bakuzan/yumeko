use super::suit::Suit;

#[derive(Debug, Clone)]
pub struct Card {
    suit: Suit,
    name: String,
    value: u32,
    order: usize,
}

impl Card {
    pub fn new(name: &str, suit: &Suit, value: &u32, order: &usize) -> Card {
        Card {
            name: String::from(name),
            suit: suit.clone(),
            value: value.clone(),
            order: order.clone(),
        }
    }
}
