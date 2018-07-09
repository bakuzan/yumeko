extern crate rand;

pub use self::deal::hand::Hand;
pub use self::deck::card::Card;

pub mod constants;
mod deal;
mod deck;
mod game;
mod inform;
mod user_input;

fn handle_user_choice(cards: &Vec<Card>, player_hand: &Vec<Card>) -> (u32, Vec<Card>, Vec<Card>) {
    inform::display_user_options(player_hand);

    loop {
        let trimmed = user_input::take();

        match trimmed.parse::<u32>() {
            Ok(choice) => {
                if choice == constants::PLAYER_HIT {
                    println!("HIT: {}", choice);

                    let (cards, player_hand) = deal::take_a_card(&cards, &player_hand);
                    return (choice, cards.to_vec(), player_hand.to_vec());
                } else if choice == constants::PLAYER_STAY {
                    println!("STAY: {}", choice);

                    return (choice, cards.to_vec(), player_hand.to_vec());
                } else if choice == constants::PLAYER_SPLIT {
                    println!("SPLIT: {}", choice);

                //TODO
                // SPLIT LOGIC
                } else {
                    println!("Invalid Choice: {}", trimmed);
                }
            }
            Err(..) => {
                println!("Invalid Choice: {}", trimmed);
            }
        };
    }
}

fn process_dealer_turn(cards: &Vec<Card>, dealer_hand: &Vec<Card>) -> (u32, Vec<Card>, Vec<Card>) {
    let mut dealer_not_satified = true;
    let mut current_deck = cards.to_vec();
    let mut current_hand = dealer_hand.to_vec();

    while dealer_not_satified {
        inform::display_dealer_hand(&current_hand);

        let updated = deal::take_a_card(&current_deck, &dealer_hand);

        current_deck = updated.0;
        current_hand = updated.1;

        dealer_not_satified = game::get_hand_total(&current_hand) < constants::DEALER_STANDS_VALUE;
    }

    return (
        constants::DEALER_DONE,
        current_deck.to_vec(),
        current_hand.to_vec(),
    );
}

fn play_a_hand(cards: Vec<Card>) {
    let (cards, player_hand, dealer_hand) = deal::deal_round(&cards);

    inform::display_dealers_first_card(&dealer_hand);

    let mut user_is_active = true;
    let mut active_deck = cards;
    let mut player_active_hand = player_hand;
    let mut dealer_active_hand = dealer_hand;

    while user_is_active {
        inform::display_player_hand(&player_active_hand);

        let processed_input = handle_user_choice(&active_deck, &player_active_hand);

        active_deck = processed_input.1;
        player_active_hand = processed_input.2;

        let action = processed_input.0;
        let hand_is_valid = game::is_valid_hand(&player_active_hand);
        user_is_active = action != constants::PLAYER_STAY && hand_is_valid;
    }

    let player_hand_is_valid = game::is_valid_hand(&player_active_hand);
    if player_hand_is_valid {
        let processed_input = process_dealer_turn(&active_deck, &dealer_active_hand);

        active_deck = processed_input.1;
        dealer_active_hand = processed_input.2;
    }

    let dealer_hand_is_valid = game::is_valid_hand(&dealer_active_hand);
    let round_result = game::get_round_result(&player_active_hand, &dealer_active_hand);

    inform::display_round_summary(
        &player_active_hand,
        &dealer_active_hand,
        round_result,
        (player_hand_is_valid, dealer_hand_is_valid),
    );

    game::play_again(active_deck, &play_a_hand);
}

pub fn play_blackjack() {
    println!("Yumeko - Blackjack");

    let cards = deck::get_shuffled_deck();
    play_a_hand(cards);
}
