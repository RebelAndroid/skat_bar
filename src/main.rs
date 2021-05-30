use rand::{Rng, seq::SliceRandom};
use rand::thread_rng;
use text_io::read;

fn main() {
    println!("Skat Bar!");    
    let mut player_hand = Vec::<&str>::new();
    let mut computer_hand = Vec::<&str>::new();
    let mut skat_bar1 = Vec::<&str>::new();
    let mut skat_bar2 = Vec::<&str>::new();

    // shuffle deck and deal cards
    let deck = get_deck();
    for i in 0..9{
        player_hand.push(&deck[i]);
    }
    for i in 9..18{
        computer_hand.push(&deck[i]);
    }
    skat_bar1.push(&deck[18]);
    skat_bar1.push(&deck[19]);
    skat_bar2.push(&deck[20]);
    skat_bar2.push(&deck[21]);
    
    let mut trump_red = false;
    let mut trump_even = false;

    let mut rng = thread_rng();

    // choose trump, computer dealt so player chooses first
    while true{
        println!("choose trump (red/black/odd/even)");
        let trump: String = read!("{}\n");
        if trump == "r"{
            trump_red = true;
            // TODO: make the computer choose trump intelligently
            trump_even = rng.gen_bool(0.5);
            break;
        }
        if trump == "b"{
            trump_red = false;
            trump_even = rng.gen_bool(0.5);
            break;
        }
        if trump == "o"{
            trump_even = false;
            trump_red = rng.gen_bool(0.5);
            break;
        }
        if trump == "e"{
            trump_even = true;
            trump_red = rng.gen_bool(0.5);
            break;
        }        
        println!("invalid input!")
    }

    println!("trump is {} and {}", if trump_red {"red"} else {"black"}, if trump_even {"even"} else {"odd"});

    let player_points = 0;
    let computer_points = 0;

    while true{
        let mut trick = Vec::<&str>::new();
        println!("Your hand is {:?}", player_hand);
        println!("what card would you like to play 0-{}", player_hand.len() - 1);
        let card_chosen: i32 = read!();
        if card_chosen < 0 || card_chosen >= player_hand.len() as i32{
            println!("invalid index!");
            continue;
        }
        trick.push(player_hand.remove(card_chosen as usize));
    }

}

fn is_trump(card: &str, trump_red: bool, trump_even: bool) -> bool{
    if card == "G" || card == "J"{
        return true;
    }
    let suit = card.chars().nth(0).unwrap();
    let number = card.chars().nth(1).unwrap().to_digit(10).unwrap();
    if trump_red && (suit == 'C' || suit == 'S'){
        // card is club or spade and red is trump
        return false;
    }
    if !trump_red && (suit == 'H' || suit == 'D'){
        // card is heart or diamond and black is trump
        return false;
    }

    return true;

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