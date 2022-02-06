/*
Wordlers

Words from Popular.txt at https://github.com/dolph/dictionary

To-do:
- Highlight the exact/present letters in letters_available.
*/

use std::{io, env};

use console::Term;
use console::style;

mod game_state; // name of file
use game_state::GameState; // name of file :: struct name

mod words;
use words::Words;

// Console color. See this page for color listing:
// https://www.ditig.com/256-colors-cheat-sheet
const ORANGE: u8 = 214;


fn main() {
    // Read command line argument for word length. args[0] is path to executable.
    let args: Vec<String> = env::args().collect();

    let mut word_length = 5;
    if args.len() > 1 {

        if args[1] == "help".to_string() {
            let help_str: &str = "
            Command line arguments:
            - Word length: default is 5.
            - Guesses must be real words: default is true.
            For example, to play with 6-letter words and allow fake words as guesses:
            % cargo run wordlers 6 false
            ";
            println!("{}", &help_str);
            std::process::exit(0);
        }
        else {
            let length = args[1].parse().unwrap();
            if length > 0 {
                word_length = length;
            }
        }
    }

    let mut guesses_must_be_words = true;
    if args.len() > 2 {
        if args[2] == "false" {
            guesses_must_be_words = false;
        }
    }
    
    // If word_file_name is not == "" then that file will be used,
    // otherwise the built-in word list will be used.
    let word_file_name = ""; // or "words.txt";
    let guesses_max = 6;

    let game_state = GameState::new(
        word_length,
        guesses_max,
        guesses_must_be_words
    );
    Game::new(game_state, word_file_name).go();
}


struct Game {
    state: GameState,
    word_list: Vec<String>,
}

impl Game {
    pub fn new(game_state: GameState, word_file_name: &str) -> Self {
        let words = Words{};
        let filtered_words = words.get_words(word_file_name, game_state.word_length);
        Self {
            state: game_state,
            word_list: filtered_words,
        }
    }

    fn go(&mut self) {
        Term::stdout().clear_screen().unwrap();
        self.print_app_name();
        println!("Word length: {}", self.state.word_length);

        self.state.solution = self.create_solution_word();
        self.enter_guess_loop()
    }

    fn print_app_name(&self) {
        println!("{}{}",
        style("=== Wordle").green(),
        style("rs ===").color256(ORANGE));
    }

    fn enter_guess_loop(&mut self) {
        while self.state.guesses_remaining() > 0 {
            // Get user's guess.
            let guess = self.get_guess();
            if guess.len() != self.state.word_length {
                println!("Guess must be {} letters.", self.state.word_length);
                continue;
            }

            if self.state.guesses_must_be_words {
                if !self.word_list.contains(&guess.to_lowercase()) {
                    println!("Guess must be a valid word.");
                    continue;
                }
            }

            // Determine the result.
            let (exact, present) = self.state.match_guess_to_solution(&guess);
            let result = self.state.create_result(&guess, &exact, &present);
            self.state.remove_unfound_letters(&guess);
            self.state.add_guess(guess);
            self.state.add_result(result);

            // Clear the term and print the results.
            Term::stdout().clear_screen().unwrap();
            self.print_app_name();
            println!("{}orrect letter and position\n{}etter present but wrong position\n",
            style("C").green().underlined(),
            style("L").color256(ORANGE));

            for r in &self.state.results {
                println!("{}", r);
            }

            // Check for win.
            if exact.len() == self.state.word_length {
                println!("*** Solved. Well done! ***\n");
                break
            }

            if self.state.guesses_remaining() == 0 {
                println!("\nSorry, the word was: {}.\nGame over.\n", self.state.solution);
            }
        }
    }

    fn create_solution_word(&mut self) -> String {
        let random_index = fastrand::usize(0..self.word_list.len());
        let sol = &self.word_list[random_index];
        sol.to_uppercase()
    }

    fn get_guess(&self) -> String {
        println!("Letters available: {}", self.state.letters_available);
        println!("Guess {} of {}:", self.state.guess_count() + 1, self.state.guesses_max);
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        guess.trim().to_string().to_uppercase()
    }
}


