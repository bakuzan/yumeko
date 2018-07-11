extern crate rand;

pub use self::deal::hand::Hand;
pub use self::deck::card::Card;
pub use self::deck::suit::Suit;

pub mod constants;
mod deal;
mod deck;
mod game;
mod inform;
mod user_input;
mod utils;

fn handle_user_choice(cards: &Vec<Card>, player_hand: &mut Hand) -> (u32, Vec<Card>, Option<Hand>) {
    if player_hand.is_blackjack() {
        inform::display_blackjack(player_hand);
        return (constants::PLAYER_STAY, cards.to_vec(), None);
    }

    inform::display_user_options(player_hand);

    loop {
        let trimmed = user_input::take();

        match trimmed.parse::<u32>() {
            Ok(choice) => {
                if choice == constants::PLAYER_HIT {
                    println!("Player Hits");
                    let cards = deal::take_a_card(&cards, player_hand);

                    return (choice, cards.to_vec(), None);
                } else if choice == constants::PLAYER_STAY {
                    println!("Player Stands");

                    return (choice, cards.to_vec(), None);
                } else if choice == constants::PLAYER_SPLIT {
                    println!("Player Splits");

                    let mut second_hand = player_hand.split();
                    let cards = deal::take_a_card(&cards, player_hand);
                    let cards = deal::take_a_card(&cards, &mut second_hand);

                    return (choice, cards.to_vec(), Some(second_hand));
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

fn process_player_turn(cards: &Vec<Card>, hands: &mut Vec<Hand>) -> (Vec<Card>, Vec<Hand>) {
    let mut current_deck = cards.to_vec();
    let mut has_unprocessed = true;
    let mut processed_hand_count = 0;

    while has_unprocessed {
        let mut user_is_active = true;
        let mut cloned_hands = hands.clone();
        has_unprocessed = false;

        while user_is_active {
            let mut player_active_hand = &mut cloned_hands[processed_hand_count];

            inform::display_player_hand(&player_active_hand);

            let updated = handle_user_choice(&current_deck, &mut player_active_hand);
            let action = updated.0;
            current_deck = updated.1;

            if action == constants::PLAYER_SPLIT {
                let second_hand = updated.2.unwrap();
                hands.push(second_hand);
                has_unprocessed = true;
            }

            let hand_is_valid = player_active_hand.is_valid();
            user_is_active = action != constants::PLAYER_STAY && hand_is_valid;
        }

        let updated_hand = cloned_hands.remove(processed_hand_count);
        hands.remove(processed_hand_count);
        hands.insert(processed_hand_count, updated_hand);
        processed_hand_count += 1;
    }

    (current_deck.to_vec(), hands.to_vec())
}

fn process_dealer_turn(cards: &Vec<Card>, dealer_hand: &mut Hand) -> Vec<Card> {
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
    inform::display_separator();
    let (cards, player_hand, dealer_hand) = deal::deal_round(&cards);

    inform::display_dealers_first_card(&dealer_hand);

    let active_deck = cards;
    let mut player_hands = vec![player_hand];
    let mut dealer_active_hand = dealer_hand;

    let (mut active_deck, player_hands) = process_player_turn(&active_deck, &mut player_hands);

    let player_hand_is_valid = game::player_has_valid_hand(&player_hands);
    let dealer_blackjack = dealer_active_hand.is_blackjack();
    let player_blackjack = game::player_has_blackjack(&player_hands);
    let no_blackjacks = player_blackjack || dealer_blackjack;

    if player_hand_is_valid && no_blackjacks {
        active_deck = process_dealer_turn(&active_deck, &mut dealer_active_hand);
    }

    let dealer_hand_is_valid = dealer_active_hand.is_valid();
    let round_result = game::get_round_result(&player_hands, &dealer_active_hand);

    inform::display_round_summary(
        &player_hands,
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
