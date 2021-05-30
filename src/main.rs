use rand::seq::SliceRandom;
use rand::thread_rng;


fn main() {
    println!("Skat Bar!");
    let deck = get_deck();
    println!("{:?}", deck);
}

fn get_deck() -> [String; 22] {
    let mut rng = thread_rng();
    let mut deck = [String::from("C2"), String::from("C3"), String::from("C4"), String::from("C5"), String::from("C6"),
                    String::from("S2"), String::from("S3"), String::from("S4"), String::from("S5"), String::from("S6"),
                    String::from("H2"), String::from("H3"), String::from("H4"), String::from("H5"), String::from("H6"),
                    String::from("D2"), String::from("D3"), String::from("D4"), String::from("D5"), String::from("D6"),
                    String::from("G"), String::from("J")];

    deck.shuffle(&mut rng);
    deck
}