pub const YES_OPTIONS: &'static str = "y|yes|ok";

pub const JOKER: &'static str = "Joker";
pub const ACE: &'static str = "Ace";
pub const TWO: &'static str = "Two";
pub const THREE: &'static str = "Three";
pub const FOUR: &'static str = "Four";
pub const FIVE: &'static str = "Five";
pub const SIX: &'static str = "Six";
pub const SEVEN: &'static str = "Seven";
pub const EIGHT: &'static str = "Eight";
pub const NINE: &'static str = "Nine";
pub const TEN: &'static str = "Ten";
pub const JACK: &'static str = "Jack";
pub const QUEEN: &'static str = "Queen";
pub const KING: &'static str = "King";

pub const JOKER_DATA: (&'static str, u32, usize) = (JOKER, 0, 13);
pub const CARD_DATA: [(&'static str, u32); 13] = [
    (ACE, 1),
    (TWO, 2),
    (THREE, 3),
    (FOUR, 4),
    (FIVE, 5),
    (SIX, 6),
    (SEVEN, 7),
    (EIGHT, 8),
    (NINE, 9),
    (TEN, 10),
    (JACK, 10),
    (QUEEN, 10),
    (KING, 10),
];

pub const DEALER_DONE: u32 = 0;
pub const PLAYER_STAY: u32 = 1;
pub const PLAYER_HIT: u32 = 2;
pub const PLAYER_SPLIT: u32 = 3;

pub const STARTING_HAND_COUNT: usize = 2;
pub const BLACKJACK_MAXIMUM: u32 = 21;
pub const DEALER_STANDS_VALUE: u32 = 17;

pub const CHOICE_TEXT: &'static str = "
\nWhat would you like to do?
    1) Stand
    2) Hit";

pub const CHOICE_TEXT_SPLIT: &'static str = "    3) Split";
