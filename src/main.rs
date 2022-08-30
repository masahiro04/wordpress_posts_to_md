use scraper::{Html, Selector};

use select::document::Document;
use select::predicate::{Class, Name, Predicate};
// use std::collections::HashMap;
extern crate reqwest;
// NOTE(okubo): 参考記事
// https://github.com/kadekillary/scraping-with-rust/blob/master/src/main.rs

fn main() {
    println!("Hello, world!");
    let response = reqwest::blocking::get(
        "https://mokubo.website/2022/08/how-to-create-your-own-vim-plugins/",
    )
    .unwrap();

    let document = Document::from_read(response).unwrap();
    let post = document.find(Class("post")).next().unwrap();

    for node in post.find(Name("img")) {
        // let url = node.find(Class("title").descendant(Name("a")));
        // let images = node.find(Name("img")).next().unwrap();

        println!("{}", node.attr("src").unwrap());
    }

    // let fragment = Html::parse_fragment(&response);
    // let images = Selector::parse("img").unwrap();
    //
    // for image in fragment.select(&images) {
    //     println!("{}", image.inner_html());
    // }

    // println!("{}", response);
    // println!("{}", fragment);
    // let resp = reqwest::blocking::get("https://httpbin.org/ip").json::<HashMap<String, String>>();
    // println!("{:#?}", resp);
}
