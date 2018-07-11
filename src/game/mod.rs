use super::constants;
use super::deck;
use super::user_input;
use super::utils;
use super::Card;
use super::Hand;

pub fn pair_is_equal(hand: &Hand) -> bool {
    let cards = hand.get();
    let card_one = cards.get(0).unwrap();
    let card_two = cards.get(1).unwrap();

    card_one.name == card_two.name
}

pub fn is_valid_hand(hand: &Hand) -> bool {
    hand.total() <= constants::BLACKJACK_MAXIMUM
}

pub fn player_has_valid_hand(hands: &Vec<Hand>) -> bool {
    let hand_validities: Vec<bool> = utils::iter_map_collect(hands, |h| is_valid_hand(&h));

    hand_validities.contains(&true)
}

pub fn player_has_blackjack(hands: &Vec<Hand>) -> bool {
    hands.len() == 1 && hands.get(0).unwrap().is_blackjack()
}

pub fn get_round_result(player: &Vec<Hand>, dealer: &Hand) -> (bool, String) {
    let dealer_total = dealer.total();
    let player_totals: Vec<u32> = utils::iter_map_collect(player, |h| h.total());

    let result = player_totals.iter().any(|&t| t > dealer_total);
    let player_totals_display =
        utils::iter_map_collect(&player_totals, |&t| t.to_string()).join(", ");

    let message = format!(
        "  You: {}\n  Dealer: {}",
        player_totals_display, dealer_total
    );

    (result, message)
}

fn player_answers_yes(choice: &str) -> bool {
    let options: &str = constants::YES_OPTIONS;
    let lower_choice: &str = &choice.to_lowercase();

    options.contains(lower_choice)
}

pub fn play_again(cards: Vec<Card>, callback: &Fn(Vec<Card>)) {
    println!("Play again?");

    let trimmed = user_input::take();

    if player_answers_yes(&trimmed) {
        let next_round_deck = if cards.len() < 25 {
            deck::get_shuffled_deck()
        } else {
            cards
        };

        callback(next_round_deck);
    } else {
        println!("Bye!");
    }
}
