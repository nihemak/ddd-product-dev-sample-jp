use anyhow::Result; // main関数でのエラーハンドリング簡略化のため
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// --- Domain Layer ---
mod domain {
    use thiserror::Error;
    use uuid::Uuid;

    // --- 値オブジェクト ---
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct 注文ID(Uuid);

    impl 注文ID {
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

    #[derive(Debug, Clone)]
    pub struct 商品 {
        pub id: 商品ID,
        #[allow(dead_code)]
        pub 名前: String,
        pub 価格: u32,
    }

    // --- エンティティ ---
    #[derive(Debug, Clone, PartialEq)]
    pub enum 注文状態 {
        受付済み,
        発送準備中,
        発送済み,
        キャンセル済み,
    }

    #[derive(Debug, Clone)]
    pub struct 注文 {
        pub id: 注文ID,
         #[allow(dead_code)]
        pub 商品リスト: Vec<商品ID>,
         #[allow(dead_code)]
        pub 合計金額: u32,
        pub 状態: 注文状態,
    }

    // --- ドメインエラー ---
    #[derive(Error, Debug, PartialEq)]
    pub enum DomainError {
        #[error("注文に商品が含まれていません")]
        注文商品空エラー,
        #[error("指定された状態遷移は許可されていません: 現在の状態={current:?}, 要求された状態={requested:?}")]
        不正な状態遷移エラー {
            current: 注文状態,
            requested: 注文状態,
        },
        #[error("注文が見つかりません: ID={0:?}")]
        注文NotFound(注文ID),
         #[error("商品が見つかりません: ID={0:?}")]
        商品NotFound(商品ID),
        // 他のドメインルール違反に対応するエラー
    }

    // --- ドメインサービス / ロジック関数 ---

    // 商品リストから合計金額を計算する (純粋関数)
    pub fn calculate_total_price(
        item_ids: &[商品ID],
        get_item_price: impl Fn(&商品ID) -> Option<u32>, // 商品価格取得関数を外部から注入
    ) -> Result<u32, DomainError> {
        let mut total: u32 = 0;
        if item_ids.is_empty() {
            return Ok(0);
        }

        for item_id in item_ids {
            match get_item_price(item_id) {
                 Some(price) => {
                     total = total.checked_add(price)
                        .ok_or_else(|| DomainError::注文商品空エラー)?
                 },
                 None => return Err(DomainError::商品NotFound(item_id.clone())),
            }
        }
        Ok(total)
    }


    // 新しい注文を作成する関数 (コンストラクタ関数)
    pub fn create_order(
        item_ids: Vec<商品ID>,
        get_item_price: impl Fn(&商品ID) -> Option<u32>,
    ) -> Result<注文, DomainError> {
        if item_ids.is_empty() {
            return Err(DomainError::注文商品空エラー);
        }
        let total_price = calculate_total_price(&item_ids, get_item_price)?;

        Ok(注文 {
            id: 注文ID::new(),
            商品リスト: item_ids,
            合計金額: total_price,
            状態: 注文状態::受付済み,
        })
    }

    // 状態遷移関数 (ROP スタイル)
    // Resultを返すことで失敗の可能性を明示
    pub fn mark_as_preparing(order: 注文) -> Result<注文, DomainError> {
        match order.状態 {
            注文状態::受付済み => Ok(注文 {
                状態: 注文状態::発送準備中,
                ..order // 他のフィールドはそのまま
            }),
            _ => Err(DomainError::不正な状態遷移エラー {
                current: order.状態.clone(),
                requested: 注文状態::発送準備中,
            }),
        }
    }

    pub fn mark_as_shipped(order: 注文) -> Result<注文, DomainError> {
        match order.状態 {
            注文状態::発送準備中 => Ok(注文 {
                状態: 注文状態::発送済み,
                ..order
            }),
            _ => Err(DomainError::不正な状態遷移エラー {
                current: order.状態.clone(),
                requested: 注文状態::発送済み,
            }),
        }
    }

