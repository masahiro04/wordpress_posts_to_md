// use scraper::Html;
use std::error::Error;

// use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use select::document::Document;
use select::predicate::{Class, Name};
// use std::collections::HashMap;
extern crate reqwest;
// NOTE(okubo): 参考記事
// https://github.com/kadekillary/scraping-with-rust/blob/master/src/main.rs

fn browse_wikipedia(url: String) -> Result<String, Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(&url)?;
    tab.wait_until_navigated().unwrap();

    let mut content = String::new();
    match tab.get_content() {
        Ok(t) => content = t,
        Err(e) => eprintln!("{}", e),
    }
    Ok(content)
}
fn main() {
    let mut content = String::new();
    match browse_wikipedia(String::from(
        "https://mokubo.website/2022/08/how-to-create-your-own-vim-plugins/",
    )) {
        Ok(t) => content = t,
        Err(_) => {
            println!("Oops");
        }
    }

    println!("Hello, world!");
    // let response = reqwest::blocking::get(
    //     "https://mokubo.website/2022/08/how-to-create-your-own-vim-plugins/",
    // )
    // .unwrap();

    // https://stackoverflow.com/questions/32674905/pass-string-to-function-taking-read-trait
    let document = Document::from_read(content.as_bytes()).unwrap();
    let post = document.find(Class("post")).next().unwrap();

    for node in post.find(Name("img")) {
        // let url = node.find(Class("title").descendant(Name("a")));
        // let images = node.find(Name("img")).next().unwrap();

        println!("{}", node.attr("src").unwrap());
    }
}
