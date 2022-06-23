**Hangman game written fully in Rust**

This is my first project in Rust, I'm still learning so it's probably not ideal or a little wacky on the code, but It helped me learn a lot

Hangman game, at least in this context, is a two player game where one of the players thinks of a single word that the other side has to guess. The other side has to guess the word either letter by letter or they can also just guess the word, but they only got a few attempts to guess. Each time you make a wrong guess, you lose one attempt - you draw lines for hangman until you draw it fully.

The rules are simple
- you can guess one letter at a time
- if the letter is in the word you're guessing, it's revealed at all the occurances in the word, so for example "apple", has two letters 'p' so if you guess that letter, both get revealed, so you'd get something like this "-pp---"
- if you make a bad guess you lose one attempt until there's none left *I couldn't figure out yet how to draw hangman in ascii without suffering*