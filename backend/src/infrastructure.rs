use crate::domain::core::{
    プレゼント予約ベース, ユーザーID, ラッピング種類, 予約受付済みプレゼント予約型, 商品ID,
    届け先ID, 支払いID, 記念日, 金額,
};
use crate::domain::{
    DomainError, InfrastructureError, プレゼント予約Repository, プレゼント予約状態, 予約ID,
};
use async_trait::async_trait;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
// use dotenv::dotenv; // 未使用
// use crate::domain::core::予約を受け付ける; // Clippy: unused import
// use chrono::NaiveDate; // Clippy: unused import
use chrono_tz::Asia::Tokyo;
use uuid::Uuid;

// --- インメモリリポジトリの実装 ---

/* --- 古い商品リポジトリ実装をコメントアウト --- */
/*
#[derive(Clone, Default)]
pub struct InMemory商品Repository {
    items: Arc<Mutex<HashMap<商品ID, 商品>>>,
}

impl InMemory商品Repository {
     pub fn new() -> Self {
         let mut items_map = HashMap::new();
         let item1 = 商品 {
             id: 商品ID::new(),
             名前: "すごい本".to_string(),
             価格: 3000,
         };
         let item2 = 商品 {
             id: 商品ID::new(),
             名前: "便利なツール".to_string(),
             価格: 5000,
         };
         items_map.insert(item1.id, item1);
         items_map.insert(item2.id, item2);

         Self { items: Arc::new(Mutex::new(items_map)) }
     }

     #[allow(dead_code)]
     pub fn get_sample_item_ids(&self) -> Vec<商品ID> {
         self.items.lock().unwrap().keys().cloned().collect()
     }
}

impl 商品Repository for InMemory商品Repository {
    fn find_by_id(&self, id: &商品ID) -> Result<Option<商品>, DomainError> {
        let items_map = self.items.lock().unwrap();
        Ok(items_map.get(id).cloned())
    }
}
*/

/* --- 古い注文リポジトリ実装をコメントアウト --- */
/*
#[derive(Clone, Default)]
pub struct InMemory注文Repository {
    orders: Arc<Mutex<HashMap<注文ID, 注文>>>,
}

 impl InMemory注文Repository {
    pub fn new() -> Self {
        Self { orders: Arc::new(Mutex::new(HashMap::new())) }
    }
}

impl 注文Repository for InMemory注文Repository {
    fn save(&self, order: &注文) -> Result<(), DomainError> {
        let mut orders_map = self.orders.lock().unwrap();
        println!("Saving order: {:?}", order); // 永続化処理のシミュレーション
        orders_map.insert(order.id, order.clone());
        Ok(())
    }

    fn find_by_id(&self, id: &注文ID) -> Result<Option<注文>, DomainError> {
        let orders_map = self.orders.lock().unwrap();
        println!("Finding order by id: {:?}", id);
        Ok(orders_map.get(id).cloned())
    }
}
*/

#[derive(Clone, Default)]
pub struct InMemoryプレゼント予約Repository {
    reservations: Arc<Mutex<HashMap<予約ID, プレゼント予約状態>>>,
}

