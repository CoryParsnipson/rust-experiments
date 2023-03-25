use std::io;

const VOWELS: &str = "aeiou";

fn is_vowel(c: char) -> bool {
    if let Some(_) = VOWELS.find(c) {
        true
    } else {
        false
    }
}

fn translate(input: &str) -> String {
    let tokens = input.split_whitespace();
    let mut words: Vec<String> = Vec::new();

    for word in tokens {
        if word.len() == 0 {
            continue;
        }

        let w: String;
        if is_vowel(word.chars().nth(0).unwrap()) {
            w = word.to_owned() + &String::from("-hay");
        } else {
            w = word[1..].to_owned() + &String::from("-") + &word[0..1] + &String::from("ay");
        }
        words.push(w);
    }

    return words.join(" ");
}

fn main() {
    println!("Enter in a string to translate into pig-latin:");

    let mut orig = String::new();
    io::stdin()
        .read_line(&mut orig)
        .expect("failed to read line");

    let translation = translate(&orig);
    println!("Translation:\n========\n{}", translation);
}
