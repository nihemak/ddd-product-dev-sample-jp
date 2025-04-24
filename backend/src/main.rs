// use anyhow::Result; // anyhowは必須ではなくなるかも
// use std::sync::Arc;
use std::net::TcpListener;
use std::env;
use sqlx::PgPool;
use anyhow::Result;
use std::sync::Arc; // InMemoryRepository を Arc<Mutex<>> でラップするため

// クレートからモジュールや型をインポート
// use ddd_sample_jp::application; // 注文サービスを使うため
// use ddd_sample_jp::infrastructure; // InMemoryリポジトリを使うため
// use ddd_sample_jp::domain; // 各状態の型やリポジトリトレイトを使うため
// use ddd_sample_jp::domain; // 必要ならドメイン要素もインポート

// クレートからrun関数をインポート
use ddd_sample_jp::run;

// --- Main / Presentation Layer ---
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 環境変数からデータベース接続URLを取得
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // データベース接続プールを作成
    let connection_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create Postgres connection pool.");

    // アプリケーションがリッスンするアドレスとポート
    // TODO: 設定ファイルや環境変数から読み込むようにする
    let address = "0.0.0.0:8080";
    let listener = TcpListener::bind(address)?;
    println!("Listening on {}", address);

    // アプリケーションサーバーを起動
    run(listener, connection_pool).await?.await // runが返すServerをawaitし、その実行完了もawait

    /* ---- 既存のサンプルコードは一旦コメントアウト ----
    // --- 依存関係の構築 (Dependency Injection) ---
    let item_repo = Arc::new(infrastructure::InMemory商品Repository::new());
    let order_repo = Arc::new(infrastructure::InMemory注文Repository::new());
    let order_service = application::注文サービス::new(order_repo.clone(), item_repo.clone());

    println!("--- DDD Sample Start ---");

    let sample_item_ids = item_repo.get_sample_item_ids();

    if sample_item_ids.is_empty() {
        println!("サンプル商品がありません。");
        return Ok(());
    }
    let items_to_order = vec![sample_item_ids[0]];

    // --- ユースケースの実行 ---
    println!("\n1. 商品を注文します: {:?}", items_to_order);
    match order_service.注文受付(items_to_order) {
        Ok(order_id) => {
            println!("注文成功！ 注文ID: {:?}", order_id);

            println!("\n2. 注文詳細を取得します");
            match order_service.注文詳細取得(&order_id) {
                 Ok(Some(order)) => println!("取得した注文: {:?}", order),
                 Ok(None) => println!("注文が見つかりませんでした。"),
                 Err(e) => println!("注文取得エラー: {}", e),
            }

            println!("\n3. 注文を発送準備中にします");
            match order_service.注文発送準備(&order_id) {
                 Ok(_) => {
                     println!("発送準備中に状態変更成功！");
                     match order_service.注文詳細取得(&order_id) {
                         Ok(Some(order)) => println!("現在の注文状態: {:?}", order.状態),
                         _ => println!("注文再取得エラー"),
                     }

                     println!("\n4. 注文を発送済みにします");
                     match order_service.注文発送(&order_id) {
                         Ok(_) => {
                              println!("発送済みに状態変更成功！");
                             match order_service.注文詳細取得(&order_id) {
                                 Ok(Some(order)) => println!("現在の注文状態: {:?}", order.状態),
                                 _ => println!("注文再取得エラー"),
                             }
                         }
                         Err(e) => println!("発送済みへの変更エラー: {}", e),
                     }
                 }
                 Err(e) => println!("発送準備中への変更エラー: {}", e),
            }

        }
        Err(e) => {
            println!("注文エラー: {}", e);
        }
    }

     println!("\n5. 受付済みの注文を直接発送済みにしようとしてみる（エラーになるはず）");
     if !sample_item_ids.is_empty() {
        let items_to_order_2 = vec![sample_item_ids[0]];
        if let Ok(order_id_2) = order_service.注文受付(items_to_order_2) {
            println!("新しい注文ID: {:?}", order_id_2);
            match order_service.注文発送(&order_id_2) {
                 Ok(_) => println!("エラーが発生するはずが、成功してしまいました。"),
                 Err(e) => println!("期待通りのエラー: {}", e),
            }
        }
     } else {
         println!("不正遷移テスト用の商品がありません。");
     }


    println!("--- DDD Sample End ---");
    ---- 既存のサンプルコードここまで ---- */

    // Ok(()) // `run(..).await?.await` が Result<(), std::io::Error> を返すので不要
} 