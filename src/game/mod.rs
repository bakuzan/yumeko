use super::constants;
use super::deck;
use super::user_input;
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

pub fn get_round_result(player: &Hand, dealer: &Hand) -> (bool, String) {
    let player_total = player.total();
    let dealer_total = dealer.total();

    let result = player_total > dealer_total;
    let message = format!("  You: {}\n  Dealer: {}", player_total, dealer_total);

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
