use colored::*;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use url::Url;

const EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp"];
const PROMPT_LIMIT: usize = 254;
const TITLE_LIMIT: usize = 16;

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    title: Option<String>,
    text: String,
    background: Option<String>,
    turns: Option<u32>,
}

impl Card {
    fn new() -> Card {
        Card {
            title: take_title(),
            text: take_text(),
            background: take_background(),
            turns: take_turns(),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = format!(
            "{} {}",
            "Title:".bold().green(),
            self.title
                .as_ref()
                .unwrap_or(&"".to_string())
                .cyan()
                .italic()
        );
        let text = format!("{} {}", "Prompt:".bold().green(), self.text.cyan().italic());
        let background = format!(
            "{} {}",
            "background:".bold().green(),
            self.background
                .as_ref()
                .unwrap_or(&"".to_string())
                .cyan()
                .italic()
        );
        let turns = format!(
            "{} {}",
            "Turns:".bold().green(),
            self.turns
                .as_ref()
                .unwrap_or(&0)
                .to_string()
                .cyan()
                .italic()
        );
        write!(f, "{}\n{}\n{}\n{}", title, text, background, turns)
    }
}

fn print_help() {
    let previous = format!(
        "{} {}",
        "\"%PREVIOUS_PLAYER%\"".green(),
        "Displays the previous player in the list".cyan().italic()
    );
    let next = format!(
        "{} {}",
        "\"%NEXT_PLAYER%\"".green(),
        "Displays the next player in the list".cyan().italic()
    );
    let random = format!(
        "{} {}",
        "\"%PLAYER%\"".green(),
        "Displays a random player.".cyan().italic()
    );
    let current = format!(
        "{} {}",
        "\"%SELF%\"".green(),
        "Displays the user whose turn it is currently"
            .cyan()
            .italic()
    );
    let turns = format!(
        "{} {}",
        "\"%TURNS%\"".green(),
        "Displays how many turns the card lasts".cyan().italic()
    );

    println!("{}\n{}\n{}\n{}\n{}", previous, next, random, current, turns);
}

fn take_title() -> Option<String> {
    loop {
        println!(
            "{} {}",
            "Enter the card title:".cyan().bold(),
            "(optional)".cyan().italic()
        );

        match take_input() {
            None => return None,
            Some(output) => {
                let char_count = output.chars().count();

                if char_count > TITLE_LIMIT {
                    println!(
                        "{} {} {}",
                        "Your input was".red().bold(),
                        (char_count - TITLE_LIMIT).to_string().red().underline(),
                        "characters too long".red().bold()
                    );
                    continue;
                }

                //print!("\x1B[2J");
                return Some(output);
            }
        }
    }
}

fn take_text() -> String {
    loop {
        print_help();
        println!("{}", "Enter the card prompt:".cyan().bold());

        match take_input() {
            None => {
                println!("{}", "The prompt cant be empty.".red());
                continue;
            }
            Some(output) => {
                let char_count = output.chars().count();

                if char_count > PROMPT_LIMIT {
                    println!(
                        "{} {} {}",
                        "Your input was".red().bold(),
                        (char_count - PROMPT_LIMIT).to_string().red().underline(),
                        "characters too long".red().bold()
                    );
                    continue;
                }

                //print!("\x1B[2J");
                return output;
            }
        }
    }
}

fn take_background() -> Option<String> {
    loop {
        println!(
            "{} {}",
            "Enter the cards's background url:".cyan().bold(),
            "(optional)".cyan().italic()
        );

        match take_input() {
            None => return None,
            Some(output) => {
                let url = Url::parse(&output);
                if url.is_err() {
                    println!("{}", "not a valid url".red().bold());
                    continue;
                }

                if !EXTENSIONS
                    .iter()
                    .any(|extension| output.ends_with(extension))
                {
                    println!("{}", "not a valid image url".red().bold());
                    continue;
                }

                //print!("\x1B[2J");
                return Some(output);
            }
        }
    }
}

fn take_turns() -> Option<u32> {
    loop {
        println!(
            "{} {}",
            "Enter how many turns the card must last:".cyan().bold(),
            "(optional)".cyan().italic()
        );

        match take_input() {
            None => return None,
            Some(x) => {
                let output: u32 = match x.parse::<u32>() {
                    Err(_) => {
                        println!("{}", "Couldn't parse this as a number".red());
                        continue;
                    }
                    Ok(x) => x,
                };

                //print!("\x1B[2J");
                return Some(output);
            }
        }
    }
}

fn take_input() -> Option<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut buffer)
        .expect("Couldn't read from terminal");
    buffer = buffer.trim().to_string();
    if buffer.is_empty() {
        return None;
    }
    Some(buffer)
}

fn open_file() -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("cards.json")
        .expect("Couldn't open or create file.")
}

fn write_file(cards: &Vec<Card>) -> Result<&Vec<Card>, io::Error> {
    let mut file = open_file();
    let content = serde_json::to_string(&cards)?;
    file.write_fmt(format_args!("{}", content))?;
    file.flush()?;
    Ok(cards)
}

fn open_cards() -> Option<Vec<Card>> {
    let mut file = open_file();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read file");
    serde_json::from_str(&contents).ok()
}

fn add_card_loop(cards: &mut Vec<Card>) {
    loop {
        let card = Card::new();
        cards.push(card);
        print!("\x1B[2J");

        match write_file(cards) {
            Err(_) => println!("{}", "Error writing to file".red().bold()),
            Ok(_) => println!("{}", "Card added".green().bold()),
        }

        println!("{}", "Add another card? (y/n)".yellow().bold());
        let input = take_input();
        match input {
            None => {
                println!("{}", "Exiting...".red());
                break;
            }
            Some(x) => {
                if x == "y" {
                    print!("\x1B[2J");
                    continue;
                } else {
                    println!("{}", "Exiting...".red());
                    break;
                }
            }
        }
    }
}

fn dispay_cards(cards: &mut Vec<Card>) {
    if cards.is_empty() {
        println!("{}", "No cards to display".red().bold());
        return;
    }

    println!("{}", "Displaying cards...".yellow().bold());
    for card in cards {
        println!("\n----------------------\n\n{}", card);
    }
    println!("\n----------------------\n");
}
fn main() {
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();

    let mut cards: Vec<Card> = vec![];

    match open_cards() {
        None => {
            println!("{}", "No cards found. Creating new cards.".cyan());
        }
        Some(x) => {
            cards = x;
            println!(
                "{} {} {}",
                "Successfully loaded".green().bold(),
                cards.len().to_string().green().underline(),
                "cards.".green().bold()
            );
        }
    }

    loop {
        println!("{}", "What would you like to do?".cyan().bold());
        println!("{}", "1. Add a card".cyan().bold());
        println!("{}", "2. View cards".cyan().bold());
        println!("{}", "3. Exit".cyan().bold());
        let input = take_input();
        print!("\x1B[2J");
        match input {
            None => {
                println!("{}", "Exiting...".red());
                break;
            }
            Some(x) => {
                if x == "1" {
                    add_card_loop(&mut cards);
                } else if x == "2" {
                    dispay_cards(&mut cards);
                } else if x == "3" {
                    println!("{}", "Exiting...".red());
                    break;
                } else {
                    println!("{}", "Invalid input".red());
                }
            }
        }
    }
}
