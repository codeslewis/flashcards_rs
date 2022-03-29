use text_io::read;

pub struct Card {
    term: String,
    definition: String,
}

impl Card {
    fn new(term: String, definition: String) -> Self {
        Card { term, definition }
    }

    pub fn read_card_from_stdin() -> Self {
        let term_prompt = "Enter the term for the card";
        let definition_prompt = "Now enter the definition for the card";
        println!("{}", term_prompt);
        let term_entered = read!();
        println!("{}", definition_prompt);
        let definition_entered: String = read!();

        Card::new(
            term_entered,
            definition_entered)
    }

    pub fn check_answer(&self) {
        let correct = "Your answer is correct!";
        let incorrect = "Your answer is wrong...";
        println!("Enter the answer for {}", self.term);
        let answer: String = read!();
        if answer == self.definition {
            println!("{}", correct);
        } else {
            println!("{}", incorrect);
        }
        
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("Card:\n{}\nDefinition:\n{}", self.term, self.definition)
    }
}
