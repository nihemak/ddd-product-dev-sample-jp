use anyhow::Result; // anyhow::Result を使う想定
use thiserror::Error;
use std::sync::Arc;
use std::collections::HashSet;
use chrono::{DateTime};
use chrono_tz::Tz;
use crate::domain::{self, プレゼント予約Repository, DomainError, 予約ID, ユーザーID, 届け先ID, 記念日, ラッピング種類, 商品ID, 支払いID, 金額, プレゼント予約状態};

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

/// プレゼント予約に関するユースケースを提供するサービス
pub struct プレゼント予約サービス {
    reservation_repo: Arc<dyn プレゼント予約Repository>,
    // 必要に応じて他のリポジトリ (例: 商品リポジトリ) も追加
}

impl プレゼント予約サービス {
    /// 新しいプレゼント予約サービスを生成する
    pub fn new(reservation_repo: Arc<dyn プレゼント予約Repository>) -> Self {
        Self { reservation_repo }
    }

    /// プレゼント予約を受け付ける (MVP: 発送代行を想定)
    pub fn プレゼント予約受付(
        &self,
        依頼者id: ユーザーID,
        届け先id: 届け先ID,
        記念日: 記念日,
        メッセージ内容: Option<String>,
        ラッピング: ラッピング種類,
        配送希望日時: Option<DateTime<Tz>>,
        商品idリスト: HashSet<商品ID>, // MVPでは手配品情報かもしれないが、一旦IDリストで
        支払いid: 支払いID, // 支払い処理はMVP以降で実装想定
        合計金額: 金額,
    ) -> AppResult<予約ID> {
        // 1. ドメインのファクトリ関数を呼び出して予約を作成
        let reservation_result = domain::予約を受け付ける(
            依頼者id,
            届け先id,
            記念日,
            メッセージ内容,
            ラッピング,
            配送希望日時,
            商品idリスト,
            支払いid,
            合計金額,
        );

        // 2. 結果を検証し、リポジトリで永続化
        reservation_result.map_err(ApplicationError::from) // DomainError を ApplicationError に変換
            .and_then(|received_reservation| {
                let reservation_id = received_reservation.base.id;
                // 状態をEnumでラップして保存
                let reservation_state = プレゼント予約状態::予約受付済み(received_reservation);
                self.reservation_repo.save(&reservation_state)
                    .map_err(|e| ApplicationError::Repository(e.to_string()))?; // Repository エラーをラップ
                Ok(reservation_id)
            })
    }

    /// 指定されたIDの予約詳細を取得する
    pub fn 予約詳細取得(&self, 予約id: &予約ID) -> AppResult<Option<プレゼント予約状態>> {
        self.reservation_repo.find_by_id(予約id)
            .map_err(|e| ApplicationError::Repository(e.to_string())) // Repository エラーをラップ
    }

    /// 予約を発送準備中にする
    pub fn 発送準備を開始する(&self, 予約id: &予約ID, 梱包担当者id: ユーザーID) -> AppResult<()> {
        // 1. 予約をリポジトリから取得
        let current_state = self.reservation_repo.find_by_id(予約id)
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::Domain(DomainError::予約NotFound(*予約id)))?; // 見つからない場合はエラー

