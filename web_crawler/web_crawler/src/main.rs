use crawler::Crawler;
use tokio::runtime::Runtime;
use std::io;

fn main() {
    let runtime = Runtime::new().unwrap();
    let crawler = std::sync::Arc::new(Crawler::new());
    println!("Enter the URL to crawl: ");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read line");
    let url = url.trim();
    println!("Enter where to save results (add .json after the file name) : ");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read line");
    let path = path.trim();
    println!("Running the crawler this may take a while ....");

    runtime.block_on(async {
        crawler.clone().crawl(url, 3).await; //MODIFY THE DEPT HERE EDIT THE NUMBER
        crawler.clone().save(path).await;
    });
    println!("Crawling completed");
}