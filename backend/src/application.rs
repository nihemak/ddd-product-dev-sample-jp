use std::sync::Arc;
use thiserror::Error;
/* --- 古いドメイン要素の利用箇所をコメントアウト --- */
// use crate::domain::{
//     注文作成, 発送準備中へ変更, 発送済みへ変更, 注文, 注文ID, 注文Repository,
//     注文状態, 商品, 商品ID, 商品Repository, DomainError,
// };
use crate::domain::{DomainError, 予約ID, 商品ID}; // DomainError, 予約ID, 商品ID は ApplicationError などで参照される可能性があるため残す (必要なら後で修正)

// --- アプリケーションエラー ---
#[derive(Error, Debug, PartialEq)]
pub enum ApplicationError {
    #[error("ドメインエラー: {0}")]
    Domain(#[from] DomainError),
    #[error("リポジトリ操作エラー: {0}")]
    Repository(String),
    #[allow(dead_code)]
    #[error("予期せぬエラー: {0}")]
    Unexpected(String),
}

// Result型エイリアス
pub type AppResult<T> = Result<T, ApplicationError>;

// --- ユースケース / ワークフロー ---
/* --- 古いサービス実装をコメントアウト --- */
/*
pub struct 注文サービス {
    order_repo: Arc<dyn 注文Repository>,
    item_repo: Arc<dyn 商品Repository>,
}

impl 注文サービス {
    pub fn new(
        order_repo: Arc<dyn 注文Repository>,
        item_repo: Arc<dyn 商品Repository>,
    ) -> Self {
        Self { order_repo, item_repo }
    }

    pub fn 注文受付(&self, item_ids: Vec<商品ID>) -> AppResult<注文ID> {
        let item_repo_clone = Arc::clone(&self.item_repo);
        let get_item_price = move |id: &商品ID| -> Option<u32> {
            match item_repo_clone.find_by_id(id) {
                Ok(Some(item)) => Some(item.価格),
                _ => None,
            }
        };

        let order_result = 注文作成(item_ids, get_item_price);

        order_result
            .map_err(ApplicationError::Domain)
            .and_then(|order| {
                let order_id = order.id;
                self.order_repo
                    .save(&order)
                    .map_err(|e| ApplicationError::Repository(e.to_string()))?;
                Ok(order_id)
            })
    }

    pub fn 注文発送準備(&self, order_id: &注文ID) -> AppResult<()> {
        self.order_repo
            .find_by_id(order_id)
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::Domain(DomainError::注文NotFound(*order_id)))
            .and_then(|order| 発送準備中へ変更(order).map_err(ApplicationError::Domain))
            .and_then(|updated_order| {
                self.order_repo
                    .save(&updated_order)
                    .map_err(|e| ApplicationError::Repository(e.to_string()))
            })
    }

    pub fn 注文発送(&self, order_id: &注文ID) -> AppResult<()> {
        self.order_repo
            .find_by_id(order_id)
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::Domain(DomainError::注文NotFound(*order_id)))
            .and_then(|order| 発送済みへ変更(order).map_err(ApplicationError::Domain))
            .and_then(|updated_order| {
                self.order_repo
                    .save(&updated_order)
                    .map_err(|e| ApplicationError::Repository(e.to_string()))
            })
    }

    pub fn 注文詳細取得(&self, order_id: &注文ID) -> AppResult<Option<注文>> {
        self.order_repo
            .find_by_id(order_id)
            .map_err(|e| ApplicationError::Repository(e.to_string()))
    }
}
*/

// --- Application Tests ---
/* --- 古いテストをコメントアウト --- */
/*
#[cfg(test)]
mod tests {
    use super::*; // 親モジュール(application)の要素を使う
    use mockall::predicate::*;
    use crate::domain::{Mock注文Repository, Mock商品Repository, 注文状態, 商品ID, 注文ID, 商品, 注文, DomainError}; // ワイルドカードから元に戻す
    use std::sync::Arc;

    // --- Helper functions/data for tests ---
    fn create_dummy_item(id: 商品ID, price: u32) -> 商品 {
        商品 { id, 名前: "テスト商品".to_string(), 価格: price }
    }

    fn create_dummy_order_app(id: 注文ID, item_ids: Vec<商品ID>, total: u32, state: 注文状態) -> 注文 {
        注文 { id, 商品リスト: item_ids, 合計金額: total, 状態: state }
    }

    #[test]
    fn test_place_order_success() {
        let item_id = 商品ID::new();
        let item_price = 1500u32;
        // let _order_id = 注文ID::new(); // saveの検証には使うが、結果の比較は難しい

        let mut mock_item_repo = Mock商品Repository::new();
        let mut mock_order_repo = Mock注文Repository::new();

        let item_id_clone = item_id.clone();
        mock_item_repo.expect_find_by_id()
            .with(eq(item_id_clone))
            .times(1)
            .returning(move |_| Ok(Some(create_dummy_item(item_id_clone, item_price))));

        mock_order_repo.expect_save()
             .withf(move |order: &注文| {
                 order.商品リスト == vec![item_id] &&
                 order.合計金額 == item_price &&
                 order.状態 == 注文状態::受付済み
             })
            .times(1)
            .returning(move |_order| Ok(()));


        let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(mock_item_repo));
        let result = service.注文受付(vec![item_id]);

        assert!(result.is_ok());
    }


    #[test]
    fn test_place_order_fail_item_not_found() {
        let item_id = 商品ID::new();

        let mut mock_item_repo = Mock商品Repository::new();
        let mock_order_repo = Mock注文Repository::new();

        mock_item_repo.expect_find_by_id()
            .with(eq(item_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(mock_item_repo));
        let result = service.注文受付(vec![item_id]);

        assert!(matches!(result, Err(ApplicationError::Domain(DomainError::商品NotFound(id))) if id == item_id));
    }

     #[test]
    fn test_place_order_fail_empty_items() {
         let mock_item_repo = Mock商品Repository::new();
         let mock_order_repo = Mock注文Repository::new();

         let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(mock_item_repo));
         let result = service.注文受付(vec![]);

         assert!(matches!(result, Err(ApplicationError::Domain(DomainError::注文商品空エラー))));
    }

    #[test]
    fn test_place_order_fail_save_error() {
        let item_id = 商品ID::new();
        let item_price = 1500u32;

        let mut mock_item_repo = Mock商品Repository::new();
        let mut mock_order_repo = Mock注文Repository::new();

        let item_id_clone = item_id;
         mock_item_repo.expect_find_by_id()
            .with(eq(item_id_clone))
            .times(1)
            .returning(move |_| Ok(Some(create_dummy_item(item_id_clone, item_price))));

         mock_order_repo.expect_save()
             .times(1)
             .returning(|_| Err(DomainError::注文NotFound(注文ID::new())));

        let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(mock_item_repo));
        let result = service.注文受付(vec![item_id]);

        assert!(matches!(result, Err(ApplicationError::Repository(_))));
    }


    #[test]
    fn test_prepare_order_success() {
        let order_id = 注文ID::new();
        let initial_order = create_dummy_order_app(
            order_id, vec![], 0, 注文状態::受付済み
        );

        let mut mock_order_repo = Mock注文Repository::new();

        let initial_order_clone = initial_order.clone(); // クロージャ用に clone
        mock_order_repo.expect_find_by_id()
            .with(eq(order_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_order_clone.clone()))); // Option<注文> を返す

        mock_order_repo.expect_save()
             .withf(move |order: &注文| {
                 order.id == order_id && order.状態 == 注文状態::発送準備中
             })
            .times(1)
            .returning(|_| Ok(()));

        let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
        let result = service.注文発送準備(&order_id);

        assert!(result.is_ok());
    }

    #[test]
    fn test_prepare_order_fail_not_found() {
         let order_id = 注文ID::new();
         let mut mock_order_repo = Mock注文Repository::new();

         mock_order_repo.expect_find_by_id()
             .with(eq(order_id))
             .times(1)
             .returning(|_| Ok(None));

        mock_order_repo.expect_save().times(0);

         let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
         let result = service.注文発送準備(&order_id);

         assert_eq!(result, Err(ApplicationError::Domain(DomainError::注文NotFound(order_id))));
    }

     #[test]
    fn test_prepare_order_fail_invalid_state() {
         let order_id = 注文ID::new();
         let initial_order = create_dummy_order_app(
             order_id, vec![], 0, 注文状態::発送準備中
         );
         let current_state = initial_order.状態.clone();

         let mut mock_order_repo = Mock注文Repository::new();

        let initial_order_clone = initial_order.clone(); // クロージャ用に clone
         mock_order_repo.expect_find_by_id()
             .with(eq(order_id))
             .times(1)
             .returning(move |_| Ok(Some(initial_order_clone.clone())));

         mock_order_repo.expect_save().times(0);

         let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
         let result = service.注文発送準備(&order_id);

         assert_eq!(result, Err(ApplicationError::Domain(DomainError::不正な状態遷移エラー {
             current: current_state,
             requested: 注文状態::発送準備中
         })));
    }

    #[test]
    fn test_ship_order_success() {
        let order_id = 注文ID::new();
        let initial_order = create_dummy_order_app(
            order_id, vec![], 0, 注文状態::発送準備中
        );

        let mut mock_order_repo = Mock注文Repository::new();

        let initial_order_clone = initial_order.clone(); // クロージャ用に clone
        mock_order_repo.expect_find_by_id()
            .with(eq(order_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_order_clone.clone())));

        mock_order_repo.expect_save()
            .withf(move |order: &注文| order.id == order_id && order.状態 == 注文状態::発送済み)
            .times(1)
            .returning(|_| Ok(()));

        let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
        let result = service.注文発送(&order_id);

        assert!(result.is_ok());
    }

     #[test]
    fn test_ship_order_fail_invalid_state() {
         let order_id = 注文ID::new();
         let initial_order = create_dummy_order_app(
             order_id, vec![], 0, 注文状態::受付済み
         );
         let current_state = initial_order.状態.clone();

         let mut mock_order_repo = Mock注文Repository::new();

        let initial_order_clone = initial_order.clone(); // クロージャ用に clone
         mock_order_repo.expect_find_by_id()
             .with(eq(order_id))
             .times(1)
             .returning(move |_| Ok(Some(initial_order_clone.clone())));

         mock_order_repo.expect_save().times(0);

         let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
         let result = service.注文発送(&order_id);

        assert_eq!(result, Err(ApplicationError::Domain(DomainError::不正な状態遷移エラー {
            current: current_state,
            requested: 注文状態::発送済み
        })));
    }

     #[test]
     fn test_get_order_details_success() {
         let order_id = 注文ID::new();
         let expected_order = create_dummy_order_app(
             order_id, vec![], 0, 注文状態::受付済み
         );
         let expected_order_clone = expected_order.clone();

         let mut mock_order_repo = Mock注文Repository::new();
         mock_order_repo.expect_find_by_id()
             .with(eq(order_id))
             .times(1)
             .returning(move |_| Ok(Some(expected_order_clone.clone())));

         let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
         let result = service.注文詳細取得(&order_id);

         assert!(result.is_ok());
         assert_eq!(result.unwrap(), Some(expected_order));
     }

     #[test]
     fn test_get_order_details_not_found() {
         let order_id = 注文ID::new();
         let mut mock_order_repo = Mock注文Repository::new();
         mock_order_repo.expect_find_by_id()
             .with(eq(order_id))
             .times(1)
             .returning(|_| Ok(None));

         let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
         let result = service.注文詳細取得(&order_id);

         assert!(result.is_ok());
         assert_eq!(result.unwrap(), None);
     }

    #[test]
    fn test_get_order_details_repository_error() {
        let order_id = 注文ID::new();
        let mut mock_order_repo = Mock注文Repository::new();
        mock_order_repo.expect_find_by_id()
            .with(eq(order_id))
            .times(1)
            .returning(move |_| Err(DomainError::注文NotFound(order_id)));

        let service = 注文サービス::new(Arc::new(mock_order_repo), Arc::new(Mock商品Repository::new()));
        let result = service.注文詳細取得(&order_id);

        assert!(matches!(result, Err(ApplicationError::Repository(_))));
    }
}
*/ 