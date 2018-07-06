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
                    // ADD SPLIT
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

fn process_dealer_turn(cards: &Vec<Card>, dealer_hand: &Vec<Card>) -> (u32, Vec<Card>, Vec<Card>) {
    let mut dealer_not_satified = true;
    let mut current_deck = cards.to_vec();
    let mut current_hand = dealer_hand.to_vec();

    while dealer_not_satified {
        println!("\nDealer hand: ");
        display_hand(&current_hand);

        let updated = take_a_card(&cards, &dealer_hand);

        current_deck = updated.0;
        current_hand = updated.1;

        dealer_not_satified = get_hand_total(&current_hand) < constants::DEALER_STANDS_VALUE;
    }

    return (
        constants::DEALER_DONE,
        current_deck.to_vec(),
        current_hand.to_vec(),
    );
}

fn get_hand_total(hand: &Vec<Card>) -> u32 {
    let mut total = 0;
    // TODO handle ace being 1 or 11
    for c in hand {
        total += c.value;
    }

    total
}

fn is_valid_hand(hand: &Vec<Card>) -> bool {
    get_hand_total(&hand) <= constants::BLACKJACK_MAXIMUM
}

pub fn play_blackjack() {
    println!("Yumeko - Blackjack");

    let cards = deck::get_shuffled_deck();
    let (cards, player_hand, dealer_hand) = deal_round(&cards);

    println!("player {:?}", player_hand);
    println!("dealer {:?}", dealer_hand);
    println!("{}", cards.len());

    let mut user_is_active = true;
    let mut active_deck = cards;
    let mut active_hand = player_hand;

    while user_is_active {
        println!("\nYour hand: ");
        display_hand(&active_hand);

        let processed_input = handle_user_choice(&active_deck, &active_hand);

        active_deck = processed_input.1;
        active_hand = processed_input.2;

        let action = processed_input.0;
        let hand_is_valid = is_valid_hand(&active_hand);
        user_is_active = action != constants::PLAYER_STAY && hand_is_valid;

        if !hand_is_valid {
            println!("Bust!\nDealer Wins.");
        }
    }

    let mut dealer_active_hand = dealer_hand;
    let processed_input = process_dealer_turn(&active_deck, &dealer_active_hand);

    active_deck = processed_input.1;
    dealer_active_hand = processed_input.2;

    let dealer_hand_is_valid = is_valid_hand(&dealer_active_hand);

    if !dealer_hand_is_valid {
        println!("Bust!\nPlayer Wins.");
    }

    // TODO
    // Compare scores, pick winner
}
