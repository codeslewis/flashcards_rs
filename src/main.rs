mod card;

use text_io::read;

use card::Card;

fn main() {
    let how_many_prompt = "Enter number of cards to save:";
    println!("{}", how_many_prompt);
    let how_many: u32 = read!();

    let mut card_list: Vec<Card> = Vec::new();

    for _ in 0..how_many {
        let next_card = Card::read_card_from_stdin();
        card_list.push(next_card);
    }

    println!("{} cards saved", card_list.len());

    for card in card_list.iter() {
        card.check_answer();
    }
}

// First, ask the player to enter the desired number of cards.
// Then, ask them to input the term and the definition for every flashcard.
// For each card, give a term and ask for it's definition