    #[allow(dead_code)]
    pub fn cancel_order(order: 注文) -> Result<注文, DomainError> {
         // 発送済みはキャンセルできない等のルールを追加可能
        match order.状態 {
            注文状態::発送済み => Err(DomainError::不正な状態遷移エラー{
                current: order.状態.clone(),
                requested: 注文状態::キャンセル済み,
            }),
            _ => Ok(注文 {
                状態: 注文状態::キャンセル済み,
                ..order
            })
        }
    }


    // --- リポジトリインターフェース (トレイト) ---
    // クレート境界を越えて実装を提供できるようにpubにする
    #[cfg_attr(test, mockall::automock)]
    pub trait 注文Repository: Send + Sync {
        fn save(&self, order: &注文) -> Result<(), DomainError>;
        fn find_by_id(&self, id: &注文ID) -> Result<Option<注文>, DomainError>;
    }

    #[cfg_attr(test, mockall::automock)]
    pub trait 商品Repository: Send + Sync {
        fn find_by_id(&self, id: &商品ID) -> Result<Option<商品>, DomainError>;
    }

    // --- Domain Tests ---
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashMap;

        // --- Helper functions/data for tests ---
        fn create_dummy_order(state: 注文状態) -> 注文 {
            注文 {
                id: 注文ID::new(),
                商品リスト: vec![商品ID::new()],
                合計金額: 1000,
                状態: state,
            }
        }

        fn create_dummy_item_id() -> 商品ID {
            商品ID::new()
        }

        // --- Tests for domain logic functions ---

        #[test]
        fn test_calculate_total_price_success() {
            let item_id1 = create_dummy_item_id();
            let item_id2 = create_dummy_item_id();
            let item_ids = vec![item_id1.clone(), item_id2.clone()];

            let mut prices = HashMap::new();
            prices.insert(item_id1, 1000);
            prices.insert(item_id2, 500);

            let get_price = |id: &商品ID| prices.get(id).cloned();

            assert_eq!(calculate_total_price(&item_ids, get_price), Ok(1500));
        }

        #[test]
        fn test_calculate_total_price_item_not_found() {
            let item_id1 = create_dummy_item_id();
            let item_id_unknown = create_dummy_item_id();
            let item_ids = vec![item_id1.clone(), item_id_unknown.clone()];

            let mut prices = HashMap::new();
            prices.insert(item_id1, 1000); // item_id_unknown は含まない

            let get_price = |id: &商品ID| prices.get(id).cloned();

            assert_eq!(
                calculate_total_price(&item_ids, get_price),
                Err(DomainError::商品NotFound(item_id_unknown))
            );
        }

        #[test]
        fn test_calculate_total_price_empty_list() {
             let item_ids: Vec<商品ID> = vec![];
             let get_price = |_id: &商品ID| -> Option<u32> { None };
             assert_eq!(calculate_total_price(&item_ids, get_price), Ok(0));
        }

        #[test]
        fn test_create_order_success() {
            let item_id1 = create_dummy_item_id();
            let item_ids = vec![item_id1.clone()];
            let get_price = |id: &商品ID| if id == &item_id1 { Some(2000) } else { None };

            let result = create_order(item_ids.clone(), get_price);
            assert!(result.is_ok());
            let order = result.unwrap();
            assert_eq!(order.商品リスト, item_ids);
            assert_eq!(order.合計金額, 2000);
            assert_eq!(order.状態, 注文状態::受付済み);
        }

        #[test]
        fn test_create_order_empty_items() {
            let item_ids: Vec<商品ID> = vec![];
            let get_price = |_id: &商品ID| None;
            assert_eq!(create_order(item_ids, get_price), Err(DomainError::注文商品空エラー));
        }

        #[test]
        fn test_create_order_item_not_found() {
            let item_id_unknown = create_dummy_item_id();
            let item_ids = vec![item_id_unknown.clone()];
             let get_price = |_id: &商品ID| None;
            assert_eq!(create_order(item_ids, get_price), Err(DomainError::商品NotFound(item_id_unknown)));
        }


