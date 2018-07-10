use super::constants;
use super::game;
use super::Hand;

pub fn display_separator() {
    let line = "-".repeat(20);
    println!("{}", line);
}

pub fn display_hand(hand: &Hand) {
    let cards = hand.get();
    let total = hand.total();

    for c in cards {
        println!("  {}", c.display());
    }

    println!("Hand value: {}", total);
}

pub fn display_player_hand(hand: &Hand) {
    println!("\nYour hand: ");
    display_hand(hand);
}

pub fn display_blackjack(hand: &Hand) {
    let who = hand.get_owner();
    println!("{} Blackjack!", who);
}

pub fn display_dealer_hand(hand: &Hand) {
    println!("\nDealer hand: ");
    display_hand(hand);
}

pub fn display_dealers_first_card(hand: &Hand) {
    println!("\nOne of the Dealer's cards is: ");
    let values = hand.get();
    let card_one = values.get(0).unwrap();
    println!("  {}", card_one.display());
}

pub fn display_user_options(hand: &Hand) {
    let standard_options: &str = &constants::CHOICE_TEXT;
    let cards = hand.get();

    let additional_option: &str =
        if cards.len() == constants::STARTING_HAND_COUNT && game::pair_is_equal(hand) {
            &constants::CHOICE_TEXT_SPLIT
        } else {
            ""
        };

    println!("{}\n{}", standard_options, additional_option);
}

pub fn display_round_summary(
    player_hand: &Hand,
    dealer_hand: &Hand,
    round_result: (bool, String),
    valid_flags: (bool, bool),
) {
    let (player_won, message) = round_result;
    let (player_hand_is_valid, dealer_hand_is_valid) = valid_flags;
    let player_blackjack = player_hand.is_blackjack();
    let dealer_blackjack = dealer_hand.is_blackjack();

    display_player_hand(player_hand);
    display_dealer_hand(dealer_hand);
    println!("Scores:\n{}\n", message);

    if !player_hand_is_valid {
        println!("You're Bust!\nDealer Wins.");
    } else if !dealer_hand_is_valid {
        println!("Dealer Bust!\nYou Win.");
    } else if dealer_blackjack {
        display_blackjack(dealer_hand);
    } else if player_blackjack {
        display_blackjack(player_hand);
    } else if player_won {
        println!("You won!");
    } else {
        println!("You lost");
    }
}
