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

use select::node::Node;
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

struct Rule {
    rule: String,
    format: String,
}

fn unused_tag(s: String) -> String {
    let mut trimed_string = String::from(&s);
    println!("before string: {}", s);
    let re = Regex::new(&r"<p(?: .+?)?>|</p>|<pre(?: .+?)?>|</pre>|<figure(?: .+?)?>|</figure>")
        .unwrap();
    trimed_string = re.replace_all(&trimed_string, "").to_string();
    println!("after string: {}", trimed_string);
    trimed_string.to_string()
}
fn trim_newline(s: String) -> String {
    let mut base_string = s;
    loop {
        if base_string.ends_with('\n') {
            base_string.pop();
        } else {
            break;
        }
    }

    base_string = unused_tag(base_string);
    base_string
}

fn html_to_markdown(node: Node) -> String {
    let rules = vec![
        Rule {
            rule: String::from(r"<h2(?: .+?)?>.*?</h2>"),
            format: format!("##{}", node.text()),
        },
        // Rule {
        //     rule: String::from(r"<p(?: .+?)?>.*?</p>"),
        //     format: format!("{}", node.text()),
        // },
    ];

    let trimed_html = trim_newline(node.html());
    let mut parsed_string = String::new();
    for rule in rules {
        let re = Regex::new(&rule.rule).unwrap();
        let is_matched = re.is_match(&trimed_html);
        if is_matched {
            parsed_string = rule.format;
        }
    }
    if parsed_string.is_empty() {
        return trimed_html;
    }
    parsed_string
}

fn main() {
    let response = reqwest::blocking::get(
        "https://mokubo.website/2022/08/how-to-issue-custom-queries-in-supabase-db/",
    )
    .unwrap();
    let document = Document::from_read(response).unwrap();
    let post = document.find(Class("post")).next().unwrap();
    let tags = post.find(Class("entry-content")).next().unwrap();
    let children = tags
        .children()
        .map(|tag| html_to_markdown(tag))
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
}
