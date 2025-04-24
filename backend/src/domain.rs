// src/domain.rs

// Define internal module and re-export
pub mod core {
    use uuid::Uuid;
    use thiserror::Error;
    use std::collections::HashSet; // List<商品ID> の代わりに HashSet を使う例

    // --- 値オブジェクト ---

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct 予約ID(Uuid);
    impl 予約ID {
        pub fn new() -> Self { Self(Uuid::new_v4()) }
        #[allow(dead_code)] pub fn from_uuid(id: Uuid) -> Self { Self(id) }
        #[allow(dead_code)] pub fn as_uuid(&self) -> &Uuid { &self.0 }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ユーザーID(Uuid); // 依頼者ID
    impl ユーザーID {
        pub fn new() -> Self { Self(Uuid::new_v4()) }
        #[allow(dead_code)] pub fn from_uuid(id: Uuid) -> Self { Self(id) }
        #[allow(dead_code)] pub fn as_uuid(&self) -> &Uuid { &self.0 }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct 届け先ID(Uuid);
    impl 届け先ID {
        pub fn new() -> Self { Self(Uuid::new_v4()) }
        #[allow(dead_code)] pub fn from_uuid(id: Uuid) -> Self { Self(id) }
        #[allow(dead_code)] pub fn as_uuid(&self) -> &Uuid { &self.0 }
    }

    // 商品IDはサンプルから流用、必要なら修正
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct 商品ID(Uuid);
    impl 商品ID {
        pub fn new() -> Self { Self(Uuid::new_v4()) }
        #[allow(dead_code)] pub fn from_uuid(id: Uuid) -> Self { Self(id) }
        #[allow(dead_code)] pub fn as_uuid(&self) -> &Uuid { &self.0 }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct 支払いID(Uuid);
    impl 支払いID {
        pub fn new() -> Self { Self(Uuid::new_v4()) }
        #[allow(dead_code)] pub fn from_uuid(id: Uuid) -> Self { Self(id) }
        #[allow(dead_code)] pub fn as_uuid(&self) -> &Uuid { &self.0 }
    }

    // 他の値オブジェクト（記念日、金額、メッセージ内容、ラッピングオプション、配送希望日時など）も必要に応じて追加

    #[derive(Debug, Clone, PartialEq)] // 仮: chrono::NaiveDate などを使うべき
    pub struct 記念日 { pub value: String } // 通常のstructに変更

    #[derive(Debug, Clone, Copy, PartialEq, Eq)] // 仮: 通貨も考慮すべき
    pub struct 金額(u32);
    impl 金額 {
        pub fn new(value: u32) -> Self { // 公開されたコンストラクタ関数を追加
            // ここでバリデーションを行うことも可能 (例: value >= 0)
            Self(value)
        }
        pub fn value(&self) -> u32 { self.0 } // アクセサメソッド
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
        // pub メッセージ内容: String, // 必要に応じて追加
        // pub ラッピングオプション: ...,
        // pub 配送希望日時: ...,
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
        // pub 梱包担当者id: ユーザーID, // 例
    }

    /// 発送済み状態のデータと振る舞い
    #[derive(Debug, Clone, PartialEq)]
    pub struct 発送済みプレゼント予約型 {
        pub base: プレゼント予約ベース,
        pub 配送伝票番号: String, // この状態固有のデータ
    }

    /// 配送完了状態のデータと振る舞い
    #[derive(Debug, Clone, PartialEq)]
    pub struct 配送完了プレゼント予約型 {
        pub base: プレゼント予約ベース,
        pub 配送伝票番号: String,
        // pub 配送完了日時: chrono::DateTime<chrono::Utc>, // 例
    }

    /// キャンセル済み状態のデータと振る舞い
    #[derive(Debug, Clone, PartialEq)]
    pub struct キャンセル済みプレゼント予約型 {
        pub base: プレゼント予約ベース,
        // pub キャンセル理由: String, // 例
        // pub キャンセル日時: chrono::DateTime<chrono::Utc>, // 例
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
        // 他に必要なドメイン固有のエラーを追加
    }

    // --- ドメインサービス / ロジック関数 ---
    // 例: 予約を受け付ける関数
    pub fn 予約を受け付ける(
        依頼者id: ユーザーID,
        届け先id: 届け先ID,
        記念日: 記念日,
        商品idリスト: HashSet<商品ID>,
        支払いid: 支払いID, // 引数追加
        合計金額: 金額,     // 引数追加
    ) -> Result<予約受付済みプレゼント予約型, DomainError> {
        // バリデーション: 商品リストが空でないか
        if 商品idリスト.is_empty() {
            return Err(DomainError::予約商品空エラー);
        }

        // 新しい予約IDを生成
        let 予約id = 予約ID::new();

        // プレゼント予約ベースを作成
        let base = プレゼント予約ベース {
            id: 予約id,
            依頼者id,
            届け先id,
            記念日,
            合計金額,
            支払いid,
            手配商品リスト: 商品idリスト,
        };

        // 予約受付済みプレゼント予約型を作成して返す
        Ok(予約受付済みプレゼント予約型 { base })
    }

