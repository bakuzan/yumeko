extern crate rand;

pub use self::deck::card::Card;

pub mod constants;
mod deck;

fn deal_round(cards: &Vec<Card>) -> (Vec<Card>, (Card, Card), (Card, Card)) {
    let mut cards = cards.to_vec();

    let player = (cards.remove(0), cards.remove(2));
    let dealer = (cards.remove(1), cards.remove(3));

    (cards.to_vec(), player, dealer)
}

pub fn play_blackjack() {
    println!("Yumeko - Blackjack");

    let cards = deck::get_shuffled_deck();
    let (cards, player_hand, dealer_hand) = deal_round(&cards);

    println!("player {:?}", player_hand);
    println!("dealer {:?}", dealer_hand);
    println!("{}", cards.len())
}
