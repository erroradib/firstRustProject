# Anime Quote Fetcher

This is my first project in Rust! I built a small command-line program that fetches a random anime quote and shows the quote, anime title, alternate title, and character.

It has been fun seeing how fast Rust can run. This project also gave me a chance to try Rust's type system and work with data from a web API.

## What it does

- Sends an HTTP request to the Animechan API
- Parses the JSON response into Rust structs
- Prints the quote and its anime and character details
- Waits for Enter before closing, so the output stays visible

## Tech used

- **Rust** — a compiled systems programming language focused on performance and memory safety
- **Cargo** — Rust's package manager and build tool
- **Reqwest** — an HTTP client crate used to call the API
- **Serde** — a serialization framework used here to deserialize JSON into Rust data structures
- **Animechan API** — the source of random anime quotes

## Run it

You need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed. Clone the repository, open a terminal in the project folder, and run:

```sh
cargo run
```

Cargo compiles the program and runs it. When it finishes, press Enter to close it.

## What I learned

This first project helped me practice:

- Defining `struct`s to represent API data
- Using `Result` and the `?` operator to handle errors
- Adding external crates with Cargo
- Making a blocking HTTP request and deserializing its JSON response

## Note

The program needs an internet connection to fetch a quote from the API.
