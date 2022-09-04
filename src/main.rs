use select::document::Document;
use select::predicate::Class;

use serde::Deserialize;
extern crate reqwest;

mod file;
mod text;

#[derive(Debug, Deserialize)]
struct Rendered {
    rendered: String,
}

#[derive(Debug, Deserialize)]
struct Post {
    // #[serde(rename = "id")]
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

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // TODO(okubo): 1. slugをdir名にして、requestに応じてdirを作成してその中でファイルを作っている
    // TODO(okubo): 2. file名はマジックナンバーとする。記事ごとの管理となるので、あまりそこは気にしない
    // TODO(okubo): 3. 記事はindex.mdで保存していく
    // TODO(okubo): 4. header部分にはmeta情報などを記載
    // TODO(okubo): 5. wordpressのtotal count情報からloopして検索を行っていく
    let post: Post = reqwest::Client::new()
        .get("https://mokubo.website/wp-json/wp/v2/posts/4846")
        .send()
        .await?
        .json()
        // .text()
        .await?;

    // NOTE(okubo): HTMLを扱うためのhack
    let modified_content = format!("<div class='post'>{}</div>", post.content.rendered);
    // println!("{:#?}", post);
    let document = Document::from_read(modified_content.as_bytes()).unwrap();
    let html = document.find(Class("post")).next().unwrap();
    // let title = document.find(Class("entry-title")).next().unwrap().text();
    // println!("html is: {}", html.html());
    // let title = document.find(Class("entry-title")).next().unwrap().text();
    // println!("title is: {}", title);

    // let elements = post.find(Class("entry-content")).next().unwrap();
    let children = html
        .children()
        // .nodes
        // TODO(okubo): parse_textの先頭にtitle description, created_atも入れる
        .map(|tag| text::parse_text(tag))
        .collect::<Vec<_>>();
    //
    let content = children.join("\n");
    println!("Tag size is: {}", children.len().to_string());
    match file::create_file(content) {
        Ok(_) => println!("success"),
        Err(_) => eprintln!("error"),
    };
    Ok(())
}
