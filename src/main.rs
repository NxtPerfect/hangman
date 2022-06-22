// use text_io::scan;
// use text_io::read;
use std::io;

// ! Problems: doesn't let me write the option before saying it was wrong
// ! the word has end line before the end of quotes
// ! the word to guess isn't underscored for all the letters in it, instead it shows one letter that isn't in the guessed word

pub fn guessing(hidden_word: &str, guess: char, original_word: &str, tries: usize) -> bool {
    // we need to check which char was guessed and then reveal it
    let mut is_correct = false;
    println!(
        "Your word to guess is \"{}\", with {} tries left",
        guess, tries
    );
    for n in 1..hidden_word.len() {
        if original_word.chars().nth(n).unwrap().eq(&guess) {
            is_correct = true;
        }
    }
    is_correct
}

fn main() {
    println!("Hello, welcome to hangman v1.0, please provide your word to guess");

    let mut original_word = String::new();
    // match is used to check if everything went good
    loop {
        match io::stdin().read_line(&mut original_word) {
            // call iostream, read line to word by reference
            Ok(_) => {
                println!("Is this word correct? \"{}\"", original_word.to_lowercase());
                break;
            }
            Err(e) => println!("Opps! something went wrong: {}", e),
        }
    }

    let mut hidden_word = String::new();

    for i in 1..original_word.len() {
        hidden_word += "_";
        // this needs to replace each char
    }

    let mut input = String::new();
    let mut tries = hidden_word.chars().count();

    while hidden_word.eq(&original_word.to_lowercase()) || tries > 0 {
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_) => break,
                Err(e) => println!(
                    "Opps! Something went wrong, we did a little fucky wucky uwu {}",
                    e
                ),
            }
        }

        if input.to_lowercase().eq(&original_word.to_lowercase()) {
            println!("You guessed the word, good job!");
            break;
        }

        let guess = input.chars().nth(0).unwrap();
        if guessing(&hidden_word, guess, &original_word, tries) {
            println!("That is correct!");
            for i in 1..original_word.len() {
                if original_word.chars().nth(i) == input.chars().nth(0) {
                    hidden_word
                        .chars()
                        .nth(i)
                        .clone_from(&original_word.chars().nth(i));
                    break;
                };
            }
            // ! reveal that letter in hidden_word
            // ! search through guess for the letter we guessed
            // ! if it's there then put that letter in hidden_word

            continue;
        }

        tries -= 1;
        println!("Sadly that's a bad guess isn't in the word");
    }
}
