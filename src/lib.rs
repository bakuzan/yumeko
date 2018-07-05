extern crate rand;

use std::io;

pub use self::deck::card::Card;

pub mod constants;
mod deck;

fn take_a_card(cards: &Vec<Card>, hand: &Vec<Card>) -> (Vec<Card>, Vec<Card>) {
    let mut cards = cards.to_vec();
    let mut new_hand = vec![cards.remove(0)];
    new_hand.extend(hand.to_vec());

    (cards.to_vec(), new_hand)
}

fn deal_round(cards: &Vec<Card>) -> (Vec<Card>, Vec<Card>, Vec<Card>) {
    let mut cards = cards.to_vec();

    let player = vec![cards.remove(0), cards.remove(2)];
    let dealer = vec![cards.remove(1), cards.remove(3)];

    (cards.to_vec(), player, dealer)
}

pub fn play_blackjack() {
    println!("Yumeko - Blackjack");

    let cards = deck::get_shuffled_deck();
    let (cards, player_hand, dealer_hand) = deal_round(&cards);

    println!("player {:?}", player_hand);
    println!("dealer {:?}", dealer_hand);
    println!("{}", cards.len());

    println!(
        "What would you like to do?
         1) Stay
         2) Hit"
    );

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(choice) => {
            if choice == constants::PLAYER_HIT {
                println!("HIT: {}", choice);

                let (cards, player_hand) = take_a_card(&cards, &player_hand);

                println!("current hand: {:?}", player_hand);
                println!("{}", cards.len());
            } else if choice == constants::PLAYER_STAY {
                println!("STAY: {}", choice);
            } else {
                // TODO
                // HANDLE INCORRECT INPUT
            }
        }
        Err(..) => {
            println!("this was not an integer: {}", trimmed);
            // TODO
            // HANDLE INCORRECT INPUT
        }
    };
}