impl InMemoryプレゼント予約Repository {
    pub fn new() -> Self {
        Self {
            reservations: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl プレゼント予約Repository for InMemoryプレゼント予約Repository {
    async fn save(&self, reservation: &プレゼント予約状態) -> Result<(), DomainError> {
        let mut reservations_map = self.reservations.lock().unwrap(); // Mutexをロック

        // 予約状態からIDを取得 (どの状態でも base.id でアクセスできる)
        let id = match reservation {
            プレゼント予約状態::予約受付済み(r) => r.base.id,
            プレゼント予約状態::発送準備中(r) => r.base.id,
            プレゼント予約状態::発送済み(r) => r.base.id,
            プレゼント予約状態::配送完了(r) => r.base.id,
            プレゼント予約状態::キャンセル済み(r) => r.base.id,
        };

        println!(
            "InMemory: Saving reservation {:?} with state: {:?}",
            id, reservation
        );
        reservations_map.insert(id, reservation.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &予約ID,
    ) -> Result<Option<プレゼント予約状態>, DomainError> {
        let reservations_map = self.reservations.lock().unwrap(); // Mutexをロック
        println!("InMemory: Finding reservation by id: {:?}", id);
        let found = reservations_map.get(id).cloned(); // 見つかったらクローンして返す
        Ok(found)
    }

    /// インメモリリポジトリは常に接続OKとする
    async fn check_db_connection(&self) -> Result<(), InfrastructureError> {
        println!("InMemory: Checking connection (always OK)");
        Ok(())
    }
}

// --- PostgreSQL リポジトリの実装 (ここから追加) ---

#[derive(Clone)]
pub struct PgRepository {
    pool: PgPool,
}

impl PgRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// データベースへの接続を確認する
    pub async fn check_db_connection(&self) -> Result<(), sqlx::Error> {
        // 最も単純なクエリを実行して接続を試みる
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map(|_| ()) // 成功時は () を返す
                         // エラー時は sqlx::Error がそのまま返る
    }
}

#[async_trait]
impl プレゼント予約Repository for PgRepository {
    async fn save(
        &self, reservation_state: &プレゼント予約状態
    ) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            eprintln!("DB Error: Failed to begin transaction: {}", e);
            // TODO: より適切なエラーマッピング (例: InfrastructureError::DatabaseError(e))
            DomainError::予約NotFound(予約ID::new()) // 仮のエラー
        })?;

        match reservation_state {
            プレゼント予約状態::予約受付済み(r) => {
                let base = &r.base;
                let reservation_id = *base.id.as_uuid();
                let requester_id = *base.依頼者id.as_uuid();
                let recipient_id = *base.届け先id.as_uuid();
                let anniversary_date = base.記念日.value; // NaiveDate
                let message = base.メッセージ内容.as_deref(); // Option<String> -> Option<&str>
                let wrapping_type = format!("{:?}", base.ラッピング); // Enum -> String (例: "標準")
                let desired_delivery_date = base.配送希望日時; // Option<DateTime<Tz>>
                let total_amount = base.合計金額.value() as i32; // u32 -> i32 (DBは INTEGER)
                let payment_id = *base.支払いid.as_uuid();
                let status = "Received"; // 状態文字列

                // reservations テーブルへの UPSERT
                let result = sqlx::query!(
                    r#"
                    INSERT INTO reservations (
                        id, requester_id, recipient_id, anniversary_date, message,
                        wrapping_type, desired_delivery_date, total_amount, payment_id, status,
                        -- updated_at は DEFAULT NOW() または trigger で設定される想定
                        -- 他の状態固有カラムはデフォルト値またはNULLになる
                        preparation_staff_id, shipping_slip_number, delivery_completed_at,
                        cancellation_reason, cancelled_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NULL, NULL, NULL, NULL, NULL)
                    ON CONFLICT (id) DO UPDATE SET
                        requester_id = EXCLUDED.requester_id,
                        recipient_id = EXCLUDED.recipient_id,
                        anniversary_date = EXCLUDED.anniversary_date,
                        message = EXCLUDED.message,
                        wrapping_type = EXCLUDED.wrapping_type,
                        desired_delivery_date = EXCLUDED.desired_delivery_date,
                        total_amount = EXCLUDED.total_amount,
                        payment_id = EXCLUDED.payment_id,
                        status = EXCLUDED.status,
                        -- 他の状態固有カラムをリセット (NULL に設定)
                        preparation_staff_id = NULL,
                        shipping_slip_number = NULL,
                        delivery_completed_at = NULL,
                        cancellation_reason = NULL,
                        cancelled_at = NULL,
                        updated_at = NOW() -- updated_at を更新
                    "#,
                    reservation_id,
                    requester_id,
                    recipient_id,
                    anniversary_date,      // NaiveDate
                    message,               // Option<&str>
                    wrapping_type,         // String
                    desired_delivery_date, // Option<DateTime<Tz>>
                    total_amount,          // i32
                    payment_id,
                    status // &str
                )
                .execute(&mut *tx) // &mut *tx で可変参照を渡す
                .await;

                if let Err(e) = result {
                    eprintln!("DB Error: Failed to save reservation: {}", e);
                    tx.rollback().await.ok(); // ロールバック試行
                    return Err(DomainError::予約NotFound(base.id)); // 仮
                }

                // reservation_products テーブルのクリアと INSERT (ループ処理に変更)
                let product_ids: Vec<Uuid> =
                    base.手配商品リスト.iter().map(|id| *id.as_uuid()).collect();

                // 既存の関連を削除 (変更なし)
                let delete_result = sqlx::query!(
                    "DELETE FROM reservation_products WHERE reservation_id = $1",
                    reservation_id
                )
                .execute(&mut *tx)
                .await;

                if let Err(e) = delete_result {
                    eprintln!("DB Error: Failed to delete reservation products: {}", e);
                    tx.rollback().await.ok();
                    return Err(DomainError::予約NotFound(base.id)); // 仮
                }

                // 新しい関連を INSERT (ループ処理)
                for product_id in product_ids {
                    let insert_result = sqlx::query!(
                        "INSERT INTO reservation_products (reservation_id, product_id) VALUES ($1, $2)",
                        reservation_id,
                        product_id // 個別の Uuid を渡す
                    )
                    .execute(&mut *tx)
                    .await;

                    if let Err(e) = insert_result {
                        eprintln!(
                            "DB Error: Failed to insert reservation product {}: {}",
                            product_id, e
                        );
                        // エラー発生時はトランザクション全体が commit されずに rollback されるので、
                        // ここで明示的な rollback は必須ではない。
                        // エラーを返して処理を中断する。
                        return Err(DomainError::予約NotFound(base.id)); // 仮。より具体的なエラーが良い
                    }
                }
            }
            プレゼント予約状態::発送準備中(r) => {
                let base = &r.base;
                let reservation_id = *base.id.as_uuid();
                let preparation_staff_id = *r.梱包担当者id.as_uuid();
                let status = "Preparing";

                sqlx::query!(
                    r#"
                    UPDATE reservations SET
                        status = $1,
                        preparation_staff_id = $2,
                        shipping_slip_number = NULL, -- Reset other state columns
                        delivery_completed_at = NULL,
                        cancellation_reason = NULL,
                        cancelled_at = NULL,
                        updated_at = NOW()
                    WHERE id = $3
                    "#,
                    status,
                    preparation_staff_id,
                    reservation_id
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    eprintln!("DB Error: Failed to update reservation to Preparing: {}", e);
                    DomainError::予約NotFound(base.id) // 仮
                })?;
                // 商品リストの更新は不要 (状態遷移のみ)
            }
            プレゼント予約状態::発送済み(r) => {
                let base = &r.base;
                let reservation_id = *base.id.as_uuid();
                let shipping_slip_number = &r.配送伝票番号;
                let status = "Shipped";

                sqlx::query!(
                    r#"
                    UPDATE reservations SET
                        status = $1,
                        shipping_slip_number = $2,
                        preparation_staff_id = NULL, -- Reset other state columns
                        delivery_completed_at = NULL,
                        cancellation_reason = NULL,
                        cancelled_at = NULL,
                        updated_at = NOW()
                    WHERE id = $3
                    "#,
                    status,
                    shipping_slip_number,
                    reservation_id
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    eprintln!("DB Error: Failed to update reservation to Shipped: {}", e);
                    DomainError::予約NotFound(base.id) // 仮
                })?;
            }
            プレゼント予約状態::配送完了(r) => {
                let base = &r.base;
                let reservation_id = *base.id.as_uuid();
                let delivery_completed_at = r.配送完了日時; // DateTime<Tz>
                let status = "Delivered";

                sqlx::query!(
                    r#"
                    UPDATE reservations SET
                        status = $1,
                        delivery_completed_at = $2,
                        -- preparation_staff_id は Preparing 状態でのみ設定される想定
                        -- shipping_slip_number は Shipped 状態で設定済みのはず
                        cancellation_reason = NULL,
                        cancelled_at = NULL,
                        updated_at = NOW()
                    WHERE id = $3
                    "#,
                    status,
                    delivery_completed_at,
                    reservation_id
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    eprintln!("DB Error: Failed to update reservation to Delivered: {}", e);
                    DomainError::予約NotFound(base.id) // 仮
                })?;
            }
            プレゼント予約状態::キャンセル済み(r) => {
                let base = &r.base;
                let reservation_id = *base.id.as_uuid();
                let cancellation_reason = r.キャンセル理由.as_deref(); // Option<String> -> Option<&str>
                let cancelled_at = r.キャンセル日時; // Option<DateTime<Tz>>
                let status = "Cancelled";

                sqlx::query!(
                    r#"
                    UPDATE reservations SET
                        status = $1,
                        cancellation_reason = $2,
                        cancelled_at = $3,
                        -- preparation_staff_id, shipping_slip_number, delivery_completed_at は状態によって設定済みか NULL
                        updated_at = NOW()
                    WHERE id = $4
                    "#,
                    status,
                    cancellation_reason,
                    cancelled_at,
                    reservation_id
                )
                 .execute(&mut *tx)
                 .await
                  .map_err(|e| {
                     eprintln!("DB Error: Failed to update reservation to Cancelled: {}", e);
                     DomainError::予約NotFound(base.id) // 仮
                 })?;
            }
        }

        tx.commit().await.map_err(|e| {
            eprintln!("DB Error: Failed to commit transaction: {}", e);
            // TODO: より適切なエラーマッピング
            DomainError::予約NotFound(予約ID::new())
        })
    }

    async fn find_by_id(
        &self,
        id: &予約ID,
    ) -> Result<Option<プレゼント予約状態>, DomainError> {
        let reservation_uuid = *id.as_uuid();

        // reservations テーブルから基本情報を取得
        let maybe_reservation_record = sqlx::query!(
            r#"
            SELECT
                id, requester_id, recipient_id, anniversary_date, message,
                wrapping_type, desired_delivery_date, total_amount, payment_id,
                status,
                -- 状態固有カラム
                preparation_staff_id,
                shipping_slip_number,
                delivery_completed_at,
                cancellation_reason,
                cancelled_at
            FROM reservations
            WHERE id = $1
            "#,
            reservation_uuid
        )
        .fetch_optional(&self.pool) // 見つからない場合は None を返す
        .await
        .map_err(|e| {
            // TODO: エラーマッピング改善
            eprintln!(
                "DB Error: Failed to fetch reservation by id {}: {}",
                reservation_uuid, e
            );
            DomainError::予約NotFound(*id) // 仮
        })?;

        if let Some(record) = maybe_reservation_record {
            // レコードが見つかった場合

            // reservation_products テーブルから商品IDリストを取得
            let product_records = sqlx::query!(
                "SELECT product_id FROM reservation_products WHERE reservation_id = $1",
                reservation_uuid
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                eprintln!(
                    "DB Error: Failed to fetch reservation products for id {}: {}",
                    reservation_uuid, e
                );
                DomainError::予約NotFound(*id) // 仮
            })?;

            let product_ids: HashSet<商品ID> = product_records
                .into_iter()
                .map(|rec| 商品ID::from_uuid(rec.product_id))
                .collect();

            // status に基づいて プレゼント予約状態 を構築
            let state = match record.status.as_str() {
                "Received" => {
                    let wrapping_type = match record.wrapping_type.as_str() {
                        "なし" => ラッピング種類::なし,
                        "標準" => ラッピング種類::標準,
                        "特別" => ラッピング種類::特別,
                        _ => {
                            eprintln!(
                                "DB Error: Unknown wrapping type '{}' for id {}",
                                record.wrapping_type, reservation_uuid
                            );
                            return Err(DomainError::予約NotFound(*id)); // 不明な値はエラー
                        }
                    };
                    let total_amount = 金額::new(record.total_amount as u32).map_err(|e| {
                        eprintln!(
                            "DB Error: Invalid total amount {} for id {}: {:?}",
                            record.total_amount, reservation_uuid, e
                        );
                        DomainError::予約NotFound(*id) // 金額変換エラーもエラー扱い (仮)
                    })?;
                    let base = プレゼント予約ベース {
                        id: *id,
                        依頼者id: ユーザーID::from_uuid(record.requester_id),
                        届け先id: 届け先ID::from_uuid(record.recipient_id),
                        記念日: 記念日 {
                            value: record.anniversary_date,
                        },
                        メッセージ内容: record.message,
                        ラッピング: wrapping_type,
                        配送希望日時: record
                            .desired_delivery_date
                            .map(|dt| dt.with_timezone(&Tokyo)),
                        合計金額: total_amount,
                        支払いid: 支払いID::from_uuid(record.payment_id),
                        手配商品リスト: product_ids.clone(), // Clone product_ids for base
                    };
                    プレゼント予約状態::予約受付済み(予約受付済みプレゼント予約型 {
                        base,
                    })
                }
                "Preparing" => {
                    let wrapping_type = match record.wrapping_type.as_str() {
                        "なし" => ラッピング種類::なし,
                        "標準" => ラッピング種類::標準,
                        "特別" => ラッピング種類::特別,
                        _ => {
                            eprintln!(
                                "DB Error: Unknown wrapping type '{}' for id {}",
                                record.wrapping_type, reservation_uuid
                            );
                            return Err(DomainError::予約NotFound(*id)); // 不明な値はエラー
                        }
                    };
                    let total_amount = 金額::new(record.total_amount as u32).map_err(|e| {
                        eprintln!(
                            "DB Error: Invalid total amount {} for id {}: {:?}",
                            record.total_amount, reservation_uuid, e
                        );
                        DomainError::予約NotFound(*id) // 金額変換エラーもエラー扱い (仮)
                    })?;
                    let base = プレゼント予約ベース {
                        id: *id,
                        依頼者id: ユーザーID::from_uuid(record.requester_id),
                        届け先id: 届け先ID::from_uuid(record.recipient_id),
                        記念日: 記念日 {
                            value: record.anniversary_date,
                        },
                        メッセージ内容: record.message,
                        ラッピング: wrapping_type,
                        配送希望日時: record
                            .desired_delivery_date
                            .map(|dt| dt.with_timezone(&Tokyo)),
                        合計金額: total_amount,
                        支払いid: 支払いID::from_uuid(record.payment_id),
                        手配商品リスト: product_ids.clone(), // Clone product_ids for base
                    };

                    let preparation_staff_id = record.preparation_staff_id.ok_or_else(|| {
                        eprintln!(
                            "DB Error: preparation_staff_id is NULL for Preparing state, id {}",
                            reservation_uuid
                        );
                        DomainError::予約NotFound(*id)
                    })?;

                    プレゼント予約状態::発送準備中(
                        crate::domain::core::発送準備中プレゼント予約型 {
                            base,
                            梱包担当者id: ユーザーID::from_uuid(preparation_staff_id),
                        },
                    )
                }
                "Shipped" => {
                    let wrapping_type = match record.wrapping_type.as_str() {
                        "なし" => ラッピング種類::なし,
                        "標準" => ラッピング種類::標準,
                        "特別" => ラッピング種類::特別,
                        _ => {
                            eprintln!(
                                "DB Error: Unknown wrapping type '{}' for id {}",
                                record.wrapping_type, reservation_uuid
                            );
                            return Err(DomainError::予約NotFound(*id)); // 不明な値はエラー
                        }
                    };
                    let total_amount = 金額::new(record.total_amount as u32).map_err(|e| {
                        eprintln!(
                            "DB Error: Invalid total amount {} for id {}: {:?}",
                            record.total_amount, reservation_uuid, e
                        );
                        DomainError::予約NotFound(*id) // 金額変換エラーもエラー扱い (仮)
                    })?;
                    let base = プレゼント予約ベース {
                        id: *id,
                        依頼者id: ユーザーID::from_uuid(record.requester_id),
                        届け先id: 届け先ID::from_uuid(record.recipient_id),
                        記念日: 記念日 {
                            value: record.anniversary_date,
                        },
                        メッセージ内容: record.message,
                        ラッピング: wrapping_type,
                        配送希望日時: record
                            .desired_delivery_date
                            .map(|dt| dt.with_timezone(&Tokyo)),
                        合計金額: total_amount,
                        支払いid: 支払いID::from_uuid(record.payment_id),
                        手配商品リスト: product_ids.clone(), // Clone product_ids for base
                    };

                    let shipping_slip_number = record.shipping_slip_number.ok_or_else(|| {
                        eprintln!(
                            "DB Error: shipping_slip_number is NULL for Shipped state, id {}",
                            reservation_uuid
                        );
                        DomainError::予約NotFound(*id)
                    })?;

                    プレゼント予約状態::発送済み(
                        crate::domain::core::発送済みプレゼント予約型 {
                            base,
                            配送伝票番号: shipping_slip_number,
                        },
                    )
                }
                "Delivered" => {
                    let wrapping_type = match record.wrapping_type.as_str() {
                        "なし" => ラッピング種類::なし,
                        "標準" => ラッピング種類::標準,
                        "特別" => ラッピング種類::特別,
                        _ => {
                            eprintln!(
                                "DB Error: Unknown wrapping type '{}' for id {}",
                                record.wrapping_type, reservation_uuid
                            );
                            return Err(DomainError::予約NotFound(*id)); // 不明な値はエラー
                        }
                    };
                    let total_amount = 金額::new(record.total_amount as u32).map_err(|e| {
                        eprintln!(
                            "DB Error: Invalid total amount {} for id {}: {:?}",
                            record.total_amount, reservation_uuid, e
                        );
                        DomainError::予約NotFound(*id) // 金額変換エラーもエラー扱い (仮)
                    })?;
                    let base = プレゼント予約ベース {
                        id: *id,
                        依頼者id: ユーザーID::from_uuid(record.requester_id),
                        届け先id: 届け先ID::from_uuid(record.recipient_id),
                        記念日: 記念日 {
                            value: record.anniversary_date,
                        },
                        メッセージ内容: record.message,
                        ラッピング: wrapping_type,
                        配送希望日時: record
                            .desired_delivery_date
                            .map(|dt| dt.with_timezone(&Tokyo)),
                        合計金額: total_amount,
                        支払いid: 支払いID::from_uuid(record.payment_id),
                        手配商品リスト: product_ids.clone(), // Clone product_ids for base
                    };

                    let shipping_slip_number = record.shipping_slip_number.ok_or_else(|| {
                        eprintln!(
                            "DB Error: shipping_slip_number is NULL for Delivered state, id {}",
                            reservation_uuid
                        );
                        DomainError::予約NotFound(*id)
                    })?;
                    let delivery_completed_at = record.delivery_completed_at.ok_or_else(|| {
                        eprintln!(
                            "DB Error: delivery_completed_at is NULL for Delivered state, id {}",
                            reservation_uuid
                        );
                        DomainError::予約NotFound(*id)
                    })?;

                    プレゼント予約状態::配送完了(
                        crate::domain::core::配送完了プレゼント予約型 {
                            base,
                            配送伝票番号: shipping_slip_number,
                            配送完了日時: delivery_completed_at.with_timezone(&Tokyo),
                        },
                    )
                }
                "Cancelled" => {
                    let wrapping_type = match record.wrapping_type.as_str() {
                        "なし" => ラッピング種類::なし,
                        "標準" => ラッピング種類::標準,
                        "特別" => ラッピング種類::特別,
                        _ => {
                            eprintln!(
                                "DB Error: Unknown wrapping type '{}' for id {}",
                                record.wrapping_type, reservation_uuid
                            );
                            return Err(DomainError::予約NotFound(*id)); // 不明な値はエラー
                        }
                    };
                    let total_amount = 金額::new(record.total_amount as u32).map_err(|e| {
                        eprintln!(
                            "DB Error: Invalid total amount {} for id {}: {:?}",
                            record.total_amount, reservation_uuid, e
                        );
                        DomainError::予約NotFound(*id) // 金額変換エラーもエラー扱い (仮)
                    })?;
                    let base = プレゼント予約ベース {
                        id: *id,
                        依頼者id: ユーザーID::from_uuid(record.requester_id),
                        届け先id: 届け先ID::from_uuid(record.recipient_id),
                        記念日: 記念日 {
                            value: record.anniversary_date,
                        },
                        メッセージ内容: record.message,
                        ラッピング: wrapping_type,
                        配送希望日時: record
                            .desired_delivery_date
                            .map(|dt| dt.with_timezone(&Tokyo)),
                        合計金額: total_amount,
                        支払いid: 支払いID::from_uuid(record.payment_id),
                        手配商品リスト: product_ids.clone(), // Clone product_ids for base
                    };

                    let cancellation_reason = record.cancellation_reason; // Option<String>
                    let cancelled_at = record.cancelled_at.map(|dt| dt.with_timezone(&Tokyo)); // Option<DateTime<Tz>>

                    プレゼント予約状態::キャンセル済み(
                        crate::domain::core::キャンセル済みプレゼント予約型 {
                            base,
                            キャンセル理由: cancellation_reason,
                            キャンセル日時: cancelled_at,
                        },
                    )
                }
                unknown_status => {
                    eprintln!(
                        "DB Error: Unknown reservation status '{}' for id {}",
                        unknown_status, reservation_uuid
                    );
                    return Err(DomainError::予約NotFound(*id)); // 不明な状態はエラー
                }
            };
            Ok(Some(state))
        } else {
            // レコードが見つからなかった場合
            Ok(None)
        }
    }

    /// データベースへの接続を確認する (トレイト実装)
    async fn check_db_connection(&self) -> Result<(), InfrastructureError> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map(|_| ()) // 成功時は Ok(())
            .map_err(|e| {
                eprintln!("Health Check DB Error: {}", e); // エラーログ
                InfrastructureError::ConnectionError(e.to_string()) // sqlx::Error をラップ
            })
    }
}

