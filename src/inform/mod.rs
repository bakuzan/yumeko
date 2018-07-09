use super::constants;
use super::game;
use super::Card;

pub fn display_hand(hand: &Vec<Card>) {
    let mut total = 0;
    for c in hand {
        total += c.value;
        println!("  {}", c.display());
    }

    println!("Hand value: {}", total);
}

pub fn display_player_hand(hand: &Vec<Card>) {
    println!("\nYour hand: ");
    display_hand(hand);
}

pub fn display_dealer_hand(hand: &Vec<Card>) {
    println!("\nDealer hand: ");
    display_hand(hand);
}

pub fn display_dealers_first_card(hand: &Vec<Card>) {
    println!("\nOne of the Dealer's cards is: ");
    let card_one = hand.get(0).unwrap();
    println!("  {}", card_one.display());
}

pub fn display_user_options(hand: &Vec<Card>) {
    let standard_options: &str = &constants::CHOICE_TEXT;
    let additional_option: &str =
        if hand.len() == constants::STARTING_HAND_COUNT && game::pair_is_equal(hand) {
            &constants::CHOICE_TEXT_SPLIT
        } else {
            ""
        };

    println!("{}\n{}", standard_options, additional_option);
}

pub fn display_round_summary(
    player_hand: &Vec<Card>,
    dealer_hand: &Vec<Card>,
    round_result: (bool, String),
    valid_flags: (bool, bool),
) {
    let (player_won, message) = round_result;
    let (player_hand_is_valid, dealer_hand_is_valid) = valid_flags;

    display_player_hand(player_hand);
    display_dealer_hand(dealer_hand);
    println!("Scores:\n{}\n", message);

    if !player_hand_is_valid {
        println!("You're Bust!\nDealer Wins.");
    } else if !dealer_hand_is_valid {
        println!("Dealer Bust!\nYou Win.");
    } else if player_won {
        println!("You won!");
    } else {
        println!("You lost");
    }
}
