use self::card::Card;
use self::suit::Suit;
use super::constants::constants;

mod deck {
    pub fn get_shuffled_deck() -> Vec<Card> {
        let cards = deck::build_deck(false);
        // TODO
        // SHUFFLE
        cards
    }

    fn build_deck(include_jokers: bool) -> Vec<Card> {
        let mut cards = Vec::new();

        for suit in Suit::iterator() {
            for (i, pair) in constants.CARD_DATA.iter().enumerate() {
                cards.push(Card::new(&pair.0, &suit, &pair.1, &i));
            }
        }

        if include_jokers {
            for i in 1..3 {
                let (name, value, order) = constants.JOKER_DATA;
                cards.push(Card::new(&name, &Suit::NotApplicable, &value, &order))
            }
        }

        cards
    }
}
