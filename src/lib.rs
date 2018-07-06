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

fn display_hand(hand: &Vec<Card>) {
    println!("\nYour hand: ");

    let mut total = 0;
    for c in hand {
        total += c.value;
        println!("  {}", c.display());
    }

    println!("Hand value: {}", total);
}

fn handle_user_choice(cards: &Vec<Card>, player_hand: &Vec<Card>) -> (u32, Vec<Card>, Vec<Card>) {
    println!(
        "\nWhat would you like to do?
         1) Stay
         2) Hit"
    );

    loop {
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
                    return (choice, cards.to_vec(), player_hand.to_vec());
                } else if choice == constants::PLAYER_STAY {
                    println!("STAY: {}", choice);

                    return (choice, cards.to_vec(), player_hand.to_vec());
                } else {
                    println!("Invalid Choice: {}", trimmed);
                    // TODO
                    // HANDLE INCORRECT INPUT
                }
            }
            Err(..) => {
                println!("Invalid Choice: {}", trimmed);
                // TODO
                // HANDLE INCORRECT INPUT
            }
        };
    }
}

fn is_valid_hand(hand: &Vec<Card>) -> bool {
    let mut total = 0;
    for c in hand {
        total += c.value;
    }
    return total <= constants::BLACKJACK_MAXIMUM;
}

pub fn play_blackjack() {
    println!("Yumeko - Blackjack");

    let cards = deck::get_shuffled_deck();
    let (cards, player_hand, dealer_hand) = deal_round(&cards);

    println!("player {:?}", player_hand);
    println!("dealer {:?}", dealer_hand);
    println!("{}", cards.len());

    let mut is_playing = true;
    let mut active_deck = cards;
    let mut active_hand = player_hand;

    while is_playing {
        display_hand(&active_hand);

        let processed_input = handle_user_choice(&active_deck, &active_hand);

        active_deck = processed_input.1;
        active_hand = processed_input.2;

        let action = processed_input.0;
        is_playing = action != constants::PLAYER_STAY && is_valid_hand(&active_hand);
    }
}
