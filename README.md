# Wordlers
A Wordle clone as a console application, written in Rust.

- Use standard settings (5-letter words and guesses must be words) or choose your own settings with command-line arguments.
- Uses built-in list of 25,000+ most common English words.

Command line arguments:
- Word length: default is 5.
- Guesses must be real words: default is true.
For example, to play with 6-letter words and allow fake words as guesses:

% cargo run wordlers 6 false

