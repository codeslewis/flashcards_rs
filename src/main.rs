mod card;

use text_io::read;

use card::Card;

fn main() {
    let term_prompt = "Enter the term for the card".to_string();
    let definition_prompt = "Now enter the definition for the card".to_string();
    println!("{}", term_prompt);
    let term_entered: String = read!();
    println!("{}", definition_prompt);
    let definition_entered: String = read!();

    let example = Card::new(
        term_entered.to_string(),
        definition_entered.to_string());

    println!("{}", example.to_string());
}