        // 2. 現在の状態を確認し、ドメインロジックを呼び出す
        match current_state {
            プレゼント予約状態::予約受付済み(received_reservation) => {
                // ドメインオブジェクトのメソッドを呼び出して状態遷移
                let preparing_reservation = received_reservation.発送準備を開始する(梱包担当者id)
                    .map_err(ApplicationError::from)?; // DomainErrorをラップ
                
                // 3. 新しい状態をリポジトリに保存
                let new_state = プレゼント予約状態::発送準備中(preparing_reservation);
                self.reservation_repo.save(&new_state)
                    .map_err(|e| ApplicationError::Repository(e.to_string()))
            }
            // 他の状態からの遷移は不正とする
            _ => Err(ApplicationError::Domain(DomainError::不正な状態遷移 {
                current_state_type: format!("{:?}", current_state) // 現在の状態型を文字列で渡す
            })),
        }
    }

    /// 予約を発送済みにする
    pub fn 発送を完了する(&self, 予約id: &予約ID, 配送伝票番号: String) -> AppResult<()> {
        // 1. 予約をリポジトリから取得
        let current_state = self.reservation_repo.find_by_id(予約id)
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::Domain(DomainError::予約NotFound(*予約id)))?; // 見つからない場合はエラー

        // 2. 現在の状態を確認し、ドメインロジックを呼び出す
        match current_state {
            プレゼント予約状態::発送準備中(preparing_reservation) => {
                // ドメインオブジェクトのメソッドを呼び出して状態遷移
                let shipped_reservation = preparing_reservation.発送を完了する(配送伝票番号)
                    .map_err(ApplicationError::from)?; // DomainErrorをラップ

                // 3. 新しい状態をリポジトリに保存
                let new_state = プレゼント予約状態::発送済み(shipped_reservation);
                self.reservation_repo.save(&new_state)
                    .map_err(|e| ApplicationError::Repository(e.to_string()))
            }
            // 他の状態からの遷移は不正とする
            _ => Err(ApplicationError::Domain(DomainError::不正な状態遷移 {
                current_state_type: format!("{:?}", current_state)
            })),
        }
    }

    /// 予約をキャンセルする
    pub fn 予約をキャンセルする(
        &self,
        予約id: &予約ID,
        理由: Option<String>,
        日時: Option<DateTime<Tz>>,
    ) -> AppResult<()> {
        // 1. 予約をリポジトリから取得
        let current_state = self.reservation_repo.find_by_id(予約id)
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::Domain(DomainError::予約NotFound(*予約id)))?;

        // 2. 現在の状態に応じてキャンセル処理を実行
        let cancelled_reservation_result = match current_state {
            プレゼント予約状態::予約受付済み(received) => {
                received.予約をキャンセルする(理由, 日時)
                    .map_err(ApplicationError::from)
            }
            プレゼント予約状態::発送準備中(preparing) => {
                preparing.予約をキャンセルする(理由, 日時)
                    .map_err(ApplicationError::from)
            }
            // 発送済み、配送完了、キャンセル済みからのキャンセルは不可
            _ => Err(ApplicationError::Domain(DomainError::不正な状態遷移 {
                current_state_type: format!("{:?}", current_state)
            })),
        };

        // 3. ドメイン処理が成功した場合、新しい状態を保存
        cancelled_reservation_result.and_then(|cancelled_reservation| {
            let new_state = プレゼント予約状態::キャンセル済み(cancelled_reservation);
            self.reservation_repo.save(&new_state)
                .map_err(|e| ApplicationError::Repository(e.to_string()))
        })
    }

    /// 予約を配送完了として記録する
    pub fn 配送完了を記録する(&self, 予約id: &予約ID, 記録日時: DateTime<Tz>) -> AppResult<()> {
        // 1. 予約をリポジトリから取得
        let current_state = self.reservation_repo.find_by_id(予約id)
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::Domain(DomainError::予約NotFound(*予約id)))?;

        // 2. 現在の状態を確認し、ドメインロジックを呼び出す
        match current_state {
            プレゼント予約状態::発送済み(shipped_reservation) => {
                // ドメインオブジェクトのメソッドを呼び出して状態遷移
                let delivered_reservation = shipped_reservation.配送完了を記録する(記録日時)
                    .map_err(ApplicationError::from)?; // DomainErrorをラップ

                // 3. 新しい状態をリポジトリに保存
                let new_state = プレゼント予約状態::配送完了(delivered_reservation);
                self.reservation_repo.save(&new_state)
                    .map_err(|e| ApplicationError::Repository(e.to_string()))
            }
            // 他の状態からの遷移は不正とする
            _ => Err(ApplicationError::Domain(DomainError::不正な状態遷移 {
                current_state_type: format!("{:?}", current_state)
            })),
        }
    }

    // 他のユースケースメソッド (発送準備開始、発送完了など) もここに追加していく

}


// --- Application Tests ---
#[cfg(test)]
mod tests {
    use super::*; // 親モジュール(application)の要素を使う
    use crate::domain; // ドメイン層の型やモックを使う
    use mockall::predicate::*; // mockall のマッチャーを使う
    use std::sync::Arc;
    use chrono::NaiveDate;

