extern crate yumeko;

fn main() {
    let deck = yumeko::get_shuffled_deck();

    for card in deck {
        println!("{:?}", card);
    }
}