    // 例: 状態遷移の関数 (シグネチャのみ)
    impl 予約受付済みプレゼント予約型 {
        pub fn 発送準備を開始する(self) -> Result<発送準備中プレゼント予約型, DomainError> {
            // 状態遷移ロジック
            unimplemented!()
        }
        pub fn 予約をキャンセルする(self) -> Result<キャンセル済みプレゼント予約型, DomainError> {
            // 状態遷移ロジック
            unimplemented!()
        }
    }
    // 他の状態遷移関数も同様に定義 (発送準備中 -> 発送済み など)


    // --- リポジトリインターフェース (トレイト) ---
    // #[cfg_attr(test, mockall::automock)] // 必要ならmockallを有効化
    pub trait プレゼント予約Repository: Send + Sync {
        fn save(&self, reservation: &プレゼント予約状態) -> Result<(), DomainError>;
        fn find_by_id(&self, id: &予約ID) -> Result<Option<プレゼント予約状態>, DomainError>;
        // 必要に応じて他の検索メソッドを追加 (例: find_by_user_id)
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
    use std::collections::HashSet;
    use uuid::Uuid;

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
        let date_str = "2025-12-24".to_string();
        let kinenbi = 記念日 { value: date_str.clone() };
        assert_eq!(kinenbi.value, date_str);
        // TODO: chrono などを使って日付としての妥当性検証を追加する
    }

    #[test]
    fn test_金額_creation() {
        let kingaku_val = 5000u32;
        let kingaku = 金額::new(kingaku_val); // new() を使って生成
        assert_eq!(kingaku.value(), kingaku_val); // アクセサメソッドを使用
        // TODO: 通貨やマイナス値の考慮など、より詳細なテストを追加する
    }

    // --- 予約受付テスト ---

    #[test]
    fn test_予約を受け付ける_success() {
        let 依頼者 = ユーザーID::new();
        let 届け先 = 届け先ID::new();
        let 記念日_obj = 記念日 { value: "2025-01-01".to_string() };
        let 商品1 = 商品ID::new();
        let mut 商品リスト = HashSet::new();
        商品リスト.insert(商品1);
        let 支払い = 支払いID::new();
        let 金額_obj = 金額::new(10000);

        let result = 予約を受け付ける(
            依頼者,
            届け先,
            記念日_obj.clone(),
            商品リスト.clone(),
            支払い,
            金額_obj,
        );

        assert!(result.is_ok());
        let reservation = result.unwrap();

        assert_eq!(reservation.base.依頼者id, 依頼者);
        assert_eq!(reservation.base.届け先id, 届け先);
        assert_eq!(reservation.base.記念日, 記念日_obj);
        assert_eq!(reservation.base.手配商品リスト, 商品リスト);
        assert_eq!(reservation.base.支払いid, 支払い);
        assert_eq!(reservation.base.合計金額, 金額_obj);
        assert_ne!(reservation.base.id.as_uuid().to_string(), ""); // IDが生成されているか
    }

    #[test]
    fn test_予約を受け付ける_fail_empty_items() {
        let 依頼者 = ユーザーID::new();
        let 届け先 = 届け先ID::new();
        let 記念日_obj = 記念日 { value: "2025-01-01".to_string() };
        let 商品リスト = HashSet::new(); // 空のリスト
        let 支払い = 支払いID::new();
        let 金額_obj = 金額::new(0);

        let result = 予約を受け付ける(
            依頼者,
            届け先,
            記念日_obj.clone(),
            商品リスト.clone(),
            支払い,
            金額_obj,
        );

        assert!(result.is_err());
        assert_eq!(result.err(), Some(DomainError::予約商品空エラー));
    }

    // --- 他のドメインロジックや状態遷移のプレースホルダーテスト ---

    #[test]
    fn test_initial_state_creation_placeholder() {
        // let result = 予約を受け付ける(...);
        // assert!(result.is_ok());
        // let reservation = result.unwrap();
        // assert!(matches!(reservation.base.状態, プレゼント予約状態::予約受付済み(_)));
        assert!(true); // Placeholder test
    }

     #[test]
     fn test_state_transition_placeholder() {
         // let initial_state = 予約受付済みプレゼント予約型 { ... };
         // let result = initial_state.発送準備を開始する();
         // assert!(result.is_ok());
         // let next_state = result.unwrap();
         // assert!(matches!(next_state.base.状態, プレゼント予約状態::発送準備中(_)));
         assert!(true); // Placeholder test
     }

    // 他のテストケース (エラーケース、境界値など) も追加していく
} 