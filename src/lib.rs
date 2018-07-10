extern crate rand;

pub use self::deal::hand::Hand;
pub use self::deck::card::Card;

pub mod constants;
mod deal;
mod deck;
mod game;
mod inform;
mod user_input;

fn handle_user_choice(cards: &Vec<Card>, player_hand: &mut Hand) -> (u32, Vec<Card>) {
    inform::display_user_options(player_hand);

    loop {
        let trimmed = user_input::take();

        match trimmed.parse::<u32>() {
            Ok(choice) => {
                if choice == constants::PLAYER_HIT {
                    println!("HIT: {}", choice);

                    let cards = deal::take_a_card(&cards, player_hand);
                    return (choice, cards.to_vec());
                } else if choice == constants::PLAYER_STAY {
                    println!("STAY: {}", choice);

                    return (choice, cards.to_vec());
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

fn process_dealer_turn(cards: &Vec<Card>, dealer_hand: &mut Hand) -> (Vec<Card>) {
    let mut dealer_not_satified = true;
    let mut current_deck = cards.to_vec();

    while dealer_not_satified {
        inform::display_dealer_hand(&dealer_hand);

        current_deck = deal::take_a_card(&current_deck, dealer_hand);

        dealer_not_satified = dealer_hand.total() < constants::DEALER_STANDS_VALUE;
    }

    current_deck.to_vec()
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

        let updated = handle_user_choice(&active_deck, &mut player_active_hand);
        active_deck = updated.1;

        let action = updated.0;
        let hand_is_valid = game::is_valid_hand(&player_active_hand);
        user_is_active = action != constants::PLAYER_STAY && hand_is_valid;
    }

    let player_hand_is_valid = game::is_valid_hand(&player_active_hand);
    if player_hand_is_valid {
        active_deck = process_dealer_turn(&active_deck, &mut dealer_active_hand);
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
