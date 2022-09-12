use select::document::Document;
use select::predicate::Class;

use serde::Deserialize;
use std::fs;
// use std::future::Future;
extern crate reqwest;

mod file;
mod text;

#[derive(Debug, Deserialize)]
struct Rendered {
    rendered: String,
}

#[derive(Debug, Deserialize)]
struct Post {
    id: i32,
    date: String,
    date_gmt: String,
    modified: String,
    modified_gmt: String,
    slug: String,
    // enumにしたい
    status: String,
    link: String,
    title: Rendered,
    content: Rendered,
    excerpt: Rendered,
    categories: Vec<i32>,
}

struct PostMetadata {
    tags: String,
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // TODO(okubo): 1. slugをdir名にして、requestに応じてdirを作成してその中でファイルを作っている
    // TODO(okubo): 2. file名はマジックナンバーとする。記事ごとの管理となるので、あまりそこは気にしない
    // TODO(okubo): 3. 記事はindex.mdで保存していく
    // TODO(okubo): 4. header部分にはmeta情報などを記載
    // TODO(okubo): 5. wordpressのtotal count情報からloopして検索を行っていく

    // let resp = reqwest::Client::new()
    //     .get("https://mokubo.website/2022/08/i-installed-cloud_firestore-in-flutter-and-got-both-msporturlconnectiondelegate/")
    //     .send()
    //     .await
    //     .unwrap();

    // TODO(okubo): totalの数字をPERで割って全てを取得する。そしてVecを作成
    // TODO(okubo): 数字自体を取得する箇所で、問題なく動く
    // let total_posts = resp.headers().get("x-wp-total").unwrap().to_str().unwrap();
    // println!("posts length is {}:", total_posts);

    println!("sentinel1");
    let post: Post = reqwest::Client::new()
        .get("https://mokubo.website/wp-json/wp/v2/posts/4842")
        .send()
        .await?
        .json()
        .await?;
    println!("sentinel2");

    // NOTE(okubo): 管理しやすいように上の階層で対応
    // TODO(okubo): unwrapではなくerror handlingしたほうが良い
    fs::create_dir_all(format!("./posts/{}", &post.slug)).unwrap();

    // NOTE(okubo): HTMLを扱うためのhack
    let modified_content = format!("<div class='post'>{}</div>", post.content.rendered);
    let document = Document::from_read(modified_content.as_bytes()).unwrap();
    let html = document.find(Class("post")).next().unwrap();

    // TODO(okubo): parse_textの先頭にtitle description, created_atも入れる
    let sections = html
        .children()
        .map(|node| text::parse_text(node))
        .collect::<Vec<_>>();

    let mut index = 1;
    // NOTE(okubo): 画像を保存
    for section in sections.clone() {
        if section.is_image() {
            match section.download_image(&post.slug, index).await {
                Ok(_) => {
                    index += 1;
                    println!("created file");
                }
                Err(_) => eprintln!("failured"),
            };
        }
    }

    let _post_metadata = PostMetadata {
        tags: String::from(""),
        title: post.title.rendered,
    };

    // let metadata = format!("```metadata\n{}\n```", post_metadata);

    let section_string = sections
        .into_iter()
        .map(|section| section.content)
        .collect::<Vec<_>>();

    match file::create_file(post.slug, section_string.join("\n")) {
        Ok(_) => println!("success"),
        Err(_) => eprintln!("error"),
    };
    Ok(())
}
