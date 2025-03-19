use fetcher::fetch;
use parser::parser;
use storage::{Pages, save_pages};
use std::collections::HashSet;
use tokio::sync::Mutex;
use std::sync::Arc;
pub struct Crawler {
    pub visited: Arc<Mutex<HashSet<String>>>,
    pub pages: Arc<Mutex<Vec<Pages>>>,
}
impl Crawler{
    pub fn new() -> Self{Crawler { visited: (Arc::new(Mutex::new(HashSet::new()))), pages: Arc::new(Mutex::new(Vec::new())) }}
    pub fn crawl<'a>(self: Arc<Self>, url: &'a str, depth: usize) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            if depth == 0 {
                return;
            }

            let mut visited = self.visited.lock().await;
            if visited.contains(url) {
                return;
            }
            visited.insert(url.to_string());

            let body = fetch(url).await.unwrap_or_else(|_| "".to_string());

            let links = parser(&body);
            let page = Pages {
                url: url.to_string(),
                title: "Title Placeholder".to_string(),
                body: "Description Placeholder".to_string(),
                links: links.clone(),
            };

            let mut pages = self.pages.lock().await;
            pages.push(page);

            drop(visited);
            drop(pages);

            for link in links {
                self.clone().crawl(&link, depth - 1).await;
            }
        })
    }
    pub async fn save(self: Arc<Self>, path: &str) {
        let pages = self.pages.lock().await.clone();
        save_pages(pages, path);
    }
}