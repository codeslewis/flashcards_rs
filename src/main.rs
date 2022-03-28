mod card;

use card::Card;

fn main() {
    let example = Card {
        term: "purchase".to_string(),
        definition: "buy".to_string(),
    };

    println!("{}", example.to_string());
}
