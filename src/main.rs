use anyhow::Result;
use std::sync::Arc;

// クレートからモジュールや型をインポート
use ddd_sample_jp::application; // 注文サービスを使うため
use ddd_sample_jp::infrastructure; // InMemoryリポジトリを使うため
// use ddd_sample_jp::domain; // 必要ならドメイン要素もインポート

// --- Main / Presentation Layer ---
fn main() -> Result<()> {
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
    match order_service.place_order(items_to_order) {
        Ok(order_id) => {
            println!("注文成功！ 注文ID: {:?}", order_id);

            println!("\n2. 注文詳細を取得します");
            match order_service.get_order_details(&order_id) {
                 Ok(Some(order)) => println!("取得した注文: {:?}", order),
                 Ok(None) => println!("注文が見つかりませんでした。"),
                 Err(e) => println!("注文取得エラー: {}", e),
            }

            println!("\n3. 注文を発送準備中にします");
            match order_service.prepare_order(&order_id) {
                 Ok(_) => {
                     println!("発送準備中に状態変更成功！");
                     match order_service.get_order_details(&order_id) {
                         Ok(Some(order)) => println!("現在の注文状態: {:?}", order.状態),
                         _ => println!("注文再取得エラー"),
                     }

                     println!("\n4. 注文を発送済みにします");
                     match order_service.ship_order(&order_id) {
                         Ok(_) => {
                              println!("発送済みに状態変更成功！");
                             match order_service.get_order_details(&order_id) {
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
        if let Ok(order_id_2) = order_service.place_order(items_to_order_2) {
            println!("新しい注文ID: {:?}", order_id_2);
            match order_service.ship_order(&order_id_2) {
                 Ok(_) => println!("エラーが発生するはずが、成功してしまいました。"),
                 Err(e) => println!("期待通りのエラー: {}", e),
            }
        }
     } else {
         println!("不正遷移テスト用の商品がありません。");
     }


    println!("\n--- DDD Sample End ---");

    Ok(())
} 