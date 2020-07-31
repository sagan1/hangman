use rand::{Rng, thread_rng};
use std::io;

fn main() {
    let mut won: bool = false;

    let mut rng = thread_rng();

    let words = ["food", "hello", "gnome", "power", "shit developer", "dumb program"];
    let index= rng.gen_range(0, words.len());
    let mut lives = words[index].len() as u8;

    let mut underscored = String::new();

    for c in words[index].chars() {
        if c.is_whitespace() { underscored.push_str(" "); } else { underscored.push_str("_"); }
    }

    let mut current_status = underscored.clone();
    let mut guessed = String::new();

    while !won {

        println!("\ncurrent: {}    lives: {}    guessed: [{}]\n", current_status, lives, guessed);

        // get user input
        let mut raw = String::new();
        io::stdin().read_line(&mut raw);
        let input: char = raw.chars().next().unwrap();

        if words[index].contains(input) {
            // iterate over the indexes of the chars in the word
            for i in 0..words[index].chars().count() {
                // for each character that is the guessed char, replace it with the guessed char
                if words[index].chars().nth(i).unwrap() == input {
                    current_status.remove(i);
                    current_status.insert(i, input)
                }
            }

        } else {
            lives = lives - 1;
            guessed.push_str(raw.as_ref());
            if lives <= 0 {
                break;
            }
            continue;
        }

        if !current_status.contains("_") {
            won = true;
        }
    }

    if lives <= 0 {
        println!("\n   >>   You lose!   <<   \nThe word was: {}", words[index]);
    } else if won {
        println!("\n   >>   You win!   <<   ");
    }
}