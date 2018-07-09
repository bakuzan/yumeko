use self::Suit::*;
use std::fmt;
use std::slice::Iter;

#[derive(Debug, Copy, Clone)]
pub enum Suit {
  NotApplicable,
  Clubs,
  Diamonds,
  Hearts,
  Spades,
}

impl Suit {
  pub fn iterator() -> Iter<'static, Suit> {
    static SUITS: [Suit; 4] = [Clubs, Diamonds, Hearts, Spades];

    SUITS.into_iter()
  }
}

impl fmt::Display for Suit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
    // or, alternatively:
    // fmt::Debug::fmt(self, f)
  }
}
