use regex::Regex;
use select::node::Node;
use select::predicate::Name;
use std::fs::File;
fn make_header_regex(n: i32) -> String {
    format!(r"<h{}(?: .+?)?>.*?</h{}>", n, n)
}

pub fn parse_text(node: Node) -> String {
    let trimed_html = trim_newline(node.html());
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
        // let splited_file_name: Vec<String> = file_name.split(".").map(|s| s.to_string()).collect();
        // let extension = splited_file_name.last().unwrap();

        // NOTE(okubo): 画像保存機能
        // let mut file = File::create(file_name).unwrap();
        // reqwest::blocking::get(src)
        //     .unwrap()
        //     .copy_to(&mut file)
        //     .unwrap();

        // return format!("![GitHubでリビジョン管理](./{})", file_name);
    }

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

    unused_tag(base_string)
}
