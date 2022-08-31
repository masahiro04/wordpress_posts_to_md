// use std::error::Error;
// use std::io;
// use std::fs::OpenOptions;

// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;
//
// use headless_chrome::protocol::cdp::Page;
// use headless_chrome::Browser;
use select::document::Document;
use select::predicate::Class;
// use std::collections::HashMap;
extern crate reqwest;
// NOTE(okubo): 参考記事
// https://github.com/kadekillary/scraping-with-rust/blob/master/src/main.rs

// note: 下記の構成を参考に実装する
// note: https://github.com/catnose99/CatKnows/tree/master/content/blog

// fn create_file(content: String) -> Result<(), String> {
//     let path = Path::new("lorem_ipsum.md");
//     let display = path.display();
//
//     // Open a file in write-only mode, returns `io::Result<File>`
//     // ファイルを書き込み専用モードで開く。返り値は`io::Result<File>`
//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}: {}", display, why),
//         Ok(file) => file,
//     };
//
//     // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
//     // `LOREM_IPSUM`の文字列を`file`に書き込む。返り値は`io::Result<()>`
//     match file.write_all(content.as_bytes()) {
//         Err(why) => panic!("couldn't write to {}: {}", display, why),
//         Ok(_) => println!("successfully wrote to {}", display),
//     }
//     Ok(())
// }

fn main() {
    let response = reqwest::blocking::get(
        "https://mokubo.website/2022/08/how-to-issue-custom-queries-in-supabase-db/",
    )
    .unwrap();

    let document = Document::from_read(response).unwrap();
    let post = document.find(Class("post")).next().unwrap();

    let tags = post
        .find(Class("entry-content"))
        .map(|tag| tag.html())
        .collect::<Vec<_>>();

    println!("Taggs:{}", tags.join(", "));

    // for node in post.find(Class("entry-content")) {
    //     let text = node.text();
    //     println!("{}", text);
    //     println!("------------");
    //     // let url = node.find(Class("title").descendant(Name("a")));
    //     // let images = node.find(Name("img")).next().unwrap();
    //
    //     // println!("{:?}", node.as_text());
    //     // println!("{}", node.attr("src").unwrap());
    // }
}
