use self::hand::Hand;
use super::constants;
use super::Card;

pub mod hand;

pub fn take_a_card(cards: &Vec<Card>, hand: &mut Hand) -> (Vec<Card>) {
    let mut cards = cards.to_vec();
    let new_card = cards.remove(0);
    hand.add(new_card);

    cards.to_vec()
}

pub fn deal_round(cards: &Vec<Card>) -> (Vec<Card>, Hand, Hand) {
    let mut cards = cards.to_vec();

    let player = Hand::new(constants::PLAYER_ID, vec![cards.remove(0), cards.remove(2)]);
    let dealer = Hand::new(constants::DEALER_ID, vec![cards.remove(1), cards.remove(3)]);

    (cards.to_vec(), player, dealer)
}
