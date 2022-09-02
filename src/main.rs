use regex::Regex;

use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;

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

fn create_file(content: String) -> Result<(), String> {
    let path = Path::new("lorem_ipsum.md");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    // ファイルを書き込み専用モードで開く。返り値は`io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    // `LOREM_IPSUM`の文字列を`file`に書き込む。返り値は`io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    Ok(())
}

// TODO(okubo): parserの作り方
// https://betterprogramming.pub/create-your-own-markdown-parser-bffb392a06db
// 上記はmarkdown to htmlしているので逆でいけそう
// また方針としては[<h1>から始まり、</h1>で終わる]というような
// 1行ずつ処理を行う、というので問題なさそう
// 当然全部一気だと楽だけど、それだと正確に変換できないので、正確に変換することを
// したいがために、1行ずつの実行とする
// もしくは引数をListにして、複数でも良いし、単数でも、みたいな形でもいいかも

// phpだけどこのOSSも参考になる
// https://github.com/thephpleague/html-to-markdown/blob/master/src/Element.php

fn filter_tag(html: String) {
    let s = "hogehoge";
    let re = Regex::new(r"^hoge").unwrap();
    println!("{}", re.is_match(s));
}

fn trim_newline(s: &mut String) -> &mut String {
    loop {
        if s.ends_with('\n') {
            s.pop();
        } else {
            break;
        }
    }
    s
}

fn main() {
    let response = reqwest::blocking::get(
        "https://mokubo.website/2022/08/how-to-issue-custom-queries-in-supabase-db/",
    )
    .unwrap();

    let document = Document::from_read(response).unwrap();
    let post = document.find(Class("post")).next().unwrap();

    // filter_tag(String::from("hoge"));

    let tags = post.find(Class("entry-content")).next().unwrap();

    let children = tags
        .children()
        .map(|tag| {
            // note: 正規表現の参考リンク
            // https://webdesign.vdlz.xyz/Editor/hidemaru/Regex/WebSiteRegex.html
            let tag_ref = &tag;
            println!("{}", &tag_ref.html());
            let re = Regex::new(r"<h2>(.*?)</h2>").unwrap();
            let is_matched = re.is_match(&tag_ref.html());
            println!("{}", re.is_match(&tag_ref.html()));

            let mut html = tag.html();
            trim_newline(&mut html);
            html

            // is_matched.to_string()
            // String::from("hogehgoe")
        })
        .collect::<Vec<_>>();

    // TODO: tagsが一つでまとまってしまっているので、loopの段階で切り分ける
    // もしダメならregexでもいいかも
    let content = children.join("\n");
    println!("Tag size is: {}", children.len().to_string());
    // println!("Taggs:{}", children.join(", "));
    // let contents = children.join(", ");
    match create_file(content) {
        Ok(_) => println!("success"),
        Err(_) => eprintln!("error"),
    };

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
