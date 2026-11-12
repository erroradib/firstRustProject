
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Anime {
    id: u32,
    name: String,
    #[serde(rename = "altName")]
    alt_name: String,
}

#[derive(Debug, Deserialize)]
struct Character {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
struct QuoteData {
    content: String,
    anime: Anime,
    character: Character,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    status: String,
    data: QuoteData,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("\nCould not fetch an anime quote: {error}");
    }

    println!("\nPress Enter to close");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

fn run() -> Result<(), Box<dyn Error>> {

    let url = "https://api.animechan.io/v1/quotes/random";
    let client = Client::new();

    let response = client
        .get(url)
        .send()?
        .error_for_status()?;

    let result: ApiResponse = response.json()?;

    if result.status == "success" {
        println!("Anime     : {}", result.data.anime.name);
        println!("Alt Name  : {}", result.data.anime.alt_name);
        println!("Character : {}", result.data.character.name);
        println!("\n\"{}\"", result.data.content);

    } else {
        println!("API broken.");
    }

    Ok(())
}