        #[test]
        fn test_mark_as_preparing_success() {
            let order = create_dummy_order(注文状態::受付済み);
            let result = mark_as_preparing(order);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().状態, 注文状態::発送準備中);
        }

        #[test]
        fn test_mark_as_preparing_fail_invalid_state() {
            let order = create_dummy_order(注文状態::発送準備中);
            let current_state = order.状態.clone();
            let result = mark_as_preparing(order);
            assert_eq!(
                result,
                Err(DomainError::不正な状態遷移エラー {
                    current: current_state,
                    requested: 注文状態::発送準備中
                })
            );
             let order_shipped = create_dummy_order(注文状態::発送済み);
             let current_state_shipped = order_shipped.状態.clone();
             let result_shipped = mark_as_preparing(order_shipped);
             assert_eq!(
                result_shipped,
                Err(DomainError::不正な状態遷移エラー {
                    current: current_state_shipped,
                    requested: 注文状態::発送準備中
                })
            );
        }

        #[test]
        fn test_mark_as_shipped_success() {
            let order = create_dummy_order(注文状態::発送準備中);
            let result = mark_as_shipped(order);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().状態, 注文状態::発送済み);
        }

        #[test]
        fn test_mark_as_shipped_fail_invalid_state() {
            let order = create_dummy_order(注文状態::受付済み);
            let current_state = order.状態.clone();
            let result = mark_as_shipped(order);
             assert_eq!(
                result,
                Err(DomainError::不正な状態遷移エラー {
                    current: current_state,
                    requested: 注文状態::発送済み
                })
            );

             let order_shipped = create_dummy_order(注文状態::発送済み);
             let current_state_shipped = order_shipped.状態.clone();
             let result_shipped = mark_as_shipped(order_shipped);
             assert_eq!(
                result_shipped,
                Err(DomainError::不正な状態遷移エラー {
                    current: current_state_shipped,
                    requested: 注文状態::発送済み
                })
            );
        }

        #[test]
        fn test_cancel_order_success() {
             let order_received = create_dummy_order(注文状態::受付済み);
             assert_eq!(cancel_order(order_received).unwrap().状態, 注文状態::キャンセル済み);

             let order_preparing = create_dummy_order(注文状態::発送準備中);
             assert_eq!(cancel_order(order_preparing).unwrap().状態, 注文状態::キャンセル済み);
        }

        #[test]
        fn test_cancel_order_fail_when_shipped() {
            let order = create_dummy_order(注文状態::発送済み);
            let current_state = order.状態.clone();
            let result = cancel_order(order);
            assert_eq!(
                result,
                Err(DomainError::不正な状態遷移エラー {
                    current: current_state,
                    requested: 注文状態::キャンセル済み
                })
            );
        }
    }
}

// --- Application Layer ---
mod application {
    use super::domain::{
        self, create_order, mark_as_preparing, mark_as_shipped, 注文, 注文ID, 注文Repository, 注文状態, 商品, 商品ID, 商品Repository, DomainError
    };
    use std::sync::Arc; // Arcを使ってリポジトリの所有権を共有
    use thiserror::Error;

    // --- アプリケーションエラー ---
    // DomainErrorをラップしたり、アプリケーション固有のエラーを追加
    #[derive(Error, Debug, PartialEq)]
    pub enum ApplicationError {
        #[error("ドメインエラー: {0}")]
        Domain(#[from] DomainError), // DomainErrorからの変換を自動実装
        #[error("リポジトリ操作エラー: {0}")]
        Repository(String), // Infra層からの具体的なエラーメッセージなど
         #[allow(dead_code)]
         #[error("予期せぬエラー: {0}")]
        Unexpected(String),
    }

    // Result型エイリアス
    type AppResult<T> = Result<T, ApplicationError>;

    // --- ユースケース / ワークフロー ---
    pub struct 注文サービス {
        order_repo: Arc<dyn 注文Repository + Send + Sync>,
        item_repo: Arc<dyn 商品Repository + Send + Sync>,
    }

