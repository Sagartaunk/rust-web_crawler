# Rust Web Crawler

## Overview
This is a basic Rust web crawler that fetches and analyzes web pages. It is designed to:
- Fetch web pages from given URLs.
- Extract links from these web pages.
- Follow links to fetch additional web pages up to a specified depth.
- Store the fetched web pages' metadata (like title, meta descriptions, and URLs) in a structured format (e.g., JSON).
- Support concurrent fetching using Rust's async features.

## Project Structure
The project is organized as a Cargo workspace with multiple crates to separate concerns:
- `crawler`: The main crate that handles the web crawling logic.
- `fetcher`: A crate responsible for fetching web pages.
- `parser`: A crate that parses HTML and extracts links.
- `storage`: A crate that handles storing and managing the fetched data.
- `web_crawler`: The binary crate that serves as the entry point of the application.

## Dependencies
This project uses the following dependencies:
- `tokio`: For asynchronous runtime.
- `reqwest`: For making HTTP requests.
- `scraper`: For parsing HTML and extracting elements.
- `serde` and `serde_json`: For serializing and deserializing JSON data.

## Installation
To build and run this project, you need to have Rust and Cargo installed. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started).

Clone the repository and navigate to the project directory:
```bash
git clone https://github.com/Sagartaunk/rust-web_crawler.git
cd rust-web-crawler
```

## Usage
To run the web crawler, use the following command:
```bash
cargo run --bin web_crawler
```

This will start the web crawler, which will:
- Begin crawling from the specified URL (e.g., `http://example.com`).
- Follow links up to the specified depth (e.g., depth of 3).
- Save the results to a file you name.


## Example
Here is an example of how to use the web crawler:
1. Open `web_crawler/src/main.rs`.
2. Modify depth as needed:
3. Run the web crawler:
    ```bash
    cargo run --bin web_crawler
    ```

## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue.

