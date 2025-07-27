use serde::{Deserialize, Serialize}; //enables deserialization of JSON data into Rust structs
use std::fs;

/// Represents a single multiple-choice question, loaded from JSON.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub question: String,     //question prompt
    pub options: Vec<String>, //list of possible answer choices
    pub correct: Vec<usize>,  //0-based indices of the correct answer(s)
}

/// Struct to save and load progress from disk
#[derive(Debug, Deserialize, Serialize)]
pub struct Progress {
    pub current_index: usize,
    pub score: u32,
    pub total_attempted: u32,
}

//current app state (user session)
pub struct App {
    pub questions: Vec<Question>,
    pub current_index: usize,
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

        //try loading saved progress from file if it exists
        let progress = fs::read_to_string("progress.json")
            .ok()
            .and_then(|contents| serde_json::from_str::<Progress>(&contents).ok());

        let (current_index, score, total_attempted) = match progress {
            Some(p) => (
                p.current_index.min(questions.len().saturating_sub(1)),
                p.score,
                p.total_attempted,
            ),
            None => (0, 0, 0),
        };
        // Create a new App instance starting at question 0
        Self {
            questions,
            current_index,
            selected: 0,
            answered: false,
            score,
            total_attempted,
        }
    }
}

impl App {
    /// get current questions
    pub fn current_question(&self) -> &Question {
        &self.questions[self.current_index]
    }
    /// Move the cursor to the next answer option (wraps around at end)
    pub fn next_option(&mut self) {
        self.selected = (self.selected + 1) % self.current_question().options.len();
    }
    /// Move the cursor up one option (wraps to bottom if at top).
    pub fn previous_option(&mut self) {
        if self.selected == 0 {
            self.selected = self.current_question().options.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn next_question(&mut self) {
        if !self.answered {
            self.total_attempted += 1;
            if self.current_question().correct.contains(&self.selected) {
                self.score += 1;
            }
            self.answered = true;
        }
    }

    /// use this to check correctness, score
    pub fn check_answer(&mut self) {
        if !self.answered {
            self.total_attempted += 1;
            if self.current_question().correct.contains(&self.selected) {
                self.score += 1;
            }
            self.answered = true;
        }
    }

    /// save current progress to disk
    pub fn save_progress(&self) {
        let progress = Progress {
            current_index: self.current_index,
            score: self.score,
            total_attempted: self.total_attempted,
        };
    }

    pub fn reset_progress(&mut self) {
        self.current_index = 0;
        self.score = 0;
        self.total_attempted = 0;
        self.selected = 0;
        self.answered = false;
        let _ = fs::remove_file("progress.json");
    }
}
