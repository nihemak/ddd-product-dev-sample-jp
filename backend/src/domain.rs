// src/domain.rs

// Define internal module and re-export
pub mod core { // pub mod core の追加
    use uuid::Uuid;
    use thiserror::Error;

    // --- 値オブジェクト ---
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct 商品ID(Uuid);

    impl 商品ID {
        pub fn new() -> Self { Self(Uuid::new_v4()) }
        #[allow(dead_code)] pub fn from_uuid(id: Uuid) -> Self { Self(id) }
        #[allow(dead_code)] pub fn as_uuid(&self) -> &Uuid { &self.0 }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct 注文ID(Uuid);

    impl 注文ID {
        pub fn new() -> Self { Self(Uuid::new_v4()) }
        #[allow(dead_code)] pub fn from_uuid(id: Uuid) -> Self { Self(id) }
        #[allow(dead_code)] pub fn as_uuid(&self) -> &Uuid { &self.0 }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct 商品 {
        pub id: 商品ID,
        #[allow(dead_code)] pub 名前: String,
        pub 価格: u32,
    }

    // --- エンティティ ---
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum 注文状態 { 受付済み, 発送準備中, 発送済み, キャンセル済み }

    #[derive(Debug, Clone, PartialEq)]
    pub struct 注文 {
        pub id: 注文ID,
        #[allow(dead_code)] pub 商品リスト: Vec<商品ID>,
        #[allow(dead_code)] pub 合計金額: u32,
        pub 状態: 注文状態,
    }

    // --- ドメインエラー ---
    #[derive(Error, Debug, PartialEq)]
    pub enum DomainError {
        #[error("注文に商品が含まれていません")] 注文商品空エラー,
        #[error("指定された状態遷移は許可されていません: 現在の状態={current:?}, 要求された状態={requested:?}")]
        不正な状態遷移エラー { current: 注文状態, requested: 注文状態 },
        #[error("注文が見つかりません: ID={0:?}")] 注文NotFound(注文ID),
        #[error("商品が見つかりません: ID={0:?}")] 商品NotFound(商品ID),
        #[error("価格計算中にオーバーフローが発生しました")] 価格計算オーバーフロー,
    }

    // --- ドメインサービス / ロジック関数 ---
    pub fn 合計金額計算(
        item_ids: &[商品ID],
        get_item_price: impl Fn(&商品ID) -> Option<u32>,
    ) -> Result<u32, DomainError> {
        let mut total: u32 = 0;
        if item_ids.is_empty() { return Ok(0); }
        for item_id in item_ids {
            match get_item_price(item_id) {
                Some(price) => { total = total.checked_add(price).ok_or(DomainError::価格計算オーバーフロー)? },
                None => return Err(DomainError::商品NotFound(*item_id)),
            }
        }
        Ok(total)
    }

    pub fn 注文作成(
        item_ids: Vec<商品ID>,
        get_item_price: impl Fn(&商品ID) -> Option<u32>,
    ) -> Result<注文, DomainError> {
        if item_ids.is_empty() { return Err(DomainError::注文商品空エラー); }
        let total_price = 合計金額計算(&item_ids, get_item_price)?;
        Ok(注文 { id: 注文ID::new(), 商品リスト: item_ids, 合計金額: total_price, 状態: 注文状態::受付済み })
    }

    pub fn 発送準備中へ変更(order: 注文) -> Result<注文, DomainError> {
        match order.状態 {
            注文状態::受付済み => Ok(注文 { 状態: 注文状態::発送準備中, ..order }),
            _ => Err(DomainError::不正な状態遷移エラー { current: order.状態, requested: 注文状態::発送準備中 }),
        }
    }

    pub fn 発送済みへ変更(order: 注文) -> Result<注文, DomainError> {
         match order.状態 {
            注文状態::発送準備中 => Ok(注文 { 状態: 注文状態::発送済み, ..order }),
            _ => Err(DomainError::不正な状態遷移エラー { current: order.状態, requested: 注文状態::発送済み }),
        }
    }

    #[allow(dead_code)]
    pub fn 注文キャンセル(order: 注文) -> Result<注文, DomainError> {
        match order.状態 {
            注文状態::発送済み => Err(DomainError::不正な状態遷移エラー { current: order.状態, requested: 注文状態::キャンセル済み }),
            _ => Ok(注文 { 状態: 注文状態::キャンセル済み, ..order }),
        }
    }

    // --- リポジトリインターフェース (トレイト) ---
    #[cfg_attr(test, mockall::automock)]
    pub trait 注文Repository: Send + Sync {
        fn save(&self, order: &注文) -> Result<(), DomainError>;
        fn find_by_id(&self, id: &注文ID) -> Result<Option<注文>, DomainError>;
    }

    #[cfg_attr(test, mockall::automock)]
    pub trait 商品Repository: Send + Sync {
        fn find_by_id(&self, id: &商品ID) -> Result<Option<商品>, DomainError>;
    }
} // End of pub mod core

// Re-export all public items from the core module
pub use core::*;

// --- Domain Tests ---
#[cfg(test)]
mod tests {
    use super::core::*; // Use items from the inner core module now
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
        let item_ids = vec![item_id1, item_id2];
        let mut prices = HashMap::new();
        prices.insert(item_id1, 1000);
        prices.insert(item_id2, 500);
        let get_price = |id: &商品ID| prices.get(id).cloned();
        assert_eq!(合計金額計算(&item_ids, get_price), Ok(1500));
    }

