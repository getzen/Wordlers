// GameState

use console::style;
use crate::ORANGE;

pub struct GameState {
    pub word_length: usize,
    pub guesses_max: usize,
    pub guesses_must_be_words: bool,
    guesses: Vec<String>,
    pub solution: String,
    pub letters_available: String,
    pub results: Vec<String>,
}

impl GameState {
    pub fn new(word_length: usize, guesses_max: usize, guesses_must_be_words: bool) -> Self {
        Self {
            word_length,
            guesses_max,
            guesses_must_be_words,
            guesses: Vec::with_capacity(guesses_max as usize),
            solution: "".to_string(),
            letters_available: String::from("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z "),
            results: vec![],
        }
    }

    pub fn guess_count(&self) -> usize {
        self.guesses.len()
    }

    pub fn guesses_remaining(&self) -> usize {
        self.guesses_max - self.guesses.len()
    }

    pub fn add_guess(&mut self, guess: String) {
        self.guesses.push(guess);
    }

    pub fn add_result(&mut self, result: String) {
        self.results.push(result);
    }

    pub fn match_guess_to_solution(&self, guess: &str) -> (Vec<usize>, Vec<usize>) {
        let mut solution_remaining = self.solution.clone();

        let mut exact_indices: Vec<usize> = vec![];
        let mut exact_chars: Vec<char> = vec![];
        let mut present_indices: Vec<usize> = vec![];

        let mut solution_chars = self.solution.chars();
        let mut guess_chars = guess.chars();

        for i in 0..self.word_length {
            let sol_char = solution_chars.next().unwrap();
            let guess_char = guess_chars.next().unwrap();

            // Test for exact match.
            if sol_char == guess_char {
                exact_indices.push(i);
                exact_chars.push(sol_char);

                // Remove letter from solution remaining so it won't be found
                // when searching for 'present' matches.
                let index_option = solution_remaining.find(sol_char);
                if index_option != None {
                    solution_remaining.remove(index_option.unwrap());
                }
            }
        }

        // Now the 'present' matches.
        guess_chars = guess.chars();
        for i in 0..self.word_length {
            let guess_char = guess_chars.next().unwrap();

            if solution_remaining.contains(guess_char) {
                present_indices.push(i);

                // Remove letter from solution remaining so it won't be found
                // when searching for other matches.
                let index_option = solution_remaining.find(guess_char);
                if index_option != None {
                    solution_remaining.remove(index_option.unwrap());
                }
            }
        }
        (exact_indices, present_indices)
    }

    // Takes the guess and result indices and creates a human-friendly result that can be
    // used as a clue to figure out the solution word. Capital letters are exact matches.
    // Lower-case letters are present, but in the wrong spot.
    // Note: the argument type &[usize] accepts &Vec[usize].
    pub fn create_result(&self, guess: &str, exact_indices: &[usize], present_indices: &[usize]) -> String {
        let mut result = String::new();

        // Pick out exact matches first
        for i in 0..self.word_length {
            let end = i + 1;
            let slice = &guess[i..end];
            // 'exact' and 'present' results are formatted using the 'console' crate styling.
            if exact_indices.contains(&i) {
                let text = format!("{}", style(&slice).green().underlined());
                result.push_str(&text);
            }
            else if present_indices.contains(&i) {
                let text = format!("{}", style(&slice).color256(ORANGE));
                result.push_str(&text);
            }
            else
            {
                result.push_str(slice);
            }
            result.push(' ');
        }
        result
    }

    pub fn remove_unfound_letters(&mut self, guess: &str) {
        let the_chars = guess.chars();
        for c in the_chars {
            if self.solution.contains(c) {
                continue;
            }
            // This letter is not in the solution, so remove it from the letters remaining.
            let index_option = self.letters_available.find(c);
            if index_option != None {
                self.letters_available.remove(index_option.unwrap());
                // Remove the space after the letter. Since a letter was just removed, the
                // same index works.
                self.letters_available.remove(index_option.unwrap());
            }
        }
    }
}

