extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        //引数解析に問題
        eprintln!("引数解析時に問題が発生しました: {}", err);
        process::exit(1);
    });

    println!("検索文字列: {}", config.query);
    println!("ファイル名: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("アプリケーションエラー: {}", e);

        process::exit(1);
    }
}