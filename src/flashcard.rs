use std::fs;

#[derive(Debug, Clone)]
pub struct Flashcard {
    front_text: String,
    back_text: String,
    // image: Option<Image>
}

impl Flashcard {
    pub fn back(&self) -> String {
        self.back_text.clone()
    }

    pub fn front(&self) -> String {
        self.front_text.clone()
    }

    pub(crate) fn from_csv(filename: String) -> Vec<Flashcard> {
        let file = fs::read_to_string(filename)
            .expect("Could not open file");
        let mut rdr = csv::Reader::from_reader(file.as_bytes());
        let mut flashcards: Vec<Flashcard> = vec![];

        for result in rdr.records() {
            let record = result.expect("Failed to read record");
            flashcards.push(Flashcard::from((record.get(0).ok_or("").unwrap().into(), record.get(1).ok_or("").unwrap().into())));
        }

        return flashcards;  
    }
}

impl From<(String, String)> for Flashcard {
    fn from(value: (String, String)) -> Self {
        return Flashcard { front_text: value.0, back_text: value.1 }
    }
}