    impl 注文サービス {
        pub fn new(
            order_repo: Arc<dyn 注文Repository + Send + Sync>,
            item_repo: Arc<dyn 商品Repository + Send + Sync>,
        ) -> Self {
            Self { order_repo, item_repo }
        }

        // ROPスタイルでの実装例: 商品を注文する
        pub fn place_order(&self, item_ids: Vec<商品ID>) -> AppResult<注文ID> {
            let item_repo_clone = Arc::clone(&self.item_repo);
            let get_item_price = move |id: &商品ID| -> Option<u32> {
                 match item_repo_clone.find_by_id(id) {
                     Ok(Some(item)) => Some(item.価格),
                     _ => None,
                 }
            };

            let order_result = create_order(item_ids, get_item_price);

            order_result
                .map_err(ApplicationError::Domain)
                .and_then(|order| {
                    let order_id = order.id.clone();
                    self.order_repo
                        .save(&order)
                        .map_err(|e| ApplicationError::Repository(e.to_string()))?;
                    Ok(order_id)
                })
        }

        // 注文を発送準備中にする
        pub fn prepare_order(&self, order_id: &注文ID) -> AppResult<()> {
            self.order_repo
                .find_by_id(order_id)
                .map_err(|e| ApplicationError::Repository(e.to_string()))?
                .ok_or_else(|| ApplicationError::Domain(DomainError::注文NotFound(order_id.clone())))
                .and_then(|order| mark_as_preparing(order).map_err(ApplicationError::Domain))
                .and_then(|updated_order| {
                     self.order_repo.save(&updated_order)
                         .map_err(|e| ApplicationError::Repository(e.to_string()))
                })
        }

         // 注文を発送済みにする
        pub fn ship_order(&self, order_id: &注文ID) -> AppResult<()> {
            self.order_repo
                .find_by_id(order_id)
                .map_err(|e| ApplicationError::Repository(e.to_string()))?
                .ok_or_else(|| ApplicationError::Domain(DomainError::注文NotFound(order_id.clone())))
                .and_then(|order| mark_as_shipped(order).map_err(ApplicationError::Domain))
                .and_then(|updated_order| {
                     self.order_repo.save(&updated_order)
                        .map_err(|e| ApplicationError::Repository(e.to_string()))
                })
        }

        // 注文の詳細を取得する (Query)
        pub fn get_order_details(&self, order_id: &注文ID) -> AppResult<Option<注文>> {
            self.order_repo.find_by_id(order_id)
                .map_err(|e| ApplicationError::Repository(e.to_string()))
        }
    }

    // --- Application Tests ---
    #[cfg(test)]
    mod tests {
        use super::*;
        use mockall::predicate::*;
        use crate::domain::{Mock注文Repository, Mock商品Repository};

         // --- Helper functions/data for tests ---
        fn create_dummy_item(id: 商品ID, price: u32) -> 商品 {
            商品 { id, 名前: "テスト商品".to_string(), 価格: price }
        }

         fn create_dummy_order_app(id: 注文ID, item_ids: Vec<商品ID>, total: u32, state: 注文状態) -> 注文 {
            注文 { id, 商品リスト: item_ids, 合計金額: total, 状態: state }
        }


        #[test]
        fn test_place_order_success() {
            let item_id = domain::商品ID::new();
            let item_price = 1500u32;
            let order_id = domain::注文ID::new();

            let mut mock_item_repo = Mock商品Repository::new();
            let mut mock_order_repo = Mock注文Repository::new();

            let item_id_clone = item_id.clone();
            mock_item_repo.expect_find_by_id()
                .with(eq(item_id_clone.clone()))
                .times(1)
                .returning(move |_| Ok(Some(create_dummy_item(item_id_clone.clone(), item_price))));

            mock_order_repo.expect_save()
                 .withf(move |order: &注文| {
                     order.商品リスト == vec![item_id.clone()] &&
                     order.合計金額 == item_price &&
                     order.状態 == 注文状態::受付済み
                 })
                .times(1)
                .returning(move |order| {
                    Ok(())
                 });

            let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(mock_item_repo));
            let result = service.place_order(vec![item_id.clone()]);

