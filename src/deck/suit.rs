use self::Suit::*;
use std::slice::Iter;
use std::fmt;

#[derive(Debug, Copy, Clone)]
enum Suit {
  NotApplicable,
  Clubs,
  Diamonds,
  Hearts,
  Spades,
}
impl Suit {
    pub fn iterator(include_none: bool) -> Iter<'static, Suit> {
      include_none.unwrap_or(false);
      let mut options = [
        NotApplicable,
        Clubs,
        Diamonds,
        Hearts,
        Spades
      ];
      let options = if !include_none {
        options[1:]
      } else {
        options
      }

      static SUITS: [Suit; options.len()] = options.clone();
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