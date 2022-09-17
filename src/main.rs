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

fn main() {
    // let total_posts = 472;
    let total_posts = 40;
    let per_page = 20;
    let count_of_requests = total_posts / per_page;
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
}

fn post_to_md(post: Post, categories: &Vec<Category>) {
    // NOTE(okubo): 管理しやすいように上の階層で対応
    // TODO(okubo): unwrapではなくerror handlingしたほうが良い
    fs::create_dir_all(format!("./posts/{}", &post.slug)).unwrap();

    // NOTE(okubo): HTMLを扱うためのhack
    let modified_content = format!("<div class='post'>{}</div>", post.content.rendered);
    let document = Document::from_read(modified_content.as_bytes()).unwrap();
    let html = document.find(Class("post")).next().unwrap();

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
}}
```"#,
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
