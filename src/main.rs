use select::document::Document;
use select::predicate::Class;

extern crate reqwest;

mod file;
mod text;

fn main() {
    // TODO(okubo): 1. slugをdir名にして、requestに応じてdirを作成してその中でファイルを作っている
    // TODO(okubo): 2. file名はマジックナンバーとする。記事ごとの管理となるので、あまりそこは気にしない
    // TODO(okubo): 3. 記事はindex.mdで保存していく
    // TODO(okubo): 4. header部分にはmeta情報などを記載
    // TODO(okubo): 5. wordpressのtotal count情報からloopして検索を行っていく

    let response = reqwest::blocking::get(
        "https://mokubo.website/2022/08/how-to-issue-custom-queries-in-supabase-db/",
    )
    .unwrap();

    // TODO(okubo): jsonで対応
    // let json_response =
    //     reqwest::blocking::get("https://mokubo.website/wp-json/wp/v2/posts/4846").unwrap();

    let document = Document::from_read(response).unwrap();
    let post = document.find(Class("post")).next().unwrap();

    let title = document.find(Class("entry-title")).next().unwrap().text();
    println!("title is: {}", title);
    let title = document.find(Class("entry-title")).next().unwrap().text();
    println!("title is: {}", title);

    let elements = post.find(Class("entry-content")).next().unwrap();
    let children = elements
        .children()
        // TODO(okubo): parse_textの先頭にtitle description, created_atも入れる
        .map(|tag| text::parse_text(tag))
        .collect::<Vec<_>>();

    let content = children.join("\n");
    println!("Tag size is: {}", children.len().to_string());
    match file::create_file(content) {
        Ok(_) => println!("success"),
        Err(_) => eprintln!("error"),
    };
}
