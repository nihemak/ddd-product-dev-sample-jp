use std::env;

fn main() {
    // CI環境変数に基づいて cfg(ci) フラグを設定する
    if env::var("CI").is_ok() {
        println!("cargo:rustc-cfg=ci");
    }
} 