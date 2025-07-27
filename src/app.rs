use serde::{Deserialize, Serialize}; //enables deserialization of JSON data into Rust structs

/// Represents a single multiple-choice question, loaded from JSON.
#[derive(Debug, Deserialize, Clone)]
pub struct Question {
    pub question: String,     //question prompt
    pub options: Vec<String>, //list of possible answer choices
    pub correct: Vec<usize>,  //0-based indices of the correct answer(s)
}
/// App holds the overall application state.
/// It keeps track of the current question, which option is selected,
/// and whether the user has answered yet.
pub struct App {
    pub current_question: Question,
    pub selected: usize, //which option is currently highlighted
    pub answered: bool,
    pub score: u32,
    pub total_attempted: u32,
}

impl Default for App {
    /// Load questions from the JSON file and return an initialized App.
    fn default() -> Self {
        // load the raw string content of the JSON file.
        let data =
            std::fs::read_to_string("src/questions.json").expect("Unable to read questions.json");

        // Parse the raw string into a Vec of Question structs.
        let questions: Vec<Question> =
            serde_json::from_str(&data).expect("Invalid JSON format in questions.json");

        // Create a new App instance starting at question 0
        Self {
            current_question: questions[0].clone(), //show first
            selected: 0,
            answered: false,
            score: 0,
            total_attempted: 0,
        }
    }
}

impl App {
    /// Move the cursor to the next answer option (wraps around at end)
    pub fn next_option(&mut self) {
        self.selected = (self.selected + 1) % self.current_question.options.len();
    }

    /// Move the cursor up one option (wraps to bottom if at top).
    pub fn previous_option(&mut self) {
        if self.selected == 0 {
            self.selected = self.current_question.options.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    /// Mark the question as answered
    /// use this to check correctness, score
    pub fn check_answer(&mut self) {
        self.answered = true;
        //store logic here to store result and update score
    }
}
