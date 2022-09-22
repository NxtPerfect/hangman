// use text_io::scan;
// use text_io::read;
use std::io;

// ! Problems: doesn't let me write the option before saying it was wrong
// ! the word has end line before the end of quotes
// ! the word to guess isn't underscored for all the letters in it, instead it shows one letter that isn't in the guessed word

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed: bool
}

enum GameProgress {
    InProgress,
    Won,
    Lost
}

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);
    
    println!("Hello, welcome to hangman v1.0, please provide your word to guess");
    
    loop {
        println!("You have {} turns left", turns_left);
        display_progress(&letters);

        println!("Enter your letter");
        let user_char = read_user_input_character();

        if user_char == '*' {
            break;
        }

        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        if !at_least_one_revealed {
            turns_left -= 1;
        }

        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("Congrats! You guessed the word.");
                break;
            }
            GameProgress::Lost => {
                println!("You failed!");
                break;
            }
        }
    }

    println!("See you next time!");
}

fn select_word() -> String{
    let mut user_input = String::new();
    println!("Write the word that you want to guess");
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
           return user_input;
        }
        Err(_) => {
           println!("Wrong input");
           return String::from("Error");
       }
    }
}

fn create_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        }
        else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(_) => { return '*'; }
    }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    let mut all_revealed = false;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}
