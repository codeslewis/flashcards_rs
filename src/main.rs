mod card;

use card::Card;

fn main() {
    let example = Card::read_card_from_stdin();

    example.check_answer();
}
