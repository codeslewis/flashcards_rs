struct Card {
    term: String,
    definition: String,
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("Card:\n{}\nDefinition:\n{}", self.term, self.definition)
    }
}

fn main() {
    let example = Card {
        term: "purchase".to_string(),
        definition: "buy".to_string(),
    };

    println!("{}", example.to_string());
}
