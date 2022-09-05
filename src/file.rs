use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
pub fn create_file(slug: String, content: String) -> Result<(), String> {
    // TODO(okubo): unwrapではなくerror handlingしたほうが良い
    // fs::create_dir_all(format!("./posts/{}", slug)).unwrap();
    let dir = format!("./posts/{}/index.md", slug);
    let path = Path::new(&dir);
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
