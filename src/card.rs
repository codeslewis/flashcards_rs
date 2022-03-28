pub struct Card {
    pub term: String,
    pub definition: String,
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("Card:\n{}\nDefinition:\n{}", self.term, self.definition)
    }
}