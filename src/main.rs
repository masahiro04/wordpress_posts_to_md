use regex::Regex;

// use std::error::Error;
use std::fs::File;
// use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use select::document::Document;

use select::node::Node;
use select::predicate::{Class, Name};
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

fn make_header_regex(n: i32) -> String {
    format!(r"<h{}(?: .+?)?>.*?</h{}>", n, n)
}

fn parse_text(node: Node) -> String {
    let trimed_html = trim_newline(node.html());
    // let re = String::from(r"<h5(?: .+?)?>.*?</h5>");
    if Regex::new(&make_header_regex(1))
        .unwrap()
        .is_match(&trimed_html)
    {
        return format!("# {}", node.text());
    } else if Regex::new(&make_header_regex(2))
        .unwrap()
        .is_match(&trimed_html)
    {
        return format!("## {}", node.text());
    } else if Regex::new(&make_header_regex(3))
        .unwrap()
        .is_match(&trimed_html)
    {
        return format!("### {}", node.text());
    } else if Regex::new(&make_header_regex(4))
        .unwrap()
        .is_match(&trimed_html)
    {
        return format!("#### {}", node.text());
    } else if Regex::new(&make_header_regex(5))
        .unwrap()
        .is_match(&trimed_html)
    {
        return format!("##### {}", node.text());
    } else if Regex::new(&make_header_regex(6))
        .unwrap()
        .is_match(&trimed_html)
    {
        return format!("###### {}", node.text());
    } else if Regex::new(&r"<a(?: .+?)?>.*?</a>")
        .unwrap()
        .is_match(&trimed_html)
    {
        let atag = node.find(Name("a")).next().unwrap();
        let empty_text = String::from("");
        let href = atag.attr("href");
        let href = match href {
            Some(result) => result,
            None => &empty_text,
        };

        return format!("[{}]({})", node.text(), href);
        // NOTE(okubo): codeの中は改行も許可
    } else if Regex::new(&r"<code(?: .+?)?>\n|\r\n|\r|.*?</code>")
        .unwrap()
        .is_match(&trimed_html)
    {
        let code = node.find(Name("code")).next().unwrap();
        let empty_text = String::from("");
        let lang = match code.attr("lang") {
            Some(result) => result,
            None => &empty_text,
        };
        return format!("```{}\n {}\n```", lang, node.text());
    } else if Regex::new(&r"<img(?: .+?)?>")
        .unwrap()
        .is_match(&trimed_html)
    {
        println!("img tag haittayo!!!!!!!!!!!!;");
        let img = node.find(Name("img")).next().unwrap();
        let empty_text = String::from("");
        let src = match img.attr("src") {
            Some(result) => result,
            None => &empty_text,
        };

        let splited_image_url: Vec<String> = src.split("/").map(|s| s.to_string()).collect();
        let file_name = splited_image_url.last().unwrap();
        let splited_file_name: Vec<String> = file_name.split(".").map(|s| s.to_string()).collect();
        let extension = splited_file_name.last().unwrap();

        // NOTE(okubo): 画像保存機能
        let mut file = File::create(file_name).unwrap();
        let response = reqwest::blocking::get(src)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();

        println!("image src is :{}", src);
        println!("file name is :{}", file_name);
        println!("extension  is :{}", extension);
        return format!("![GitHubでリビジョン管理](./{})", file_name);
    }

    // TODO(okubo): 画像の機能を追加する

    format!("{}", trimed_html)
}

fn unused_tag(s: String) -> String {
    let mut trimed_string = String::from(&s);
    let re = Regex::new(&r"<p(?: .+?)?>|</p>|<pre(?: .+?)?>|</pre>|<figure(?: .+?)?>|</figure>|<pre(?: .+?)?>|</pre>")
        .unwrap();
    trimed_string = re.replace_all(&trimed_string, "").to_string();
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
        // TODO(okubo): parse_textの先頭にtitle description, created_atも入れる
        .map(|tag| parse_text(tag))
        .collect::<Vec<_>>();

    let content = children.join("\n");
    println!("Tag size is: {}", children.len().to_string());
    match create_file(content) {
        Ok(_) => println!("success"),
        Err(_) => eprintln!("error"),
    };
}
