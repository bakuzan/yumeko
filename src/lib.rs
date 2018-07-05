extern crate rand;

pub use self::deck::card::Card;

pub mod constants;
mod deck;

fn deal_round(deck: &Vec<Card>) -> (Vec<Card>, (&Card, &Card), (&Card, &Card)) {
    let hands = &deck[..4];
    let player = (&hands[0], &hands[2]);
    let dealer = (&hands[1], &hands[3]);

    (deck.to_vec(), player, dealer)
}

pub fn play_blackjack() {
    println!("Yumeko - Blackjack");

    let cards = deck::get_shuffled_deck();
    let (cards, player_hand, dealer_hand) = deal_round(&cards);

    println!("player {:?}", player_hand);
    println!("dealer {:?}", dealer_hand);
    println!("{}", cards.len())
}
