# Wordlers
A Wordle clone as a console application, written in Rust (Wordle + rs = Worldlers).

- Use standard settings (5-letter words and guesses must be words) or choose your own settings with command-line arguments.
- Uses built-in list of 25,000+ most common English words.
- Shows letters of the alphabet that haven't been ruled out.

Command line arguments:
- Word length: default is 5.
- Guesses must be real words: default is true.

For example, to play with 6-letter words and allow fake words as guesses:

`% cargo run wordlers 6 false`