            assert!(result.is_ok());
        }


        #[test]
        fn test_place_order_fail_item_not_found() {
            let item_id = domain::商品ID::new();

            let mut mock_item_repo = Mock商品Repository::new();
            let mock_order_repo = Mock注文Repository::new();

            mock_item_repo.expect_find_by_id()
                .with(eq(item_id.clone()))
                .times(1)
                .returning(|_| Ok(None));

            let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(mock_item_repo));
            let result = service.place_order(vec![item_id.clone()]);

            assert!(matches!(result, Err(ApplicationError::Domain(DomainError::商品NotFound(_)))));
        }

         #[test]
        fn test_place_order_fail_empty_items() {
             let mock_item_repo = Mock商品Repository::new();
             let mock_order_repo = Mock注文Repository::new();

             let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(mock_item_repo));
             let result = service.place_order(vec![]);

             assert!(matches!(result, Err(ApplicationError::Domain(DomainError::注文商品空エラー))));
        }

        #[test]
        fn test_place_order_fail_save_error() {
            let item_id = domain::商品ID::new();
            let item_price = 1500u32;

            let mut mock_item_repo = Mock商品Repository::new();
            let mut mock_order_repo = Mock注文Repository::new();

            let item_id_clone = item_id.clone();
             mock_item_repo.expect_find_by_id()
                .with(eq(item_id_clone.clone()))
                .times(1)
                .returning(move |_| Ok(Some(create_dummy_item(item_id_clone.clone(), item_price))));

             mock_order_repo.expect_save()
                 .times(1)
                 .returning(|_| Err(DomainError::注文NotFound(domain::注文ID::new())));

            let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(mock_item_repo));
            let result = service.place_order(vec![item_id.clone()]);

            assert!(matches!(result, Err(ApplicationError::Repository(_))));
        }


        #[test]
        fn test_prepare_order_success() {
            let order_id = domain::注文ID::new();
            let initial_order = create_dummy_order_app(
                order_id.clone(), vec![], 0, 注文状態::受付済み
            );

            let mut mock_order_repo = Mock注文Repository::new();

            mock_order_repo.expect_find_by_id()
                .with(eq(order_id.clone()))
                .times(1)
                .returning(move |_| Ok(Some(initial_order.clone())));

            mock_order_repo.expect_save()
                 .withf(move |order: &注文| {
                     order.id == order_id && order.状態 == 注文状態::発送準備中
                 })
                .times(1)
                .returning(|_| Ok(()));

            let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
            let result = service.prepare_order(&order_id);

            assert!(result.is_ok());
        }

        #[test]
        fn test_prepare_order_fail_not_found() {
             let order_id = domain::注文ID::new();
             let mut mock_order_repo = Mock注文Repository::new();

             mock_order_repo.expect_find_by_id()
                 .with(eq(order_id.clone()))
                 .times(1)
                 .returning(|_| Ok(None));

            mock_order_repo.expect_save().times(0);

             let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
             let result = service.prepare_order(&order_id);

             assert_eq!(result, Err(ApplicationError::Domain(DomainError::注文NotFound(order_id))));
        }

         #[test]
        fn test_prepare_order_fail_invalid_state() {
             let order_id = domain::注文ID::new();
             let initial_order = create_dummy_order_app(
                 order_id.clone(), vec![], 0, 注文状態::発送準備中
             );
             let current_state = initial_order.状態.clone();

             let mut mock_order_repo = Mock注文Repository::new();

             mock_order_repo.expect_find_by_id()
                 .with(eq(order_id.clone()))
                 .times(1)
                 .returning(move |_| Ok(Some(initial_order.clone())));

             mock_order_repo.expect_save().times(0);

             let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
             let result = service.prepare_order(&order_id);

             assert_eq!(result, Err(ApplicationError::Domain(DomainError::不正な状態遷移エラー {
                 current: current_state,
                 requested: 注文状態::発送準備中
             })));
        }

        #[test]
        fn test_ship_order_success() {
            let order_id = domain::注文ID::new();
            let initial_order = create_dummy_order_app(
                order_id.clone(), vec![], 0, 注文状態::発送準備中
            );

            let mut mock_order_repo = Mock注文Repository::new();

            mock_order_repo.expect_find_by_id()
                .with(eq(order_id.clone()))
                .times(1)
                .returning(move |_| Ok(Some(initial_order.clone())));

            mock_order_repo.expect_save()
                .withf(move |order: &注文| order.id == order_id && order.状態 == 注文状態::発送済み)
                .times(1)
                .returning(|_| Ok(()));

            let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
            let result = service.ship_order(&order_id);

            assert!(result.is_ok());
        }

         #[test]
        fn test_ship_order_fail_invalid_state() {
             let order_id = domain::注文ID::new();
             let initial_order = create_dummy_order_app(
                 order_id.clone(), vec![], 0, 注文状態::受付済み
             );
             let current_state = initial_order.状態.clone();

             let mut mock_order_repo = Mock注文Repository::new();

             mock_order_repo.expect_find_by_id()
                 .with(eq(order_id.clone()))
                 .times(1)
                 .returning(move |_| Ok(Some(initial_order.clone())));

             mock_order_repo.expect_save().times(0);

             let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
             let result = service.ship_order(&order_id);

            assert_eq!(result, Err(ApplicationError::Domain(DomainError::不正な状態遷移エラー {
                current: current_state,
                requested: 注文状態::発送済み
            })));
        }

         #[test]
         fn test_get_order_details_success() {
             let order_id = domain::注文ID::new();
             let expected_order = create_dummy_order_app(
                 order_id.clone(), vec![], 0, 注文状態::受付済み
             );
             let expected_order_clone = expected_order.clone();

             let mut mock_order_repo = Mock注文Repository::new();
             mock_order_repo.expect_find_by_id()
                 .with(eq(order_id.clone()))
                 .times(1)
                 .returning(move |_| Ok(Some(expected_order_clone.clone())));

             let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
             let result = service.get_order_details(&order_id);

             assert!(result.is_ok());
             assert_eq!(result.unwrap(), Some(expected_order));
         }

         #[test]
         fn test_get_order_details_not_found() {
             let order_id = domain::注文ID::new();
             let mut mock_order_repo = Mock注文Repository::new();
             mock_order_repo.expect_find_by_id()
                 .with(eq(order_id.clone()))
                 .times(1)
                 .returning(|_| Ok(None));

             let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
             let result = service.get_order_details(&order_id);

             assert!(result.is_ok());
             assert_eq!(result.unwrap(), None);
         }

        #[test]
        fn test_get_order_details_repository_error() {
            let order_id = domain::注文ID::new();
            let mut mock_order_repo = Mock注文Repository::new();
            mock_order_repo.expect_find_by_id()
                .with(eq(order_id.clone()))
                .times(1)
                .returning(|_| Err(DomainError::注文NotFound(order_id.clone())));

            let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
            let result = service.get_order_details(&order_id);

            assert!(matches!(result, Err(ApplicationError::Repository(_))));
        }
    }
}


