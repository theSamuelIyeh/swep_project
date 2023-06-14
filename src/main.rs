use std::io;
use colored::*;

fn main() {
    println!("\n{} {}\n", "Welcome to the WORD Counter!!".bright_green().bold(), "(Type \"/quit\" to quit)".bright_red().bold());
    loop {
        println!("{}", "**************************************".bright_yellow().bold());
        println!("{}", "Enter a sentence:".bright_magenta().bold());
        let mut sentence = String::new();

        io::stdin().read_line(&mut sentence).expect("Unable to read sentence!");
        
        match sentence.as_str() {
            "/quit\n" => {
                println!("\n{}", "Good Bye!".bright_yellow().bold());
                break;
            },
            _ => count_words(sentence),
        }
    }
}

fn count_words(sentence: String) {
    let words: Vec<&str> = sentence.split(' ').collect();
    let total_words = words.len(); 
    println!("{} {}\n", "Total words in sentence:".green().bold(), total_words.to_string().yellow().bold());
}
