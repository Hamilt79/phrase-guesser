mod list;

use std::io::Write;

enum LoopControl {
    Exit,
    Restart
}

/// Main entry point
fn main() {
    program_loop();
}

/// Program loop
fn program_loop() {
    let words: &Vec<&str> = list::get_word_list();
    loop {
        println!("Welcome!");
        // Gets the number of words in the user's phrase
        let phrase_count: i32;
        match get_word_count() {
            Ok(count) => {
                phrase_count = count;
            },
            Err(LoopControl::Restart) => {
                continue;
            },
            Err(LoopControl::Exit) => {
                return;
            }
        }
        // Gets the max number of words to display
        let max_num_words: i32;
        match get_max_num_words() {
            Ok(count) => {
                max_num_words = count;
            },
            Err(_) => {
                continue;
            },
        }

        println!("In the following prompt you will be asked to type out your word. If there are missing letters in the word you wish to know, put a space in the place of the missing letter.");

        // Gets the user's words
        let all_user_words = get_user_words(phrase_count);
        // Gets all possible word choices for the user's input and the most words in a vector
        let (all_pos_words, most_words) = get_all_pos_words(&words, max_num_words, &all_user_words);
        println!("{}", all_pos_words[0].len());

        // Prints the possible words
        for word in &all_user_words {
            // Replaces spaces with underscores for better formatting
            print!("{}  ", word.replace(" ", "_"));
        }
        println!();
        // Prints dashes under the words for better formatting
        {
            for word in &all_user_words {
                for _ in word.chars() {
                    print!("-");
                }
            }
            for _ in 0..all_user_words.len() - 1 {
                print!("--");
            }
            println!();
        }
        
        // Prints the possible words
        for i in 0..most_words {
            for e in 0..all_pos_words.len() {
                if i < all_pos_words[e].len() {
                    print!("{}  ", all_pos_words[e][i]);
                } else {
                    for _ in all_user_words[e].chars() {
                        print!(" ")
                    }
                    print!("  ");
                }
            }
            println!();
        }
    }
}

/// Gets all possible words for the user's input
/// Returns a tuple containing a vector of vectors of possible words and the most words in a vector
fn get_all_pos_words(words: &Vec<&str>, max_num_words: i32, user_words: &Vec<String>) -> (Vec<Vec<String>>, usize) {
    let mut all_pos_words: Vec<Vec<String>> = Vec::new();
    let mut most_words: usize = 0;
    for i in 0..user_words.len() {
        let pos_words = get_possible_words(&words, max_num_words, &user_words[i]);
        if pos_words.len() > most_words {
            most_words = pos_words.len();
        }
        all_pos_words.push(pos_words);
    }
    (all_pos_words, most_words)
}

/// Gets the user's words
fn get_user_words(phrase_count: i32) -> Vec<String> {
    let mut all_user_words: Vec<String> = Vec::new();
    for i in 0..phrase_count {
        print!("Enter word #{}: ", i);
        std::io::stdout().flush().unwrap_or_default();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = remove_newline(input);
        all_user_words.push(input);
    }
    all_user_words
}

/// Gets the max number of words to display
fn get_max_num_words() -> Result<i32, LoopControl> {
    match get_number_input("Please enter the max number of possible words to display: ") {
        Ok(count) => {
            if count < 1 {
                println!("Max words must be greater than 0! Restarting!");
                return Err(LoopControl::Restart);
            }
            Ok(count)
        },
        Err(_) => {
            println!("Bad max words count! Restarting!");
            Err(LoopControl::Restart)
        }
    }
}

/// Gets the number of words in the user's phrase
fn get_word_count() -> Result<i32, LoopControl> {
    match get_number_input("How many words are in your phrase? (0 to exit): ") {
        Ok(count) => {
            if count == 0 {
                return Err(LoopControl::Exit);
            } 
            if count < 0 {
                println!("Phrase count must be greater than 0! Restarting!");
                return Err(LoopControl::Restart);
            }
            println!("You have {} words in your phrase!", count);
            Ok(count)
        },
        Err(_) => {
            println!("Bad phrase count! Restarting!");
            Err(LoopControl::Restart)
        }
    }
}

/// Returns a list of possible words based on the user's input
fn get_possible_words(words: &Vec<&str>, max_words: i32, e_user_word: &str) -> Vec<String> {
    let user_word = e_user_word.to_lowercase();
    let mut list: Vec<String> = Vec::new();
    let mut count = 0;
    for e_word in words {
        // Converts the word to lowercase for better pattern matching
        let word = e_word.to_lowercase();
        // If the count is greater than the max words, then return the list
        // since we don't display any more than this
        if count >= max_words {
            return list;
        }
        if word.len() == user_word.len() {
            let mut fail = false;
            let mut word_iter = word.chars();
            let mut user_word_iter = user_word.chars();
            let mut word_next = word_iter.next();
            let mut user_next = user_word_iter.next();
            while !word_next.is_none() && !user_next.is_none() {
                // Ignore spaces
                if user_next.unwrap() != ' ' {
                    // If the characters don't match, then fail
                    if user_next.unwrap() != word_next.unwrap() {
                        fail = true;
                        break;
                    }
                }
                // Get the next characters
                word_next = word_iter.next();
                user_next = user_word_iter.next();
            }
            // If the word is valid, then add it to the list
            if !fail {
                list.push(word.to_string());
                count += 1;
            }
        }
    }
    list
}

/// Gets a number input from the user
fn get_number_input(question: &str) -> Result<i32, std::num::ParseIntError> {
    print!("{}", question);
    std::io::stdout().flush().unwrap_or_default();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = remove_newline(input);
    input.parse::<i32>()
}

/// Removes newline characters from a string
fn remove_newline(input: String) -> String {
    let mut str = String::new();
    for c in input.chars() {
        if c != '\n' && c != '\r' {
            str.push(c);
        }
    }
    str
}
