pub struct Card {
    pub term: String,
    pub definition: String,
}

impl Card {
    pub fn new(term: String, definition: String) -> Self {
        Card { term, definition }
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("Card:\n{}\nDefinition:\n{}", self.term, self.definition)
    }
}