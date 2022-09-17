use select::document::Document;
use select::predicate::Class;

use serde::Deserialize;
use std::fs;
extern crate reqwest;

mod file;
mod text;

#[derive(Debug, Deserialize, Clone)]
struct Rendered {
    rendered: String,
}

#[derive(Debug, Deserialize, Clone)]
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

// TODO(okubo): entities移動させたい
#[derive(Debug, Deserialize)]
struct Category {
    id: i32,
    name: String,
}

// TODO(okubo): usecaseに移動させたい
fn get_category_name_by_id(categories: &Vec<Category>, ids: Vec<i32>) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();

    for id in ids {
        categories
            .into_iter()
            .map(|category| {
                if category.id == id {
                    names.push(category.name.clone());
                }
            })
            .collect::<Vec<_>>();
    }
    names
}

// #[tokio::main]
fn main() {
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

    // let total_posts = 472;
    let total_posts = 40;
    let per_page = 20;
    let count_of_requests = total_posts / per_page;
    // TODO(okubo): totalの数字をPERで割って全てを取得する。そしてVecを作成
    // TODO(okubo): 数字自体を取得する箇所で、問題なく動く
    // let total_posts = resp.headers().get("x-wp-total").unwrap().to_str().unwrap();
    // println!("posts length is {}:", total_posts);

    // NOTE(okubo): categoriesを全権取得
    // TODO(okubo): 記事のidからcategoriesのnameを返す
    let categories =
        reqwest::blocking::get("https://mokubo.website/wp-json/wp/v2/categories?per_page=100")
            .unwrap()
            .json::<Vec<Category>>()
            .unwrap();

    let mut page = 1;
    let mut posts: Vec<Post> = Vec::new();
    loop {
        let fetched_posts = reqwest::blocking::get(format!(
            "https://mokubo.website/wp-json/wp/v2/posts?page={}&per_page={}",
            page, per_page,
        ))
        .unwrap()
        .json::<Vec<Post>>()
        .unwrap();

        posts = [posts, fetched_posts].concat();

        if count_of_requests < page {
            break;
        }

        page += 1;
    }

    for post in posts {
        post_to_md(post, &categories);
    }

    // let post = reqwest::blocking::get("https://mokubo.website/wp-json/wp/v2/posts/4627")
    //     .unwrap()
    //     .json::<Post>()
    //     .unwrap();

    // NOTE(okubo): 管理しやすいように上の階層で対応
    // TODO(okubo): unwrapではなくerror handlingしたほうが良い
    //     fs::create_dir_all(format!("./posts/{}", &post.slug)).unwrap();
    //
    //     // NOTE(okubo): HTMLを扱うためのhack
    //     let modified_content = format!("<div class='post'>{}</div>", post.content.rendered);
    //     let document = Document::from_read(modified_content.as_bytes()).unwrap();
    //     let html = document.find(Class("post")).next().unwrap();
    //
    //     // TODO(okubo): parse_textの先頭にtitle description, created_atも入れる
    //     let sections = html
    //         .children()
    //         .map(|node| text::parse_text(node))
    //         .collect::<Vec<_>>();
    //
    //     let mut index = 1;
    //     // NOTE(okubo): 画像を保存
    //     for section in sections.clone() {
    //         if section.is_image() {
    //             match section.download_image(&post.slug, index) {
    //                 Ok(_) => {
    //                     index += 1;
    //                     println!("created file");
    //                 }
    //                 Err(_) => eprintln!("failured"),
    //             };
    //         }
    //     }
    //
    //     let category_names = get_category_name_by_id(&categories, post.categories)
    //         .into_iter()
    //         .map(|category| category)
    //         .collect::<Vec<_>>()
    //         .join(", ");
    //     let metadata: Vec<String> = vec![format!(
    //         r#"```metadata
    // {{
    //     "title": "{}",
    //     "date": "{}",
    //     "categories": "{}",
    // }}```"#,
    //         post.title.rendered, post.date, category_names,
    //     )];
    //
    //     let section_string = sections
    //         .into_iter()
    //         .map(|section| section.content)
    //         .collect::<Vec<_>>();
    //     let str_vec: Vec<String> = [metadata, section_string].concat();
    //
    //     match file::create_file(post.slug, str_vec.join("\n")) {
    //         Ok(_) => println!("success"),
    //         Err(_) => eprintln!("error"),
    //     };
    // Ok(())
}

fn post_to_md(post: Post, categories: &Vec<Category>) {
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
            match section.download_image(&post.slug, index) {
                Ok(_) => {
                    index += 1;
                    println!("created file");
                }
                Err(_) => eprintln!("failured"),
            };
        }
    }

    let category_names = get_category_name_by_id(&categories, post.categories)
        .into_iter()
        .map(|category| category)
        .collect::<Vec<_>>()
        .join(", ");
    let metadata: Vec<String> = vec![format!(
        r#"```metadata
{{
    "title": "{}",
    "date": "{}",
    "categories": "{}",
}}```"#,
        post.title.rendered, post.date, category_names,
    )];

    let section_string = sections
        .into_iter()
        .map(|section| section.content)
        .collect::<Vec<_>>();
    let str_vec: Vec<String> = [metadata, section_string].concat();

    match file::create_file(post.slug, str_vec.join("\n")) {
        Ok(_) => println!("success"),
        Err(_) => eprintln!("error"),
    };
    // Ok(())
}
