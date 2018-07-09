use super::Card;

pub fn take_a_card(cards: &Vec<Card>, hand: &Vec<Card>) -> (Vec<Card>, Vec<Card>) {
    let mut cards = cards.to_vec();
    let mut new_hand = vec![cards.remove(0)];
    new_hand.extend(hand.to_vec());

    (cards.to_vec(), new_hand)
}

pub fn deal_round(cards: &Vec<Card>) -> (Vec<Card>, Vec<Card>, Vec<Card>) {
    let mut cards = cards.to_vec();

    let player = vec![cards.remove(0), cards.remove(2)];
    let dealer = vec![cards.remove(1), cards.remove(3)];

    (cards.to_vec(), player, dealer)
}