// --- Infrastructure Layer ---
mod infrastructure {
    use super::domain::{self, 注文, 注文ID, 注文Repository, 商品, 商品ID, 商品Repository, DomainError};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex}; // Mutexで可変性を扱う

    // --- インメモリリポジトリの実装 ---

    // 商品リポジトリ (ダミー実装)
    #[derive(Clone, Default)] // Defaultを追加して簡単に初期化
    pub struct InMemory商品Repository {
         // Mutex<HashMap<...>> でスレッドセーフなインメモリDBを実現
        items: Arc<Mutex<HashMap<商品ID, 商品>>>,
    }

    impl InMemory商品Repository {
         pub fn new() -> Self {
             // サンプル商品データ
             let mut items_map = HashMap::new();
             let item1 = 商品 { id: 商品ID::new(), 名前: "すごい本".to_string(), 価格: 3000 };
             let item2 = 商品 { id: 商品ID::new(), 名前: "便利なツール".to_string(), 価格: 5000 };
             items_map.insert(item1.id.clone(), item1);
             items_map.insert(item2.id.clone(), item2);

             Self { items: Arc::new(Mutex::new(items_map)) }
         }

         // アプリケーション側で商品IDを知るためのヘルパー（テスト用）
         #[allow(dead_code)]
         pub fn get_sample_item_ids(&self) -> Vec<商品ID> {
             self.items.lock().unwrap().keys().cloned().collect()
         }
    }

    impl 商品Repository for InMemory商品Repository {
        fn find_by_id(&self, id: &商品ID) -> Result<Option<商品>, DomainError> {
            let items_map = self.items.lock().unwrap(); // Mutexをロック
            // clone() で HashMap から所有権のある値を取り出す
            Ok(items_map.get(id).cloned())
        }
    }


    // 注文リポジトリ (ダミー実装)
    #[derive(Clone, Default)] // Defaultを追加して簡単に初期化
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
            let mut orders_map = self.orders.lock().unwrap(); // Mutexをロック
            println!("Saving order: {:?}", order); // 永続化処理のシミュレーション
            orders_map.insert(order.id.clone(), order.clone());
            Ok(())
        }

        fn find_by_id(&self, id: &注文ID) -> Result<Option<注文>, DomainError> {
            let orders_map = self.orders.lock().unwrap();
            println!("Finding order by id: {:?}", id);
            Ok(orders_map.get(id).cloned())
        }
    }
}


