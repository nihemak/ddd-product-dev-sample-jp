// src/lib.rs - ライブラリのエントリーポイント

// モジュール宣言 (ファイルから読み込む)
pub mod domain;
pub mod application;
pub mod infrastructure;

// domain モジュール内の要素をトップレベルに re-export してみる (問題解決のため)
// pub use domain::*;
// application や infrastructure も必要に応じて re-export 可能
// pub use application::*;
// pub use infrastructure::*; 