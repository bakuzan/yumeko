use self::hand::Hand;
use super::constants;
use super::Card;
use super::Suit;

pub mod hand;

pub fn take_a_card(cards: &Vec<Card>, hand: &mut Hand) -> (Vec<Card>) {
    let mut cards = cards.to_vec();
    let new_card = cards.remove(0);
    hand.add(new_card);

    cards.to_vec()
}

pub fn deal_round(cards: &Vec<Card>) -> (Vec<Card>, Hand, Hand) {
    let mut cards = cards.to_vec();

    let split_o = Card::new("Five", &Suit::Hearts, &5, &5);
    let split_t = Card::new("Five", &Suit::Spades, &5, &5);

    let player = Hand::new(constants::PLAYER_ID, vec![split_o, split_t]); // cards.remove(0), cards.remove(2)
    let dealer = Hand::new(constants::DEALER_ID, vec![cards.remove(1), cards.remove(3)]);

    (cards.to_vec(), player, dealer)
}
