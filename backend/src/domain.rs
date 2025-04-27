#![allow(clippy::new_without_default)] // TODO: 各ID型に Default トレイトを実装する

// src/domain.rs

// Define internal module and re-export
pub mod core {
    use async_trait::async_trait;
    use chrono::{DateTime, NaiveDate}; // Utc を復活 -> 再度削除
    use chrono_tz::Tz;
    use std::collections::HashSet; // List<商品ID> の代わりに HashSet を使う例
    use thiserror::Error;
    use uuid::Uuid; // Tz を use

    // --- 値オブジェクト ---

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct 予約ID(Uuid);
    impl 予約ID {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
        #[allow(dead_code)]
        pub fn from_uuid(id: Uuid) -> Self {
            Self(id)
        }
        #[allow(dead_code)]
        pub fn as_uuid(&self) -> &Uuid {
            &self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ユーザーID(Uuid); // 依頼者ID
    impl ユーザーID {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
        #[allow(dead_code)]
        pub fn from_uuid(id: Uuid) -> Self {
            Self(id)
        }
        #[allow(dead_code)]
        pub fn as_uuid(&self) -> &Uuid {
            &self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct 届け先ID(Uuid);
    impl 届け先ID {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
        #[allow(dead_code)]
        pub fn from_uuid(id: Uuid) -> Self {
            Self(id)
        }
        #[allow(dead_code)]
        pub fn as_uuid(&self) -> &Uuid {
            &self.0
        }
    }

    // 商品IDはサンプルから流用、必要なら修正
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct 商品ID(Uuid);
    impl 商品ID {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
        #[allow(dead_code)]
        pub fn from_uuid(id: Uuid) -> Self {
            Self(id)
        }
        #[allow(dead_code)]
        pub fn as_uuid(&self) -> &Uuid {
            &self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct 支払いID(Uuid);
    impl 支払いID {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
        #[allow(dead_code)]
        pub fn from_uuid(id: Uuid) -> Self {
            Self(id)
        }
        #[allow(dead_code)]
        pub fn as_uuid(&self) -> &Uuid {
            &self.0
        }
    }

    // 他の値オブジェクト（記念日、金額、メッセージ内容、ラッピングオプション、配送希望日時など）も必要に応じて追加

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct 記念日 {
        pub value: NaiveDate,
    } // String -> NaiveDate

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // 金額型 (現在は日本円のみを扱う)
    pub struct 金額(u32);
    impl 金額 {
        pub fn new(value: u32) -> Result<Self, DomainError> {
            if value == 0 {
                Err(DomainError::不正な金額エラー { value })
            } else {
                Ok(Self(value))
            }
        }
        pub fn value(&self) -> u32 {
            self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ラッピング種類 {
        なし,
        標準,
        特別,
    }

    // --- エンティティと状態 ---

    /// プレゼント予約の状態 (ADR 0003)
    #[derive(Debug, Clone, PartialEq)]
    pub enum プレゼント予約状態 {
        予約受付済み(予約受付済みプレゼント予約型),
        発送準備中(発送準備中プレゼント予約型),
        発送済み(発送済みプレゼント予約型),
        配送完了(配送完了プレゼント予約型),
        キャンセル済み(キャンセル済みプレゼント予約型),
    }

    /// 各状態に共通のデータ (トレイトや抽象クラスの代わり)
    #[derive(Debug, Clone, PartialEq)]
    pub struct プレゼント予約ベース {
        pub id: 予約ID,
        pub 依頼者id: ユーザーID,
        pub 届け先id: 届け先ID,
        pub 記念日: 記念日,
        pub メッセージ内容: Option<String>,
        pub ラッピング: ラッピング種類,
        pub 配送希望日時: Option<DateTime<Tz>>, // Tokyo -> Tz
        pub 合計金額: 金額,
        pub 支払いid: 支払いID,
        pub 手配商品リスト: HashSet<商品ID>, // どの状態でも持ちそうなのでベースに含める例
    }

    /// 予約受付済み状態のデータと振る舞い
    #[derive(Debug, Clone, PartialEq)]
    pub struct 予約受付済みプレゼント予約型 {
        pub base: プレゼント予約ベース,
        // この状態固有のデータがあれば追加
    }

    /// 発送準備中状態のデータと振る舞い
    #[derive(Debug, Clone, PartialEq)]
    pub struct 発送準備中プレゼント予約型 {
        pub base: プレゼント予約ベース,
        pub 梱包担当者id: ユーザーID, // 担当者のIDを追加
    }

    /// 発送済み状態のデータと振る舞い
    #[derive(Debug, Clone, PartialEq)]
    pub struct 発送済みプレゼント予約型 {
        pub base: プレゼント予約ベース,
        pub 配送伝票番号: String,
    }

    /// 配送完了状態のデータと振る舞い
    #[derive(Debug, Clone, PartialEq)]
    pub struct 配送完了プレゼント予約型 {
        pub base: プレゼント予約ベース,
        pub 配送伝票番号: String,
        pub 配送完了日時: DateTime<Tz>, // Tokyo -> Tz
    }

    /// キャンセル済み状態のデータと振る舞い
    #[derive(Debug, Clone, PartialEq)]
    pub struct キャンセル済みプレゼント予約型 {
        pub base: プレゼント予約ベース,
        pub キャンセル理由: Option<String>,
        pub キャンセル日時: Option<DateTime<Tz>>, // Tokyo -> Tz
    }

    // --- ドメインエラー ---
    #[derive(Error, Debug, PartialEq)]
    pub enum DomainError {
        #[error("プレゼント予約が見つかりません: ID={0:?}")]
        予約NotFound(予約ID),
        #[error("不正な状態遷移です: 現在の状態={current_state_type}")]
        不正な状態遷移 { current_state_type: String }, // どの型からの遷移が不正かを示す
        #[error("必須項目が不足しています: {field}")]
        必須項目不足 { field: String },
        #[error("予約に商品が含まれていません")]
        予約商品空エラー, // 新しいエラーを追加
        #[error("商品が見つかりません: ID={0:?}")]
        商品NotFound(商品ID), // これは商品ドメインのエラーかもしれない
        #[error("不正な金額が指定されました: value={value}")]
        不正な金額エラー { value: u32 },
        // 他に必要なドメイン固有のエラーを追加
    }

    // --- インフラストラクチャエラー (リポジトリ操作のエラーを表現) ---
    #[derive(Error, Debug, PartialEq)]
    pub enum InfrastructureError {
        #[error("データベース操作エラー: {0}")]
        DatabaseError(String), // sqlx::Error などをラップすることを想定
        #[error("接続エラー: {0}")]
        ConnectionError(String),
        // 必要に応じて他のインフラエラーを追加
    }

    // --- ドメインサービス / ロジック関数 ---
    // 例: 予約を受け付ける関数
    #[allow(clippy::too_many_arguments)] // TODO: 引数が多いのでコマンドオブジェクト等でのリファクタリングを検討
    pub fn 予約を受け付ける(
        依頼者id: ユーザーID,
        届け先id: 届け先ID,
        記念日: 記念日,
        メッセージ内容: Option<String>,
        ラッピング: ラッピング種類,
        配送希望日時: Option<DateTime<Tz>>, // Tokyo -> Tz
        商品idリスト: HashSet<商品ID>,
        支払いid: 支払いID,
        合計金額: 金額,
    ) -> Result<予約受付済みプレゼント予約型, DomainError> {
        if 商品idリスト.is_empty() {
            return Err(DomainError::予約商品空エラー);
        }
        let 予約id = 予約ID::new();
        let base = プレゼント予約ベース {
            id: 予約id,
            依頼者id,
            届け先id,
            記念日,
            メッセージ内容,
            ラッピング,
            配送希望日時,
            合計金額,
            支払いid,
            手配商品リスト: 商品idリスト,
        };
        Ok(予約受付済みプレゼント予約型 { base })
    }

    // 例: 状態遷移の関数
    impl 予約受付済みプレゼント予約型 {
        pub fn 発送準備を開始する(
            self,
            梱包担当者id: ユーザーID,
        ) -> Result<発送準備中プレゼント予約型, DomainError> {
            Ok(発送準備中プレゼント予約型 {
                base: self.base,
                梱包担当者id, // 受け取ったIDを設定
            })
        }
        pub fn 予約をキャンセルする(
            self,
            理由: Option<String>,
            日時: Option<DateTime<Tz>>, // Tokyo -> Tz
        ) -> Result<キャンセル済みプレゼント予約型, DomainError> {
            // 予約受付済み -> キャンセル済み は許可される
            Ok(キャンセル済みプレゼント予約型 {
                base: self.base,
                キャンセル理由: 理由,
                キャンセル日時: 日時,
            })
        }
    }

    impl 発送準備中プレゼント予約型 {
        pub fn 発送を完了する(
            self,
            配送伝票番号: String,
        ) -> Result<発送済みプレゼント予約型, DomainError> {
            Ok(発送済みプレゼント予約型 {
                base: self.base,
                配送伝票番号,
            })
        }
        pub fn 予約をキャンセルする(
            self,
            理由: Option<String>,
            日時: Option<DateTime<Tz>>, // Tokyo -> Tz
        ) -> Result<キャンセル済みプレゼント予約型, DomainError> {
            // 発送準備中 -> キャンセル済み は許可される
            Ok(キャンセル済みプレゼント予約型 {
                base: self.base,
                キャンセル理由: 理由,
                キャンセル日時: 日時,
            })
        }
    }

    impl 発送済みプレゼント予約型 {
        pub fn 配送完了を記録する(
            self,
            記録日時: DateTime<Tz>, // Tokyo -> Tz
        ) -> Result<配送完了プレゼント予約型, DomainError> {
            Ok(配送完了プレゼント予約型 {
                base: self.base,
                配送伝票番号: self.配送伝票番号,
                配送完了日時: 記録日時,
            })
        }
        // 他の遷移メソッド (例: キャンセルは不可？)
    }

    // 他の状態遷移関数も同様に定義

    // --- リポジトリインターフェース (トレイト) ---
    #[cfg_attr(test, mockall::automock)]
    #[async_trait]
    pub trait プレゼント予約Repository: Send + Sync {
        async fn save(&self, reservation: &プレゼント予約状態) -> Result<(), DomainError>;
        async fn find_by_id(
            &self,
            id: &予約ID,
        ) -> Result<Option<プレゼント予約状態>, DomainError>;
        // 必要に応じて他の検索メソッドを追加 (例: find_by_user_id)

        /// リポジトリ（主にDB）への接続性を確認する
        async fn check_db_connection(&self) -> Result<(), InfrastructureError>; // これで InfrastructureError が見つかるはず
    }

    // 商品リポジトリはシンプル化のため一旦コメントアウト or 削除しても良い
    // #[cfg_attr(test, mockall::automock)]
    // pub trait 商品Repository: Send + Sync {
    //     fn find_by_id(&self, id: &商品ID) -> Result<Option<商品>, DomainError>;
    // }
} // End of pub mod core

// Re-export all public items from the core module
pub use core::*;

// --- Domain Tests ---
#[cfg(test)]
mod tests {
    use super::core::*; // Use items from the inner core module now
    use chrono::{NaiveDate, TimeZone, Utc}; // TimeZone, Utc を削除
    use chrono_tz::Asia::Tokyo;
    use std::collections::HashSet;
    use uuid::Uuid; // Tokyo を use -> 削除

    // --- 値オブジェクトのテスト ---

    #[test]
    fn test_id_value_objects_creation_and_access() {
        // new() で生成
        let yoyaku_id = 予約ID::new();
        let user_id = ユーザーID::new();
        let todokesaki_id = 届け先ID::new();
        let shohin_id = 商品ID::new();
        let shiharai_id = 支払いID::new();

        // as_uuid() でアクセスできるか
        assert_ne!(yoyaku_id.as_uuid().to_string(), "");
        assert_ne!(user_id.as_uuid().to_string(), "");
        assert_ne!(todokesaki_id.as_uuid().to_string(), "");
        assert_ne!(shohin_id.as_uuid().to_string(), "");
        assert_ne!(shiharai_id.as_uuid().to_string(), "");

        // from_uuid() で生成
        let specific_uuid = Uuid::new_v4();
        let yoyaku_id_from = 予約ID::from_uuid(specific_uuid);
        let user_id_from = ユーザーID::from_uuid(specific_uuid);
        let todokesaki_id_from = 届け先ID::from_uuid(specific_uuid);
        let shohin_id_from = 商品ID::from_uuid(specific_uuid);
        let shiharai_id_from = 支払いID::from_uuid(specific_uuid);

        // 渡したUUIDと同じか
        assert_eq!(yoyaku_id_from.as_uuid(), &specific_uuid);
        assert_eq!(user_id_from.as_uuid(), &specific_uuid);
        assert_eq!(todokesaki_id_from.as_uuid(), &specific_uuid);
        assert_eq!(shohin_id_from.as_uuid(), &specific_uuid);
        assert_eq!(shiharai_id_from.as_uuid(), &specific_uuid);
    }

    #[test]
    fn test_記念日_creation() {
        let date_opt = NaiveDate::from_ymd_opt(2025, 12, 24);
        assert!(date_opt.is_some());
        let expected_date = date_opt.unwrap();
        let kinenbi = 記念日 {
            value: expected_date,
        };
        assert_eq!(kinenbi.value, expected_date);
    }

    #[test]
    fn test_金額_creation() {
        let 金額_value = 5000u32;
        let 金額_result = 金額::new(金額_value);
        assert!(金額_result.is_ok());
        let 金額_obj = 金額_result.unwrap();
        assert_eq!(金額_obj.value(), 金額_value);
    }

    #[test]
    fn test_金額_creation_fail_zero() {
        let 金額_value = 0u32;
        let 金額_result = 金額::new(金額_value);
        assert!(金額_result.is_err());
        assert_eq!(
            金額_result.err(),
            Some(DomainError::不正な金額エラー { value: 0 })
        );
    }

    // --- 予約受付テスト ---

    #[test]
    fn test_予約を受け付ける_success() {
        let 依頼者 = ユーザーID::new();
        let 届け先 = 届け先ID::new();
        let 記念日_obj = 記念日 {
            value: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        };
        let message = Some("お誕生日おめでとう！".to_string());
        let wrapping = ラッピング種類::特別;
        let delivery_time = Some(Tokyo.with_ymd_and_hms(2025, 1, 10, 10, 0, 0).unwrap()); // これは Tz::Asia__Tokyo::Tokyo で呼び出すのが正しい
        let 商品1 = 商品ID::new();
        let mut 商品リスト = HashSet::new();
        商品リスト.insert(商品1);
        let 支払い = 支払いID::new();
        let 金額_obj = 金額::new(10000).unwrap();

        let result = 予約を受け付ける(
            依頼者,
            届け先,
            記念日_obj.clone(),
            message.clone(),
            wrapping,
            delivery_time,
            商品リスト.clone(),
            支払い,
            金額_obj,
        );

        assert!(result.is_ok());
        let reservation = result.unwrap();
        assert_eq!(reservation.base.依頼者id, 依頼者);
        assert_eq!(reservation.base.届け先id, 届け先);
        assert_eq!(reservation.base.記念日, 記念日_obj);
        assert_eq!(reservation.base.メッセージ内容, message);
        assert_eq!(reservation.base.ラッピング, wrapping);
        assert_eq!(reservation.base.配送希望日時, delivery_time);
        assert_eq!(reservation.base.手配商品リスト, 商品リスト);
        assert_eq!(reservation.base.支払いid, 支払い);
        assert_eq!(reservation.base.合計金額, 金額_obj);
        assert_ne!(reservation.base.id.as_uuid().to_string(), "");
    }

    #[test]
    fn test_予約を受け付ける_fail_empty_items() {
        let 依頼者 = ユーザーID::new();
        let 届け先 = 届け先ID::new();
        let 記念日_obj = 記念日 {
            value: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        };
        let message = None;
        let wrapping = ラッピング種類::なし;
        let delivery_time = None;
        let 商品リスト = HashSet::new();
        let 支払い = 支払いID::new();
        let 金額_obj = 金額::new(1).unwrap();

        let result = 予約を受け付ける(
            依頼者,
            届け先,
            記念日_obj.clone(),
            message,
            wrapping,
            delivery_time,
            商品リスト.clone(),
            支払い,
            金額_obj,
        );

        assert!(result.is_err());
        assert_eq!(result.err(), Some(DomainError::予約商品空エラー));
    }

    // --- 状態遷移テスト ---

    #[test]
    fn test_予約受付済みから発送準備中へ遷移_success() {
        let 記念日_obj = 記念日 {
            value: NaiveDate::from_ymd_opt(2025, 2, 14).unwrap(),
        };
        let reservation_received_result = 予約を受け付ける(
            ユーザーID::new(),
            届け先ID::new(),
            記念日_obj.clone(),
            Some("Happy Valentine!".to_string()),
            ラッピング種類::標準,
            None,
            create_dummy_product_ids(),
            支払いID::new(),
            金額::new(5000).unwrap(),
        );
        assert!(reservation_received_result.is_ok());
        let reservation_received = reservation_received_result.unwrap();
        let original_base = reservation_received.base.clone();
        let 梱包担当者 = ユーザーID::new(); // 梱包担当者IDを定義
        let result = reservation_received.発送準備を開始する(梱包担当者); // 引数に梱包担当者IDを渡す
        assert!(result.is_ok());
        let reservation_preparing = result.unwrap();
        assert_eq!(reservation_preparing.base, original_base);
        assert_eq!(reservation_preparing.梱包担当者id, 梱包担当者); // 梱包担当者IDのアサーション
        assert!(matches!(
            プレゼント予約状態::発送準備中(reservation_preparing),
            プレゼント予約状態::発送準備中(_)
        ));
    }

    #[test]
    fn test_発送準備中から発送済みへ遷移_success() {
        let 記念日_obj = 記念日 {
            value: NaiveDate::from_ymd_opt(2025, 3, 14).unwrap(),
        };
        let reservation_received = 予約を受け付ける(
            ユーザーID::new(),
            届け先ID::new(),
            記念日_obj.clone(),
            None,
            ラッピング種類::なし,
            None,
            create_dummy_product_ids(),
            支払いID::new(),
            金額::new(8000).unwrap(),
        )
        .unwrap();
        let 梱包担当者 = ユーザーID::new(); // ダミーの梱包担当者ID
        let reservation_preparing_result =
            reservation_received.発送準備を開始する(梱包担当者); // IDを渡す
        assert!(reservation_preparing_result.is_ok());
        let reservation_preparing = reservation_preparing_result.unwrap();
        let original_base = reservation_preparing.base.clone(); // base は変わらないはず
        assert_eq!(reservation_preparing.梱包担当者id, 梱包担当者); // 梱包担当者IDが設定されているか確認

        let slip_number = "1234-5678-9012".to_string();
        let result = reservation_preparing.発送を完了する(slip_number.clone());
        assert!(result.is_ok());
        let reservation_shipped = result.unwrap();
        assert_eq!(reservation_shipped.base, original_base); // base は引き継がれる
        assert_eq!(reservation_shipped.配送伝票番号, slip_number);
        assert!(matches!(
            プレゼント予約状態::発送済み(reservation_shipped),
            プレゼント予約状態::発送済み(_)
        ));
    }

    #[test]
    fn test_発送済みから配送完了へ遷移_success() {
        let 記念日_obj = 記念日 {
            value: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
        };
        let reservation_received = 予約を受け付ける(
            ユーザーID::new(),
            届け先ID::new(),
            記念日_obj.clone(),
            Some("Test".to_string()),
            ラッピング種類::標準,
            None,
            create_dummy_product_ids(),
            支払いID::new(),
            金額::new(3000).unwrap(),
        )
        .unwrap();
        let 梱包担当者 = ユーザーID::new(); // 梱包担当者IDを追加
        let reservation_preparing = reservation_received.発送準備を開始する(梱包担当者).unwrap(); // 引数を追加
        let slip_number = "9876-5432-1098".to_string();
        let reservation_shipped = reservation_preparing
            .発送を完了する(slip_number.clone())
            .unwrap();
        let original_base = reservation_shipped.base.clone();
        let completion_time = Utc::now().with_timezone(&Tokyo); // Utc::now() を経由
        let result = reservation_shipped.配送完了を記録する(completion_time.clone());
        assert!(result.is_ok());
        let reservation_delivered = result.unwrap();
        assert_eq!(reservation_delivered.base, original_base);
        assert_eq!(reservation_delivered.配送伝票番号, slip_number);
        assert_eq!(reservation_delivered.配送完了日時, completion_time);
        assert!(matches!(
            プレゼント予約状態::配送完了(reservation_delivered),
            プレゼント予約状態::配送完了(_)
        ));
    }

    #[test]
    fn test_予約受付済みからキャンセル済みへ遷移_success() {
        let 記念日_obj = 記念日 {
            value: NaiveDate::from_ymd_opt(2025, 5, 1).unwrap(),
        };
        let reservation_received = 予約を受け付ける(
            ユーザーID::new(),
            届け先ID::new(),
            記念日_obj.clone(),
            None,
            ラッピング種類::なし,
            None,
            create_dummy_product_ids(),
            支払いID::new(),
            金額::new(1000).unwrap(),
        )
        .unwrap();
        let original_base = reservation_received.base.clone();
        let reason = Some("顧客都合".to_string());
        let time = Some(Utc::now().with_timezone(&Tokyo)); // Utc::now() を経由
        let result =
            reservation_received.予約をキャンセルする(reason.clone(), time.clone());
        assert!(result.is_ok());
        let reservation_cancelled = result.unwrap();
        assert_eq!(reservation_cancelled.base, original_base);
        assert_eq!(reservation_cancelled.キャンセル理由, reason);
        assert_eq!(reservation_cancelled.キャンセル日時, time);
        assert!(matches!(
            プレゼント予約状態::キャンセル済み(reservation_cancelled),
            プレゼント予約状態::キャンセル済み(_)
        ));
    }

    #[test]
    fn test_発送準備中からキャンセル済みへ遷移_success() {
        let 記念日_obj = 記念日 {
            value: NaiveDate::from_ymd_opt(2025, 5, 5).unwrap(),
        };
        let reservation_received = 予約を受け付ける(
            ユーザーID::new(),
            届け先ID::new(),
            記念日_obj.clone(),
            Some("Msg".to_string()),
            ラッピング種類::標準,
            None,
            create_dummy_product_ids(),
            支払いID::new(),
            金額::new(2000).unwrap(),
        )
        .unwrap();
        let 梱包担当者 = ユーザーID::new(); // ダミーの梱包担当者ID
        let reservation_preparing_result =
            reservation_received.発送準備を開始する(梱包担当者); // IDを渡す
        assert!(reservation_preparing_result.is_ok());
        let reservation_preparing = reservation_preparing_result.unwrap();
        let original_base = reservation_preparing.base.clone(); // base は変わらないはず
        assert_eq!(reservation_preparing.梱包担当者id, 梱包担当者); // 梱包担当者IDが設定されているか確認

        let reason = None;
        let time = Some(Utc::now().with_timezone(&Tokyo)); // Utc::now() を経由
        let result =
            reservation_preparing.予約をキャンセルする(reason.clone(), time.clone());
        assert!(result.is_ok());
        let reservation_cancelled = result.unwrap();
        assert_eq!(reservation_cancelled.base, original_base); // base は引き継がれる
        assert_eq!(reservation_cancelled.キャンセル理由, reason);
        assert_eq!(reservation_cancelled.キャンセル日時, time);
        assert!(matches!(
            プレゼント予約状態::キャンセル済み(reservation_cancelled),
            プレゼント予約状態::キャンセル済み(_)
        ));
    }

    // Helper function to create a HashSet<商品ID> for tests
    fn create_dummy_product_ids() -> HashSet<商品ID> {
        vec![商品ID::new()].into_iter().collect()
    }
}
