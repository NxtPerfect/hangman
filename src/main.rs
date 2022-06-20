// use text_io::scan;
// use text_io::read;
use std::io;

pub fn guessing(hidden_word: &str, guess: char, tries: usize) -> bool {
    let mut is_correct = false;
    println!(
        "Your word to guess is \"{}\", with {} tries left",
        guess, tries
    );
    for n in 1..hidden_word.len() {
        if hidden_word.chars().nth(n).unwrap().eq(&guess) {
            // char from &str
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
    let mut hidden_word = String::from(&original_word);
    for i in 1..hidden_word.len() {
        hidden_word.replace_range(i - 1..i, "_");
    }
    let mut input = String::new();

    let mut i = 0;
    while hidden_word.eq(&original_word.to_lowercase()) || i < 5 {
        let tries = hidden_word.chars().count();
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_) => break,
                Err(e) => println!(
                    "Opps! Something went wrong, we did a little fucky wucky uwu {}",
                    e
                ),
            }
        }
        let guess = input.chars().nth(0).unwrap();
        if guessing(&hidden_word, guess, tries) {
            println!("That is correct!");
            continue;
        }
        i += 1;
        println!("Sadly this letter isn't in the word");
    }
}