    // モックリポジトリを使うための準備
    use crate::domain::Mockプレゼント予約Repository;

    // --- テスト用のヘルパー関数やデータ --- 
    fn create_dummy_ids() -> (ユーザーID, 届け先ID, 支払いID, HashSet<商品ID>) {
        let user_id = ユーザーID::new();
        let address_id = 届け先ID::new();
        let payment_id = 支払いID::new();
        let mut product_ids = HashSet::new();
        product_ids.insert(商品ID::new());
        (user_id, address_id, payment_id, product_ids)
    }

    fn create_dummy_kinenbi() -> 記念日 {
        記念日 { value: NaiveDate::from_ymd_opt(2025, 12, 25).unwrap() }
    }

    fn create_dummy_kingaku() -> 金額 {
        金額::new(5000).unwrap()
    }

    // --- 予約受付ユースケースのテスト --- 

    #[test]
    fn test_プレゼント予約受付_success() {
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let メッセージ = Some("テストメッセージ".to_string());
        let ラッピング = ラッピング種類::標準;
        let 配送日時 = None; // Option<DateTime<Tz>>

        let mut mock_repo = Mockプレゼント予約Repository::new();

        // save が呼ばれることを期待する
        // 引数の検証: 渡される reservation が期待通りか確認
        // 予約IDは内部で生成されるため、他のフィールドが一致するかを withf でチェック
        let expected_依頼者id = 依頼者id.clone();
        let expected_届け先id = 届け先id.clone();
        let expected_記念日 = 記念日.clone();
        let expected_商品idリスト = 商品idリスト.clone();
        let expected_支払いid = 支払いid.clone();
        let expected_金額 = 金額.clone();
        let expected_メッセージ = メッセージ.clone();
        let expected_ラッピング = ラッピング.clone();
        let expected_配送日時 = 配送日時.clone();

        mock_repo.expect_save()
            .withf(move |state: &プレゼント予約状態| {
                match state {
                    プレゼント予約状態::予約受付済み(ref received) => {
                        // 予約ID 以外の一致を確認
                        received.base.依頼者id == expected_依頼者id &&
                        received.base.届け先id == expected_届け先id &&
                        received.base.記念日 == expected_記念日 &&
                        received.base.メッセージ内容 == expected_メッセージ &&
                        received.base.ラッピング == expected_ラッピング &&
                        received.base.配送希望日時 == expected_配送日時 && // Option<DateTime> の比較
                        received.base.手配商品リスト == expected_商品idリスト &&
                        received.base.支払いid == expected_支払いid &&
                        received.base.合計金額 == expected_金額
                    }
                    _ => false, // 他の状態が来たらテスト失敗
                }
            })
            .times(1) // 1回だけ呼ばれる
            .returning(|_| Ok(())); // 成功を返す

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.プレゼント予約受付(
            依頼者id,
            届け先id,
            記念日,
            メッセージ,
            ラッピング,
            配送日時,
            商品idリスト,
            支払いid,
            金額,
        );

        // 結果が Ok であることを確認 (中の ID はドメイン層でテスト済みとする)
        assert!(result.is_ok());
        // assert!(matches!(result.unwrap(), domain::予約ID(_))); // フィールドがプライベートなため不可
    }

    #[test]
    fn test_プレゼント予約受付_fail_domain_error() {
        let (依頼者id, 届け先id, 支払いid, _) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let メッセージ = None;
        let ラッピング = ラッピング種類::なし;
        let 配送日時 = None;
        let 商品idリスト = HashSet::new(); // 空の商品リスト

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_save().times(0); // save は呼ばれないはず

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.プレゼント予約受付(
            依頼者id,
            届け先id,
            記念日,
            メッセージ,
            ラッピング,
            配送日時,
            商品idリスト,
            支払いid,
            金額,
        );

        // 結果が Err で、中身が期待するドメインエラーと等しいことを確認
        assert!(result.is_err());
        match result.err().unwrap() {
            ApplicationError::Domain(domain_error) => {
                assert_eq!(domain_error, DomainError::予約商品空エラー);
            }
            _ => panic!("Expected ApplicationError::Domain"),
        }
        // assert!(matches!(result.err().unwrap(), ApplicationError::Domain(DomainError::予約商品空エラー))); // PartialEq で比較する方が安全
    }