    #[test]
    fn test_calculate_total_price_item_not_found() {
        let item_id1 = create_dummy_item_id();
        let item_id_unknown = create_dummy_item_id();
        let item_ids = vec![item_id1, item_id_unknown];
        let mut prices = HashMap::new();
        prices.insert(item_id1, 1000);
        let get_price = |id: &商品ID| prices.get(id).cloned();
        assert_eq!(
            合計金額計算(&item_ids, get_price),
            Err(DomainError::商品NotFound(item_id_unknown))
        );
    }

    #[test]
    fn test_calculate_total_price_empty_list() {
         let item_ids: Vec<商品ID> = vec![];
         let get_price = |_id: &商品ID| -> Option<u32> { None };
         assert_eq!(合計金額計算(&item_ids, get_price), Ok(0));
    }

    #[test]
    fn test_calculate_total_price_overflow() {
        let item_id1 = create_dummy_item_id();
        let item_id2 = create_dummy_item_id();
        let item_ids_two = vec![item_id1, item_id2];
        let get_price_max = |id: &商品ID| {
            if id == &item_id1 { Some(u32::MAX) }
            else if id == &item_id2 { Some(1) }
            else { None }
        };
        assert_eq!(
            合計金額計算(&item_ids_two, get_price_max),
            Err(DomainError::価格計算オーバーフロー)
        );
        let get_price_half_max = |id: &商品ID| {
             if id == &item_id1 { Some(u32::MAX / 2 + 1) }
             else if id == &item_id2 { Some(u32::MAX / 2 + 1) }
             else { None }
         };
         assert_eq!(
             合計金額計算(&item_ids_two, get_price_half_max),
             Err(DomainError::価格計算オーバーフロー)
         );
    }

    #[test]
    fn test_create_order_success() {
        let item_id1 = create_dummy_item_id();
        let item_ids = vec![item_id1];
        let get_price = |id: &商品ID| if id == &item_id1 { Some(2000) } else { None };
        let result = 注文作成(item_ids.clone(), get_price);
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
        assert_eq!(注文作成(item_ids, get_price), Err(DomainError::注文商品空エラー));
    }

    #[test]
    fn test_create_order_item_not_found() {
        let item_id_unknown = create_dummy_item_id();
        let item_ids = vec![item_id_unknown];
         let get_price = |_id: &商品ID| None;
        assert_eq!(注文作成(item_ids, get_price), Err(DomainError::商品NotFound(item_id_unknown)));
    }

    #[test]
    fn test_mark_as_preparing_success() {
        let order = create_dummy_order(注文状態::受付済み);
        let result = 発送準備中へ変更(order);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().状態, 注文状態::発送準備中);
    }

    #[test]
    fn test_mark_as_preparing_fail_invalid_state() {
        let order = create_dummy_order(注文状態::発送準備中);
        let current_state = order.状態;
        let result = 発送準備中へ変更(order);
        assert_eq!(result, Err(DomainError::不正な状態遷移エラー { current: current_state, requested: 注文状態::発送準備中 }));
         let order_shipped = create_dummy_order(注文状態::発送済み);
         let current_state_shipped = order_shipped.状態;
         let result_shipped = 発送準備中へ変更(order_shipped);
         assert_eq!(result_shipped, Err(DomainError::不正な状態遷移エラー { current: current_state_shipped, requested: 注文状態::発送準備中 }));
    }

    #[test]
    fn test_mark_as_shipped_success() {
        let order = create_dummy_order(注文状態::発送準備中);
        let result = 発送済みへ変更(order);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().状態, 注文状態::発送済み);
    }

    #[test]
    fn test_mark_as_shipped_fail_invalid_state() {
        let order = create_dummy_order(注文状態::受付済み);
        let current_state = order.状態;
        let result = 発送済みへ変更(order);
         assert_eq!(result, Err(DomainError::不正な状態遷移エラー { current: current_state, requested: 注文状態::発送済み }));
         let order_shipped = create_dummy_order(注文状態::発送済み);
         let current_state_shipped = order_shipped.状態;
         let result_shipped = 発送済みへ変更(order_shipped);
         assert_eq!(result_shipped, Err(DomainError::不正な状態遷移エラー { current: current_state_shipped, requested: 注文状態::発送済み }));
    }

    #[test]
    fn test_cancel_order_success() {
         let order_received = create_dummy_order(注文状態::受付済み);
         assert_eq!(注文キャンセル(order_received).unwrap().状態, 注文状態::キャンセル済み);
         let order_preparing = create_dummy_order(注文状態::発送準備中);
         assert_eq!(注文キャンセル(order_preparing).unwrap().状態, 注文状態::キャンセル済み);
    }

    #[test]
    fn test_cancel_order_fail_when_shipped() {
        let order = create_dummy_order(注文状態::発送済み);
        let current_state = order.状態;
        let result = 注文キャンセル(order);
        assert_eq!(result, Err(DomainError::不正な状態遷移エラー { current: current_state, requested: 注文状態::キャンセル済み }));
    }
} 