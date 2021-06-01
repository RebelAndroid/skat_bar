use rand::{Rng, seq::SliceRandom};
use rand::thread_rng;
use text_io::read;
use std::cmp::Ordering;

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
    
    let trump_red;
    let trump_even;

    let mut rng = thread_rng();

    // choose trump, computer dealt so player chooses first
    loop{
        println!("Your hand is {:?}", player_hand);
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

    let mut player_points = 0;
    let mut computer_points = 0;

    // in two player skat bar the player that didn't deal goes first
    // in this computer version the computer is always treated as going first
    let player_starts = true;

    loop{
        if player_hand.is_empty(){
            break;
        }
        // run trick
        let mut trick = Vec::<&str>::new();

        if player_starts{
            println!("Your hand is {:?}", player_hand);
            println!("what card would you like to play 0-{}", player_hand.len() - 1);
            let card_chosen: i32 = read!();
            if card_chosen < 0 || card_chosen >= player_hand.len() as i32{
                println!("invalid index!");
                continue;
            }
            trick.push(player_hand.remove(card_chosen as usize));
            
            if is_trump(trick[0], trump_red, trump_even){
                // if the player plays trump, the computer must follow suit
                for i in 0..(computer_hand.len()-1){
                    if is_trump(computer_hand[0], trump_red, trump_even){
                        trick.push(computer_hand.remove(i));
                        break;
                    }
                }
                // if it can't follow suit it plays it's first card
                // TODO: make the computer smarter
                if trick.len() == 1{
                    trick.push(computer_hand.remove(0));
                }
            }else{
                // if the player doesn't play trump, get what suit the player played
                let suit = trick[0].chars().nth(0).unwrap();
                // and play a card of that suit
                for i in 0..(computer_hand.len()-1){
                    if computer_hand[i].chars().nth(0).unwrap() == suit{
                        trick.push(computer_hand.remove(i));
                        break;
                    }
                }
                // TODO: make the computer smarter
                if trick.len() == 1{
                    trick.push(computer_hand.remove(0));
                }
            }
        }else{
            // computer chooses first
            trick.push(computer_hand.remove(0));
            let suit = trick[0].chars().nth(0).unwrap();
            // whether or not the player can follow suit
            let mut can_follow = false;
            if is_trump(trick[0], trump_red, trump_even){
                for card in &player_hand{
                    if is_trump(card, trump_red, trump_even){
                        can_follow = true;
                    }
                }
            }else{
                for card in &player_hand{
                    if card.chars().nth(0).unwrap() == suit{
                        can_follow = true;
                    }
                }
            }
        }
        // evaluate trick

        // first suit played
        let suit = trick[0].chars().nth(0).unwrap();
        // compare player and computer cards
        let order = compare_cards(trick[0], trick[1], trump_red, trump_even, suit);
        // number of points won in trick
        let trick_points = points(trick[0]) + points(trick[1]);
        println!("Trick: {:?}", trick);
        if order == Ordering::Greater{
            if player_starts{
                // player wins trick
                println!("player wins trick");
                player_points += trick_points;
            }else{
                // computer wins trick
                println!("computer wins trick");
                computer_points += trick_points;
            }
        }else if order == Ordering::Less{
            if player_starts{
                // player loses trick
                println!("computer wins trick");
                computer_points += trick_points;
            }else{
                // player wins trick
                println!("player wins trick");
                player_points += trick_points;
            }
        }else{
            panic!("equal cards!: {}, {}", trick[0], trick[1]);
        }
    }

    println!("Player points: {}, Computer Points: {}", player_points, computer_points);
    

    if player_points > computer_points{
        println!("player wins!");
    }else if player_points < computer_points{
        println!("computer wins!");
    }else{
        println!("tie!");
    }

}
// calculates the number of points a given card is worth
fn points(card: &str) -> i32{
    // the guarantee and joker are worth 4 points
    if card == "G" || card == "J"{
        return 4;
    }
    // the other cards are worth their number
    return card.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;
}

// gives the suit order of the given suit as a number (higher numbers beat lower nubers)
fn suit_order(suit: char) -> i32{
    match suit{
        'C' => 4,
        'S' => 3,
        'H' => 2,
        'D' => 1,
        _ => panic!("invalid suit: {}", suit),
    }
}

fn compare_cards(a: &str, b: &str, trump_red: bool, trump_even: bool, suit: char) -> Ordering{
    if a == "G"{ return Ordering::Greater; }
    if b == "G"{ return Ordering::Less; }
    if a == "J"{ return Ordering::Greater; }
    if b == "J"{ return Ordering::Less; }

    let a_trump = is_trump(a, trump_red, trump_even);
    let b_trump = is_trump(b, trump_red, trump_even);

    let a_suit = a.chars().nth(0).unwrap();
    let b_suit = b.chars().nth(0).unwrap();
    let a_number = a.chars().nth(1).unwrap().to_digit(10).unwrap();
    let b_number = b.chars().nth(1).unwrap().to_digit(10).unwrap();


    if a_trump && !b_trump{
        return Ordering::Greater;
    }
    if !a_trump && b_trump{
        return Ordering::Less;
    }
    if a_trump && b_trump{
        // compare values of cards
        if a_number > b_number{
            return Ordering::Greater;
        }
        if a_number < b_number{
            return Ordering::Less;
        }
        return suit_order(a_suit).cmp(&suit_order(b_suit));
    }
    // a and b are not trump
    if a_suit == suit && b_suit != suit{
        return Ordering::Greater;
    }
    if a_suit != suit && b_suit == suit{
        return Ordering::Less;
    }
    if a_suit == suit && b_suit == suit{
        return a_number.cmp(&b_number);
    }
    if a_suit != suit && b_suit != suit{
        if a_number > b_number{
            return Ordering::Greater;
        }
        if a_number < b_number{
            return Ordering::Less;
        }
        return suit_order(a_suit).cmp(&suit_order(b_suit));
    }
    panic!("??? {}, {}, {}, {}, {}", a, b, trump_red, trump_red, suit);
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

    if (number % 2 == 0) != trump_even{
        // parity is not correct
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