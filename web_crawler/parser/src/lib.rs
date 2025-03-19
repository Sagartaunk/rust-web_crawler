use scraper::{Html , Selector};
pub fn parser(html: &str) -> Vec<String>{
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let mut links = Vec::new();
    for i in document.select(&selector){
        let link = i.value().attr("href");
        if let Some(link) = link{
            links.push(link.to_string());
        }
    }
    links
}