    #[test]
    fn test_プレゼント予約受付_fail_repo_error() {
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let メッセージ = Some("テストメッセージ".to_string());
        let ラッピング = ラッピング種類::標準;
        let 配送日時 = None;

        let mut mock_repo = Mockプレゼント予約Repository::new();

        // save が呼ばれるが、エラーを返すように設定
        mock_repo.expect_save()
            .times(1)
            .returning(|_| Err(DomainError::必須項目不足 { field: "テストエラー".to_string() })); // 適当なドメインエラーを返す

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.プレゼント予約受付(
            依頼者id,
            届け先id,
            記念日,
            メッセージ,
            ラッピング,
            配送日時,
            商品idリスト,
            支払いid,
            金額,
        );

        // 結果が Err で、中身が ApplicationError::Repository であることを確認
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Repository(_)));
    }

    // --- 予約詳細取得ユースケースのテスト --- 

    #[test]
    fn test_予約詳細取得_success_found() {
        let target_id = 予約ID::new();
        // 適当な予約状態を作成 (ここでは予約受付済みとする)
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received_reservation = domain::予約を受け付ける(
            依頼者id, 届け先id, 記念日.clone(), None, ラッピング種類::なし, None, 商品idリスト.clone(), 支払いid, 金額
        ).unwrap();
        // ID を差し替える (本来はリポジトリが永続化時に ID を持つので、 find_by_id は既存のIDで検索するはず)
        // しかし、テストのために `予約を受け付ける` で生成されたIDを無視し、 target_id を持つ予約状態を作る
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..received_reservation.base // 他のフィールドはコピー
        };
        let expected_state = プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 { base: base_with_target_id });
        let expected_state_clone = expected_state.clone(); // クロージャ用にクローン

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id)) // 特定のIDで呼ばれることを期待
            .times(1)
            .returning(move |_| Ok(Some(expected_state_clone.clone()))); // Ok(Some(予約状態)) を返す

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約詳細取得(&target_id);

        // 結果が Ok(Some(期待する予約状態)) であることを確認
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(expected_state));
    }

    #[test]
    fn test_予約詳細取得_success_not_found() {
        let target_id = 予約ID::new();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(|_| Ok(None)); // Ok(None) を返す

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約詳細取得(&target_id);

        // 結果が Ok(None) であることを確認
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_予約詳細取得_fail_repo_error() {
        let target_id = 予約ID::new();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Err(DomainError::予約NotFound(target_id))); // move キーワードを追加

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約詳細取得(&target_id);

        // 結果が Err で、中身が ApplicationError::Repository であることを確認
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Repository(_)));
    }

    // --- 発送準備開始ユースケースのテスト --- 

    #[test]
    fn test_発送準備を開始する_success() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();
        // find_by_id が返す予約受付済み状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received_reservation = domain::予約を受け付ける(
            依頼者id, 届け先id, 記念日.clone(), None, ラッピング種類::なし, None, 商品idリスト.clone(), 支払いid, 金額
        ).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..received_reservation.base };
        let initial_state = プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 { base: base_with_target_id.clone() });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();

        // find_by_id の期待値設定
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save の期待値設定
        // 渡される state が 発送準備中 で、データが引き継がれているか確認
        let expected_handler_id = handler_id.clone();
        mock_repo.expect_save()
            .withf(move |state: &プレゼント予約状態| {
                match state {
                    プレゼント予約状態::発送準備中(ref preparing) => {
                        preparing.base == base_with_target_id && preparing.梱包担当者id == expected_handler_id
                    }
                    _ => false,
                }
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送準備を開始する(&target_id, handler_id);

        assert!(result.is_ok());
    }

    #[test]
    fn test_発送準備を開始する_fail_not_found() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(|_| Ok(None)); // 見つからない
        mock_repo.expect_save().times(0); // save は呼ばれない

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送準備を開始する(&target_id, handler_id);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Domain(DomainError::予約NotFound(_))));
    }

    #[test]
    fn test_発送準備を開始する_fail_invalid_state() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();
        // 不正な状態 (例: 発送準備中) を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..received.base };
        let invalid_initial_state = プレゼント予約状態::発送準備中(domain::発送準備中プレゼント予約型 { base: base_with_target_id, 梱包担当者id: ユーザーID::new() });
        let invalid_initial_state_clone = invalid_initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(invalid_initial_state_clone.clone()))); // 不正な状態を返す
        mock_repo.expect_save().times(0); // save は呼ばれない

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送準備を開始する(&target_id, handler_id);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Domain(DomainError::不正な状態遷移 { .. })));
    }

    #[test]
    fn test_発送準備を開始する_fail_repo_save_error() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();
        // find_by_id が返す予約受付済み状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received_reservation = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..received_reservation.base };
        let initial_state = プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 { base: base_with_target_id });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));
        mock_repo.expect_save()
            .times(1)
            .returning(|_| Err(DomainError::必須項目不足 { field: "Save Error".to_string() })); // save がエラーを返す

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送準備を開始する(&target_id, handler_id);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Repository(_)));
    }

    #[test]
    fn test_発送準備を開始する_fail_repo_find_error() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Err(DomainError::予約NotFound(target_id))); // find_by_id がエラー
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送準備を開始する(&target_id, handler_id);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Repository(_)));
    }

    // --- 発送完了ユースケースのテスト --- 

    #[test]
    fn test_発送を完了する_success() {
        let target_id = 予約ID::new();
        let slip_number = "123-456".to_string();
        // find_by_id が返す発送準備中状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let handler_id = ユーザーID::new();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..preparing.base };
        let initial_state = プレゼント予約状態::発送準備中(domain::発送準備中プレゼント予約型 { base: base_with_target_id.clone(), 梱包担当者id: handler_id });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();

        // find_by_id の期待値設定
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save の期待値設定
        // 渡される state が 発送済み で、データと伝票番号が正しいか確認
        let expected_slip_number = slip_number.clone();
        mock_repo.expect_save()
            .withf(move |state: &プレゼント予約状態| {
                match state {
                    プレゼント予約状態::発送済み(ref shipped) => {
                        shipped.base == base_with_target_id && shipped.配送伝票番号 == expected_slip_number
                    }
                    _ => false,
                }
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送を完了する(&target_id, slip_number);

        assert!(result.is_ok());
    }

    #[test]
    fn test_発送を完了する_fail_not_found() {
        let target_id = 予約ID::new();
        let slip_number = "123-456".to_string();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(|_| Ok(None)); // 見つからない
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送を完了する(&target_id, slip_number);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Domain(DomainError::予約NotFound(_))));
    }

    #[test]
    fn test_発送を完了する_fail_invalid_state() {
        let target_id = 予約ID::new();
        let slip_number = "123-456".to_string();
        // 不正な状態 (例: 予約受付済み) を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..received.base };
        let invalid_initial_state = プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 { base: base_with_target_id });
        let invalid_initial_state_clone = invalid_initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(invalid_initial_state_clone.clone()))); // 不正な状態を返す
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送を完了する(&target_id, slip_number);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Domain(DomainError::不正な状態遷移 { .. })));
    }

    #[test]
    fn test_発送を完了する_fail_repo_save_error() {
        let target_id = 予約ID::new();
        let slip_number = "123-456".to_string();
        // find_by_id が返す発送準備中状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let handler_id = ユーザーID::new();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..preparing.base };
        let initial_state = プレゼント予約状態::発送準備中(domain::発送準備中プレゼント予約型 { base: base_with_target_id, 梱包担当者id: handler_id });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));
        mock_repo.expect_save()
            .times(1)
            .returning(|_| Err(DomainError::必須項目不足 { field: "Save Error".to_string() })); // save がエラーを返す

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送を完了する(&target_id, slip_number);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Repository(_)));
    }

    #[test]
    fn test_発送を完了する_fail_repo_find_error() {
        let target_id = 予約ID::new();
        let slip_number = "123-456".to_string();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Err(DomainError::予約NotFound(target_id))); // find_by_id がエラー
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送を完了する(&target_id, slip_number);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Repository(_)));
    }

    // --- 配送完了記録ユースケースのテスト --- 

    #[test]
    fn test_配送完了を記録する_success() {
        let target_id = 予約ID::new();
        use chrono::Utc;
        use chrono_tz::Asia::Tokyo;
        let completion_time = Utc::now().with_timezone(&Tokyo);
        // find_by_id が返す発送済み状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let handler_id = ユーザーID::new();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let slip_number = "slip-123".to_string();
        let shipped = preparing.発送を完了する(slip_number.clone()).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..shipped.base };
        let initial_state = プレゼント予約状態::発送済み(domain::発送済みプレゼント予約型 { base: base_with_target_id.clone(), 配送伝票番号: slip_number.clone() });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save の期待値設定
        // 渡される state が 配送完了 で、データと完了日時が正しいか確認
        let expected_completion_time = completion_time.clone();
        mock_repo.expect_save()
            .withf(move |state: &プレゼント予約状態| {
                match state {
                    プレゼント予約状態::配送完了(ref delivered) => {
                        delivered.base == base_with_target_id &&
                        delivered.配送伝票番号 == slip_number &&
                        delivered.配送完了日時 == expected_completion_time
                    }
                    _ => false,
                }
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.配送完了を記録する(&target_id, completion_time);

        assert!(result.is_ok());
    }

    #[test]
    fn test_配送完了を記録する_fail_not_found() {
        let target_id = 予約ID::new();
        use chrono::Utc;
        use chrono_tz::Asia::Tokyo;
        let completion_time = Utc::now().with_timezone(&Tokyo);

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(|_| Ok(None)); // 見つからない
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.配送完了を記録する(&target_id, completion_time);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Domain(DomainError::予約NotFound(_))));
    }

    #[test]
    fn test_配送完了を記録する_fail_invalid_state() {
        let target_id = 予約ID::new();
        use chrono::Utc;
        use chrono_tz::Asia::Tokyo;
        let completion_time = Utc::now().with_timezone(&Tokyo);
        // 不正な状態 (例: 発送準備中) を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let handler_id = ユーザーID::new();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..preparing.base };
        let invalid_initial_state = プレゼント予約状態::発送準備中(domain::発送準備中プレゼント予約型 { base: base_with_target_id, 梱包担当者id: handler_id });
        let invalid_initial_state_clone = invalid_initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(invalid_initial_state_clone.clone()))); // 不正な状態を返す
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.配送完了を記録する(&target_id, completion_time);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Domain(DomainError::不正な状態遷移 { .. })));
    }

    #[test]
    fn test_配送完了を記録する_fail_repo_save_error() {
        let target_id = 予約ID::new();
        use chrono::Utc;
        use chrono_tz::Asia::Tokyo;
        let completion_time = Utc::now().with_timezone(&Tokyo);
        // find_by_id が返す発送済み状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let handler_id = ユーザーID::new();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let slip_number = "slip-123".to_string();
        let shipped = preparing.発送を完了する(slip_number.clone()).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..shipped.base };
        let initial_state = プレゼント予約状態::発送済み(domain::発送済みプレゼント予約型 { base: base_with_target_id, 配送伝票番号: slip_number });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));
        mock_repo.expect_save()
            .times(1)
            .returning(|_| Err(DomainError::必須項目不足 { field: "Save Error".to_string() })); // save がエラー

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.配送完了を記録する(&target_id, completion_time);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Repository(_)));
    }

    #[test]
    fn test_配送完了を記録する_fail_repo_find_error() {
        let target_id = 予約ID::new();
        use chrono::Utc;
        use chrono_tz::Asia::Tokyo;
        let completion_time = Utc::now().with_timezone(&Tokyo);

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Err(DomainError::予約NotFound(target_id))); // find がエラー
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.配送完了を記録する(&target_id, completion_time);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Repository(_)));
    }

        // --- 予約キャンセルユースケースのテスト ---

    #[test]
    fn test_予約をキャンセルする_success_from_予約受付済み() {
        let target_id = 予約ID::new();
        let reason = Some("テスト理由".to_string());
        // Utc と Tokyo をテスト内で use します
        use chrono::Utc;
        use chrono_tz::Asia::Tokyo;
        let time = Some(Utc::now().with_timezone(&Tokyo));
        // find_by_id が返す予約受付済み状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..received.base };
        let initial_state = プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 { base: base_with_target_id.clone() });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save の期待値設定
        let expected_reason = reason.clone();
        let expected_time = time.clone();
        // base_with_target_id を move する必要があるので、ここで clone する
        let expected_base = base_with_target_id.clone();
        mock_repo.expect_save()
            .withf(move |state: &プレゼント予約状態| {
                match state {
                    プレゼント予約状態::キャンセル済み(ref cancelled) => {
                        // expected_base を move しないように参照で比較
                        cancelled.base == expected_base &&
                        cancelled.キャンセル理由 == expected_reason &&
                        cancelled.キャンセル日時 == expected_time
                    }
                    _ => false,
                }
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約をキャンセルする(&target_id, reason, time);

        assert!(result.is_ok());
    }

     #[test]
    fn test_予約をキャンセルする_success_from_発送準備中() {
        let target_id = 予約ID::new();
        let reason = None; // 理由なしケース
        use chrono::Utc;
        use chrono_tz::Asia::Tokyo;
        let time = Some(Utc::now().with_timezone(&Tokyo));
        // find_by_id が返す発送準備中状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let handler_id = ユーザーID::new();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..preparing.base };
        let initial_state = プレゼント予約状態::発送準備中(domain::発送準備中プレゼント予約型 { base: base_with_target_id.clone(), 梱包担当者id: handler_id });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save の期待値設定
        let expected_reason = reason.clone();
        let expected_time = time.clone();
        // base_with_target_id を move する必要があるので、ここで clone する
        let expected_base = base_with_target_id.clone();
        mock_repo.expect_save()
            .withf(move |state: &プレゼント予約状態| {
                match state {
                    プレゼント予約状態::キャンセル済み(ref cancelled) => {
                        // expected_base を move しないように参照で比較
                        cancelled.base == expected_base &&
                        cancelled.キャンセル理由 == expected_reason &&
                        cancelled.キャンセル日時 == expected_time
                    }
                    _ => false,
                }
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約をキャンセルする(&target_id, reason, time);

        assert!(result.is_ok());
    }

    #[test]
    fn test_予約をキャンセルする_fail_not_found() {
        let target_id = 予約ID::new();
        let reason = None;
        let time = None;

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
             .times(1)
            .returning(|_| Ok(None)); // 見つからない
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約をキャンセルする(&target_id, reason, time);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Domain(DomainError::予約NotFound(_))));
    }

     #[test]
    fn test_予約をキャンセルする_fail_invalid_state() {
        let target_id = 予約ID::new();
        let reason = None;
        let time = None;
        // 不正な状態 (例: 発送済み) を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let handler_id = ユーザーID::new();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let shipped = preparing.発送を完了する("slip".to_string()).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..shipped.base };
        let invalid_initial_state = プレゼント予約状態::発送済み(domain::発送済みプレゼント予約型 { base: base_with_target_id, 配送伝票番号: "slip".to_string() });
        let invalid_initial_state_clone = invalid_initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
             .times(1)
            .returning(move |_| Ok(Some(invalid_initial_state_clone.clone()))); // 不正な状態を返す
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約をキャンセルする(&target_id, reason, time);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Domain(DomainError::不正な状態遷移 { .. })));
     }

     #[test]
    fn test_予約をキャンセルする_fail_repo_save_error() {
        let target_id = 予約ID::new();
        let reason = None;
        let time = None;
        // find_by_id が返す予約受付済み状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(依頼者id, 届け先id, 記念日, None, ラッピング種類::なし, None, 商品idリスト, 支払いid, 金額).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース { id: target_id.clone(), ..received.base };
        let initial_state = プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 { base: base_with_target_id });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));
        mock_repo.expect_save()
             .times(1)
            .returning(|_| Err(DomainError::必須項目不足 { field: "Save Error".to_string() })); // save がエラー

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約をキャンセルする(&target_id, reason, time);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Repository(_)));
     }

    #[test]
    fn test_予約をキャンセルする_fail_repo_find_error() {
        let target_id = 予約ID::new();
        let reason = None;
        let time = None;

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo.expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Err(DomainError::予約NotFound(target_id))); // find がエラー
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約をキャンセルする(&target_id, reason, time);

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ApplicationError::Repository(_)));
    }
} 