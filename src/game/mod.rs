use super::constants;
use super::deck;
use super::user_input;
use super::Card;

pub fn pair_is_equal(hand: &Vec<Card>) -> bool {
    let card_one = hand.get(0).unwrap();
    let card_two = hand.get(1).unwrap();

    card_one.name == card_two.name
}

pub fn get_hand_total(hand: &Vec<Card>) -> u32 {
    let mut total = 0;
    // TODO handle ace being 1 or 11
    for c in hand {
        total += c.value;
    }

    total
}

pub fn is_valid_hand(hand: &Vec<Card>) -> bool {
    get_hand_total(&hand) <= constants::BLACKJACK_MAXIMUM
}

pub fn get_round_result(player: &Vec<Card>, dealer: &Vec<Card>) -> (bool, String) {
    let player_total = get_hand_total(player);
    let dealer_total = get_hand_total(dealer);

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
