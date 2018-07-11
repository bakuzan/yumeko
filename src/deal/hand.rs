use super::constants;
use super::Card;

#[derive(Debug, Clone)]
pub struct Hand {
    id: u32,
    values: Vec<Card>,
}

impl Hand {
    pub fn new(id: u32, values: Vec<Card>) -> Hand {
        Hand {
            id,
            values: values.to_vec(),
        }
    }

    pub fn get_owner(&self) -> String {
        if self.id == constants::DEALER_ID {
            String::from("Dealer")
        } else {
            String::from("Player")
        }
    }

    pub fn get(&self) -> Vec<Card> {
        self.values.to_vec()
    }

    pub fn add(&mut self, new_card: Card) {
        self.values.push(new_card);
    }

    pub fn split(&mut self) -> Hand {
        let card = self.values.remove(1);
        let hand_two = Hand::new(constants::PLAYER_ID, vec![card]);

        hand_two
    }

    fn hand_contains_at_least(&self, card_name: &str, number: usize) -> bool {
        let card_name_string = card_name.to_string();
        let matches: Vec<Card> = self.get()
            .iter()
            .filter(|c| c.name == card_name_string)
            .cloned()
            .collect();

        matches.len() >= number
    }

    pub fn total(&self) -> u32 {
        let mut total = 0;

        for c in &self.values {
            total += c.value;
        }

        // Only treat an ace as 11 if you won't go bust.
        let ace_as_eleven_total = total + 10;
        let hand_has_ace = self.hand_contains_at_least(&constants::ACE, 1);

        if !hand_has_ace || ace_as_eleven_total > constants::BLACKJACK_MAXIMUM {
            total
        } else {
            ace_as_eleven_total
        }
    }

    pub fn is_blackjack(&self) -> bool {
        self.values.len() == 2
            && self.hand_contains_at_least(&constants::ACE, 1)
            && self.total() == constants::BLACKJACK_MAXIMUM
    }

    pub fn can_split(&self) -> bool {
        let cards = self.get();
        let card_one = cards.get(0).unwrap();
        let card_two = cards.get(1).unwrap();

        cards.len() == 2 && card_one.name == card_two.name
    }

    pub fn is_valid(&self) -> bool {
        self.total() <= constants::BLACKJACK_MAXIMUM
    }
}
