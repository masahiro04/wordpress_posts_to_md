use regex::Regex;
use select::node::Node;
use select::predicate::Name;
use std::fs::File;

pub enum SectionKind {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Text,
    Link,
    Code,
    Image(String),
}

pub struct Section {
    pub kind: SectionKind,
    pub content: String,
}

// use tokio;
fn make_header_regex(n: i32) -> String {
    format!(r"<h{}(?: .+?)?>.*?</h{}>", n, n)
}

// #[tokio::main]
pub fn parse_text(node: Node) -> Section {
    let trimed_html = trim_newline(node.html());
    if Regex::new(&make_header_regex(1))
        .unwrap()
        .is_match(&trimed_html)
    {
        return Section {
            kind: SectionKind::H1,
            content: format!("# {}", node.text()),
        };
        // return Ok(format!("# {}", node.text()));
    } else if Regex::new(&make_header_regex(2))
        .unwrap()
        .is_match(&trimed_html)
    {
        return Section {
            kind: SectionKind::H2,
            content: format!("## {}", node.text()),
        };
        // return Ok(format!("## {}", node.text()));
    } else if Regex::new(&make_header_regex(3))
        .unwrap()
        .is_match(&trimed_html)
    {
        return Section {
            kind: SectionKind::H3,
            content: format!("### {}", node.text()),
        };
        // return Ok(format!("### {}", node.text()));
    } else if Regex::new(&make_header_regex(4))
        .unwrap()
        .is_match(&trimed_html)
    {
        return Section {
            kind: SectionKind::H4,
            content: format!("#### {}", node.text()),
        };
        // return Ok(format!("#### {}", node.text()));
    } else if Regex::new(&make_header_regex(5))
        .unwrap()
        .is_match(&trimed_html)
    {
        return Section {
            kind: SectionKind::H5,
            content: format!("##### {}", node.text()),
        };
        // return Ok(format!("##### {}", node.text()));
    } else if Regex::new(&make_header_regex(6))
        .unwrap()
        .is_match(&trimed_html)
    {
        return Section {
            kind: SectionKind::H6,
            content: format!("###### {}", node.text()),
        };
        // return Ok(format!("###### {}", node.text()));
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

        return Section {
            kind: SectionKind::Link,
            content: format!("[{}]({})", node.text(), href),
        };

        // return Ok(format!("[{}]({})", node.text(), href));
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

        return Section {
            kind: SectionKind::Code,
            content: format!("```{}\n {}\n```", lang, node.text()),
        };

        // return Ok(format!("```{}\n {}\n```", lang, node.text()));
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
        // let mut file = File::create(file_name).unwrap();
        // let image_string: String = reqwest::Client::new().get(src).send().await?.text().await?;
        // std::io::copy(&mut image_string.as_bytes(), &mut file);

        // .copy_to(&mut file)
        // .unwrap();

        return Section {
            kind: SectionKind::Image(src.to_string()),
            content: format!("![GitHubでリビジョン管理](./{})", file_name),
        };

        // return Ok(format!("![GitHubでリビジョン管理](./{})", file_name));
    }

    return Section {
        kind: SectionKind::Text,
        content: format!("{}", trimed_html),
    };

    // Ok(format!("{}", trimed_html))
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

    unused_tag(base_string)
}
