#[allow(unused_parens)]

use std::io;
use std::cmp;
use rand::Rng;
use std::collections::HashMap;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut input_text = String::new();

    println!("Input some text encrypted using Caesar Cipher and I'll try to decrypt it.");

    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read the text!");

    let mut word_matches: HashMap<usize, usize> = HashMap::new();
    let words_to_req: Vec<String> = get_random_words(input_text.trim(), 5);

    let client = reqwest::Client::new();
    
    for shift in 1..26 {
        let mut words_found: usize = 0;
        let mut current_decrypted_word: String;

        for word in words_to_req.iter() {
            current_decrypted_word = decrypt(&word, shift);

            let resp = client.get(format!("https://en.wiktionary.org/wiki/{}", current_decrypted_word))
                .send()
                .await?
                .text()
                .await?;

            if (!resp.contains("Wiktionary does not yet have an entry")) {      
                words_found += 1;
            }   
        }

        word_matches.insert(words_found, shift);

        if (words_found >= words_to_req.len()) {
            let text_decrypted = decrypt(&input_text, shift);
            display_result(text_decrypted);
            return Ok(());
        }
    }

    let top_shift = word_matches.iter().max();

    match top_shift {
        Some((_, shift)) => {
            let text_decrypted = decrypt(&input_text, *shift);
            display_result(text_decrypted);
        }

        None => {
            println!("Sorry, I couldn't find any matches, make sure your text is correct and try again!");
        }
    }

    Ok(())
}

fn decrypt(text: &str, shift: usize) -> String {
    let shift = (shift - 1) as isize;
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut decrypted_str = String::new();

    for character in text.chars() {
        match alphabet.find(character.to_ascii_lowercase()) {
            Some(val) => {
                let index: isize = val as isize;
                let new_index: isize;

                if (index - shift > 0) {
                    new_index = index - shift;
                } else {
                    new_index = 26 + index - shift;
                }

                decrypted_str.push(
                    alphabet.chars().nth((new_index as usize) - 1).unwrap()
                );
            },

            None => {
                decrypted_str.push(
                    character
                );
            }
        }
    }

    decrypted_str
}

fn get_random_words(text: &str, amount: u8) -> Vec<String> {
    let mut thread = rand::thread_rng();
    let mut rand_words: Vec<String> = Vec::new();
    let words: Vec<String> = get_all_words(&text);

    while (rand_words.len() < cmp::min(words.len(), amount as usize)) {
        let index: usize = thread.gen_range(0..words.len());
        if (rand_words.contains(&words[index])) { continue; }
        
        rand_words.push(
            words[index].clone()
        );
    }

    rand_words
}

fn get_all_words(text: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();

    for word in text.split_whitespace() {
        let word: String = word.to_string();
        let mut word_filtered: String = word.clone();
        word_filtered.retain(|c| !r#"(),".;:'"#.contains(c));
        if (word_filtered != word) { continue; }

        words.push(
            word_filtered.to_lowercase()
        );
    }

    words
}

fn display_result(result_text: String) {
    println!("Found a potential result!");
    println!("{}", result_text);
}