// --- テスト ---
#[cfg(all(test, not(ci)))]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use sqlx::postgres::PgPoolOptions;
    use std::env; // tests モジュール内で use する
                  // use crate::domain::core::予約を受け付ける; // 関数ローカルで use

    // テスト用のヘルパー関数: DB接続プールを取得
    async fn setup_db_pool() -> sqlx::PgPool {
        // dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to create pool.")
    }

    // テスト用のヘルパー関数: ダミーの予約受付済み状態を作成
    fn create_dummy_received_reservation() -> プレゼント予約状態 {
        use crate::domain::core::予約を受け付ける; // 関数内で use する例
        let requester_id = ユーザーID::from_uuid(Uuid::new_v4());
        let recipient_id = 届け先ID::from_uuid(Uuid::new_v4());
        let anniversary = 記念日 {
            value: NaiveDate::from_ymd_opt(2024, 12, 25).unwrap(), // ここで NaiveDate を使う
        };
        let product_ids = HashSet::from([商品ID::from_uuid(Uuid::new_v4())]);
        let payment_id = 支払いID::from_uuid(Uuid::new_v4());
        let total_amount = 金額::new(5000).unwrap();

        let received = 予約を受け付ける(
            requester_id,
            recipient_id,
            anniversary,
            Some("テストメッセージ".to_string()),
            ラッピング種類::標準,
            None, // 配送希望日時なし
            product_ids,
            payment_id,
            total_amount,
        )
        .unwrap();
        プレゼント予約状態::予約受付済み(received)
    }

    #[tokio::test]
    async fn test_pg_save_and_find_by_id_received() {
        let pool = setup_db_pool().await;
        let repository = PgRepository::new(pool.clone()); // PgRepository をインスタンス化

        let reservation_state = create_dummy_received_reservation();
        let reservation_id = match &reservation_state {
            プレゼント予約状態::予約受付済み(r) => r.base.id,
            _ => panic!("Test setup error: Unexpected initial state"),
        };

        // テストデータのクリーンアップ (テスト開始前にも実行)
        sqlx::query!(
            "DELETE FROM reservation_products WHERE reservation_id = $1",
            reservation_id.as_uuid()
        )
        .execute(&pool)
        .await
        .expect("Failed to clean up test products data (before test)");
        sqlx::query!(
            "DELETE FROM reservations WHERE id = $1",
            reservation_id.as_uuid()
        )
        .execute(&pool)
        .await
        .expect("Failed to clean up test reservation data (before test)");

        // 保存処理のテスト
        let save_result = repository.save(&reservation_state).await;
        assert!(save_result.is_ok(), "save failed: {:?}", save_result.err());

        // 取得処理のテスト
        let found_reservation = repository.find_by_id(&reservation_id).await;
        assert!(
            found_reservation.is_ok(),
            "find_by_id failed: {:?}",
            found_reservation.err()
        );
        assert!(
            found_reservation.as_ref().unwrap().is_some(),
            "Reservation not found after save"
        );

        // 取得したデータが保存したデータと一致するか検証
        // 注意: DateTime<Tz> は DB 保存時にマイクロ秒が丸められる可能性があるため、完全一致しない場合がある
        // ここでは PartialEq を使って比較する
        assert_eq!(found_reservation.unwrap().unwrap(), reservation_state);

        // テスト後のデータクリーンアップ
        sqlx::query!(
            "DELETE FROM reservation_products WHERE reservation_id = $1",
            reservation_id.as_uuid()
        )
        .execute(&pool)
        .await
        .expect("Failed to clean up test products data (after test)");
        sqlx::query!(
            "DELETE FROM reservations WHERE id = $1",
            reservation_id.as_uuid()
        )
        .execute(&pool)
        .await
        .expect("Failed to clean up test reservation data (after test)");
    }

    // TODO: 他の状態 (発送準備中、発送済みなど) の save/find_by_id テストケースを追加
    // TODO: find_by_id で見つからない場合のテストケースを追加
    // TODO: save でエラーが発生する場合 (例: 重複IDなど) のテストケースを追加 (必要であれば)
}