// --- Main / Presentation Layer ---
fn main() -> Result<()> { // anyhow::Result を使用
    // --- 依存関係の構築 (Dependency Injection) ---
    let item_repo = Arc::new(infrastructure::InMemory商品Repository::new());
    let order_repo = Arc::new(infrastructure::InMemory注文Repository::new());
    let order_service = application::注文サービス::new(order_repo.clone(), item_repo.clone()); // リポジトリはArcで共有

    println!("--- DDD Sample Start ---");

    // サンプル商品IDを取得
    let item_id_for_order = domain::商品ID::new();
    let initial_items = infrastructure::InMemory商品Repository::new();
    let sample_item_ids = initial_items.items.lock().unwrap().keys().cloned().collect::<Vec<_>>();

    if sample_item_ids.is_empty() {
        println!("サンプル商品がありません。");
        return Ok(());
    }
     let items_to_order = vec![sample_item_ids[0].clone()]; // 最初のサンプル商品を注文

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
                     // 変更後の状態を確認
                     match order_service.get_order_details(&order_id) {
                         Ok(Some(order)) => println!("現在の注文状態: {:?}", order.状態),
                         _ => println!("注文再取得エラー"),
                     }

                     println!("\n4. 注文を発送済みにします");
                     match order_service.ship_order(&order_id) {
                         Ok(_) => {
                              println!("発送済みに状態変更成功！");
                             // 変更後の状態を確認
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

     // --- 不正な状態遷移のテスト ---
     println!("\n5. 受付済みの注文を直接発送済みにしようとしてみる（エラーになるはず）");
     let items_to_order_2 = vec![sample_item_ids[0].clone()];
     if let Ok(order_id_2) = order_service.place_order(items_to_order_2) {
         println!("新しい注文ID: {:?}", order_id_2);
         match order_service.ship_order(&order_id_2) {
              Ok(_) => println!("エラーが発生するはずが、成功してしまいました。"),
              Err(e) => println!("期待通りのエラー: {}", e), // DomainError::不正な状態遷移エラー が表示されるはず
         }
     }


    println!("\n--- DDD Sample End ---");

    Ok(())
} 