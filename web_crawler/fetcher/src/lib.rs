use reqwest::Error;
pub async fn fetch(url: &str) -> Result<String, Error>{
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}