use serde::{Serialize, Deserialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Pages{
    pub url: String,
    pub title : String,
    pub body: String,
    pub links: Vec<String>,
}

pub fn save_pages(pages : Vec<Pages> , path : &str){
    let file = File::create(path).expect("Unable to create file");
    let _ = serde_json::to_writer_pretty(file, &pages);
}