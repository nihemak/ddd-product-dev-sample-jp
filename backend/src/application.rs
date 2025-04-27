#![allow(clippy::unnecessary_lazy_evaluations)] // TODO: ok_or_else を ok_or に修正する

use crate::domain::{
    self, DomainError, InfrastructureError, プレゼント予約Repository, プレゼント予約状態,
    ユーザーID, ラッピング種類, 予約ID, 商品ID, 届け先ID, 支払いID, 記念日, 金額,
};
use anyhow::Result; // anyhow::Result を使う想定
use chrono::DateTime;
use chrono_tz::Tz;
use std::collections::HashSet;
use std::sync::Arc;
use thiserror::Error;

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
    #[allow(clippy::too_many_arguments)]
    pub async fn プレゼント予約受付(
        &self,
        依頼者id: ユーザーID,
        届け先id: 届け先ID,
        記念日: 記念日,
        メッセージ内容: Option<String>,
        ラッピング: ラッピング種類,
        配送希望日時: Option<DateTime<Tz>>,
        商品idリスト: HashSet<商品ID>, // MVPでは手配品情報かもしれないが、一旦IDリストで
        支払いid: 支払いID,            // 支払い処理はMVP以降で実装想定
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
        let received_reservation = reservation_result.map_err(ApplicationError::from)?;

        // ↓↓↓ await と map_err の順序変更 ↓↓↓
        let reservation_id = received_reservation.base.id;
        let reservation_state =
            プレゼント予約状態::予約受付済み(received_reservation);
        self.reservation_repo
            .save(&reservation_state)
            .await // await を追加
            .map_err(|e| ApplicationError::Repository(e.to_string()))?; // Repository エラーをラップ
        Ok(reservation_id)
    }

    /// 指定されたIDの予約詳細を取得する
    pub async fn 予約詳細取得(
        &self,
        予約id: &予約ID,
    ) -> AppResult<Option<プレゼント予約状態>> {
        // ↓↓↓ await と map_err の順序変更 ↓↓↓
        self.reservation_repo
            .find_by_id(予約id)
            .await // await を追加
            .map_err(|e| ApplicationError::Repository(e.to_string())) // Repository エラーをラップ
    }

    /// 予約を発送準備中にする
    pub async fn 発送準備を開始する(
        &self,
        予約id: &予約ID,
        梱包担当者id: ユーザーID,
    ) -> AppResult<()> {
        // 1. 予約をリポジトリから取得
        let current_state = self
            .reservation_repo
            .find_by_id(予約id)
            .await // await を追加
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::Domain(DomainError::予約NotFound(*予約id)))?; // 見つからない場合はエラー

        // 2. 現在の状態を確認し、ドメインロジックを呼び出す
        match current_state {
            プレゼント予約状態::予約受付済み(received_reservation) => {
                // ドメインオブジェクトのメソッドを呼び出して状態遷移
                let preparing_reservation = received_reservation
                    .発送準備を開始する(梱包担当者id)
                    .map_err(ApplicationError::from)?; // DomainErrorをラップ

                // 3. 新しい状態をリポジトリに保存
                let new_state = プレゼント予約状態::発送準備中(preparing_reservation);
                self.reservation_repo
                    .save(&new_state)
                    .await // await を追加
                    .map_err(|e| ApplicationError::Repository(e.to_string()))
            }
            // 他の状態からの遷移は不正とする
            _ => Err(ApplicationError::Domain(
                DomainError::不正な状態遷移 {
                    current_state_type: format!("{:?}", current_state), // 現在の状態型を文字列で渡す
                },
            )),
        }
    }

    /// 予約を発送済みにする
    pub async fn 発送を完了する(
        &self,
        予約id: &予約ID,
        配送伝票番号: String,
    ) -> AppResult<()> {
        // 1. 予約をリポジトリから取得
        let current_state = self
            .reservation_repo
            .find_by_id(予約id)
            .await // await を追加
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::Domain(DomainError::予約NotFound(*予約id)))?; // 見つからない場合はエラー

        // 2. 現在の状態を確認し、ドメインロジックを呼び出す
        match current_state {
            プレゼント予約状態::発送準備中(preparing_reservation) => {
                // ドメインオブジェクトのメソッドを呼び出して状態遷移
                let shipped_reservation = preparing_reservation
                    .発送を完了する(配送伝票番号)
                    .map_err(ApplicationError::from)?; // DomainErrorをラップ

                // 3. 新しい状態をリポジトリに保存
                let new_state = プレゼント予約状態::発送済み(shipped_reservation);
                self.reservation_repo
                    .save(&new_state)
                    .await // await を追加
                    .map_err(|e| ApplicationError::Repository(e.to_string()))
            }
            // 他の状態からの遷移は不正とする
            _ => Err(ApplicationError::Domain(
                DomainError::不正な状態遷移 {
                    current_state_type: format!("{:?}", current_state),
                },
            )),
        }
    }

    /// 予約をキャンセルする
    pub async fn 予約をキャンセルする(
        &self,
        予約id: &予約ID,
        理由: Option<String>,
        日時: Option<DateTime<Tz>>,
    ) -> AppResult<()> {
        // 1. 予約をリポジトリから取得
        let current_state = self
            .reservation_repo
            .find_by_id(予約id)
            .await // await を追加
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::Domain(DomainError::予約NotFound(*予約id)))?;

        // 2. 現在の状態に応じてキャンセル処理を実行
        let cancelled_reservation_result = match current_state {
            プレゼント予約状態::予約受付済み(received) => received
                .予約をキャンセルする(理由, 日時)
                .map_err(ApplicationError::from), // DomainError -> ApplicationError
            プレゼント予約状態::発送準備中(preparing) => preparing
                .予約をキャンセルする(理由, 日時)
                .map_err(ApplicationError::from), // DomainError -> ApplicationError
            // 他の状態からのキャンセルは ApplicationError::Domain を返す
            _ => Err(ApplicationError::Domain(
                DomainError::不正な状態遷移 {
                    current_state_type: format!("{:?}", current_state),
                },
            )),
        };

        // 3. ドメイン処理が成功した場合、新しい状態を保存
        match cancelled_reservation_result {
            Ok(cancelled_reservation) => {
                let new_state =
                    プレゼント予約状態::キャンセル済み(cancelled_reservation);
                self.reservation_repo
                    .save(&new_state)
                    .await
                    .map_err(|e| ApplicationError::Repository(e.to_string()))?;
                Ok(())
            }
            Err(e) => Err(e), // エラーはそのまま返す (型は ApplicationError になっているはず)
        }
    }

    /// 予約を配送完了として記録する
    pub async fn 配送完了を記録する(
        &self,
        予約id: &予約ID,
        記録日時: DateTime<Tz>,
    ) -> AppResult<()> {
        // 1. 予約をリポジトリから取得
        let current_state = self
            .reservation_repo
            .find_by_id(予約id)
            .await // await を追加
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::Domain(DomainError::予約NotFound(*予約id)))?;

        // 2. 現在の状態を確認し、ドメインロジックを呼び出す
        match current_state {
            プレゼント予約状態::発送済み(shipped_reservation) => {
                // ドメインオブジェクトのメソッドを呼び出して状態遷移
                let delivered_reservation = shipped_reservation
                    .配送完了を記録する(記録日時)
                    .map_err(ApplicationError::from)?; // DomainErrorをラップ

                // 3. 新しい状態をリポジトリに保存
                let new_state = プレゼント予約状態::配送完了(delivered_reservation);
                self.reservation_repo
                    .save(&new_state)
                    .await // await を追加
                    .map_err(|e| ApplicationError::Repository(e.to_string()))
            }
            // 他の状態からの遷移は不正とする
            _ => Err(ApplicationError::Domain(
                DomainError::不正な状態遷移 {
                    current_state_type: format!("{:?}", current_state),
                },
            )),
        }
    }

    // 他のユースケースメソッド (発送準備開始、発送完了など) もここに追加していく

    /// サービスのヘルスチェック (DB接続確認など)
    pub async fn check_health(&self) -> Result<(), ApplicationError> {
        self.reservation_repo
            .check_db_connection()
            .await
            .map_err(|infra_err| match infra_err {
                InfrastructureError::ConnectionError(msg) => {
                    ApplicationError::Repository(format!("Health check failed: {}", msg))
                }
                InfrastructureError::DatabaseError(msg) => {
                    // check_db_connection は ConnectionError しか返さない想定だが、念のため
                    ApplicationError::Repository(format!("Health check DB error: {}", msg))
                }
            })
    }
}

// --- Application Tests ---
#[cfg(test)]
mod tests {
    use super::*; // 親モジュール(application)の要素を使う
    use crate::domain; // ドメイン層の型やモックを使う
    use crate::domain::Mockプレゼント予約Repository; // Mock を use
    use chrono::NaiveDate;
    use chrono::Utc; // Utc をインポート
    use chrono_tz::Asia::Tokyo;
    use mockall::predicate::*; // mockall のマッチャーを使う
    use std::sync::Arc; // Tokyo をインポート

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
        記念日 {
            value: NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
        }
    }

    fn create_dummy_kingaku() -> 金額 {
        金額::new(5000).unwrap()
    }

    // --- 予約受付ユースケースのテスト ---

    #[tokio::test] // #[test] -> #[tokio::test]
    async fn test_プレゼント予約受付_success() {
        // fn -> async fn
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

        mock_repo
            .expect_save()
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
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service
            .プレゼント予約受付(
                依頼者id,
                届け先id,
                記念日,
                メッセージ,
                ラッピング,
                配送日時,
                商品idリスト,
                支払いid,
                金額,
            )
            .await;

        // 結果が Ok であることを確認 (中の ID はドメイン層でテスト済みとする)
        assert!(result.is_ok());
        // assert!(matches!(result.unwrap(), domain::予約ID(_))); // フィールドがプライベートなため不可
    }

    #[tokio::test] // #[test] -> #[tokio::test]
    async fn test_プレゼント予約受付_fail_domain_error() {
        // fn -> async fn
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

        let result = service
            .プレゼント予約受付(
                依頼者id,
                届け先id,
                記念日,
                メッセージ,
                ラッピング,
                配送日時,
                商品idリスト,
                支払いid,
                金額,
            )
            .await;

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

    #[tokio::test] // #[test] -> #[tokio::test]
    async fn test_プレゼント予約受付_fail_repo_error() {
        // fn -> async fn
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let メッセージ = Some("テストメッセージ".to_string());
        let ラッピング = ラッピング種類::標準;
        let 配送日時 = None;

        let mut mock_repo = Mockプレゼント予約Repository::new();

        // save が呼ばれるが、エラーを返すように設定
        mock_repo.expect_save().times(1).returning(|_| {
            Err(DomainError::必須項目不足 {
                field: "テストエラー".to_string(),
            })
        });

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service
            .プレゼント予約受付(
                依頼者id,
                届け先id,
                記念日,
                メッセージ,
                ラッピング,
                配送日時,
                商品idリスト,
                支払いid,
                金額,
            )
            .await;

        // 結果が Err で、中身が ApplicationError::Repository であることを確認
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Repository(_)
        ));
    }

    // --- 予約詳細取得ユースケースのテスト ---

    #[tokio::test] // #[test] -> #[tokio::test]
    async fn test_予約詳細取得_success_found() {
        // fn -> async fn
        let target_id = 予約ID::new();
        // 適当な予約状態を作成 (ここでは予約受付済みとする)
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received_reservation = domain::予約を受け付ける(
            依頼者id,
            届け先id,
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid,
            金額,
        )
        .unwrap();
        // ID を差し替える (本来はリポジトリが永続化時に ID を持つので、 find_by_id は既存のIDで検索するはず)
        // しかし、テストのために `予約を受け付ける` で生成されたIDを無視し、 target_id を持つ予約状態を作る
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..received_reservation.base // 他のフィールドはコピー
        };
        let expected_state =
            プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 {
                base: base_with_target_id,
            });
        let expected_state_clone = expected_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id)) // 特定のIDで呼ばれることを期待
            .times(1)
            .returning(move |_| Ok(Some(expected_state_clone.clone())));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約詳細取得(&target_id).await;

        // 結果が Ok(Some(期待する予約状態)) であることを確認
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(expected_state));
    }

    #[tokio::test] // #[test] -> #[tokio::test]
    async fn test_予約詳細取得_success_not_found() {
        // fn -> async fn
        let target_id = 予約ID::new();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約詳細取得(&target_id).await;

        // 結果が Ok(None) であることを確認
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[tokio::test] // #[test] -> #[tokio::test]
    async fn test_予約詳細取得_fail_repo_error() {
        // fn -> async fn
        let target_id = 予約ID::new();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Err(DomainError::予約NotFound(target_id)));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.予約詳細取得(&target_id).await;

        // 結果が Err で、中身が ApplicationError::Repository であることを確認
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Repository(_)
        ));
    }

    // --- 発送準備開始ユースケースのテスト ---

    #[tokio::test] // #[test] -> #[tokio::test]
    async fn test_発送準備を開始する_success() {
        // fn -> async fn
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();
        // find_by_id が返す予約受付済み状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received_reservation = domain::予約を受け付ける(
            依頼者id,
            届け先id,
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid,
            金額,
        )
        .unwrap();
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..received_reservation.base
        };
        let initial_state =
            プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 {
                base: base_with_target_id.clone(),
            });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();

        // find_by_id の期待値設定
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save の期待値設定
        mock_repo
            .expect_save()
            .withf(move |state: &プレゼント予約状態| match state {
                プレゼント予約状態::発送準備中(ref preparing) => {
                    preparing.base == base_with_target_id && preparing.梱包担当者id == handler_id
                }
                _ => false,
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送準備を開始する(&target_id, handler_id).await;

        assert!(result.is_ok()); // Future ではなく Result に対して is_ok()
    }

    #[tokio::test] // #[test] -> #[tokio::test]
    async fn test_発送準備を開始する_fail_not_found() {
        // fn -> async fn
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(|_| Ok(None));
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));

        let result = service.発送準備を開始する(&target_id, handler_id).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Domain(DomainError::予約NotFound(_))
        ));
    }

    #[tokio::test]
    async fn test_発送準備を開始する_fail_invalid_state() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();

        // 不正な状態 (例: 発送済み) を返すようにモックを設定
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id.clone(),
            届け先id.clone(),
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid.clone(),
            金額.clone(),
        )
        .unwrap();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let shipped = preparing.発送を完了する("dummy-slip".to_string()).unwrap(); // 発送済み状態

        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..shipped.base
        };
        let invalid_state =
            プレゼント予約状態::発送済み(domain::発送済みプレゼント予約型 {
                base: base_with_target_id,
                配送伝票番号: shipped.配送伝票番号,
            });
        let invalid_state_clone = invalid_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(invalid_state_clone.clone())));
        mock_repo.expect_save().times(0); // save は呼ばれない

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.発送準備を開始する(&target_id, handler_id).await;

        assert!(result.is_err());
        match result.err().unwrap() {
            ApplicationError::Domain(DomainError::不正な状態遷移 { current_state_type }) => {
                assert!(current_state_type.contains("発送済み")); // 文字列に状態名が含まれるかチェック
            }
            e => panic!(
                "Expected ApplicationError::Domain(不正な状態遷移), got {:?}",
                e
            ),
        }
    }

    #[tokio::test]
    async fn test_発送準備を開始する_fail_repo_save_error() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();

        // find_by_id が返す予約受付済み状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received_reservation = domain::予約を受け付ける(
            依頼者id,
            届け先id,
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid,
            金額,
        )
        .unwrap();
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..received_reservation.base
        };
        let initial_state =
            プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 {
                base: base_with_target_id.clone(),
            });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();

        // find_by_id は成功
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save はエラーを返す
        mock_repo
            .expect_save()
            // .withf(...) // 必要に応じて引数検証を追加
            .times(1)
            .returning(|_| {
                Err(DomainError::必須項目不足 {
                    field: "DB save error".to_string(),
                })
            }); // 仮のエラー

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.発送準備を開始する(&target_id, handler_id).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Repository(_)
        ));
    }

    #[tokio::test]
    async fn test_発送準備を開始する_fail_repo_find_error() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();
        let mut mock_repo = Mockプレゼント予約Repository::new();

        // find_by_id がエラーを返す
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Err(DomainError::予約NotFound(target_id))); // 仮のエラー

        mock_repo.expect_save().times(0); // save は呼ばれない

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.発送準備を開始する(&target_id, handler_id).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Repository(_)
        ));
    }

    // --- 発送完了ユースケースのテスト ---

    #[tokio::test]
    async fn test_発送を完了する_success() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();
        let slip_number = "slip-12345".to_string();

        // find_by_id が返す発送準備中状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id.clone(),
            届け先id.clone(),
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid.clone(),
            金額.clone(),
        )
        .unwrap();
        let preparing = received.発送準備を開始する(handler_id).unwrap(); // 発送準備中状態

        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..preparing.base
        };
        let initial_state =
            プレゼント予約状態::発送準備中(domain::発送準備中プレゼント予約型 {
                base: base_with_target_id.clone(),
                梱包担当者id: preparing.梱包担当者id,
            });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();

        // find_by_id の期待値
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save の期待値 (発送済み状態になるはず)
        let expected_slip_number = slip_number.clone();
        mock_repo
            .expect_save()
            .withf(move |state: &プレゼント予約状態| match state {
                プレゼント予約状態::発送済み(ref shipped) => {
                    shipped.base == base_with_target_id
                        && shipped.配送伝票番号 == expected_slip_number
                }
                _ => false,
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.発送を完了する(&target_id, slip_number).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_発送を完了する_fail_not_found() {
        let target_id = 予約ID::new();
        let slip_number = "slip-12345".to_string();
        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(|_| Ok(None));
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.発送を完了する(&target_id, slip_number).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Domain(DomainError::予約NotFound(_))
        ));
    }

    #[tokio::test]
    async fn test_発送を完了する_fail_invalid_state() {
        let target_id = 予約ID::new();
        let slip_number = "slip-12345".to_string();

        // 不正な状態 (例: 予約受付済み) を返すようにモックを設定
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id,
            届け先id,
            記念日,
            None,
            ラッピング種類::なし,
            None,
            商品idリスト,
            支払いid,
            金額,
        )
        .unwrap(); // 予約受付済み状態

        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..received.base
        };
        let invalid_state =
            プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 {
                base: base_with_target_id,
            });
        let invalid_state_clone = invalid_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(invalid_state_clone.clone())));
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.発送を完了する(&target_id, slip_number).await;

        assert!(result.is_err());
        match result.err().unwrap() {
            ApplicationError::Domain(DomainError::不正な状態遷移 { current_state_type }) => {
                assert!(current_state_type.contains("予約受付済み"));
            }
            e => panic!(
                "Expected ApplicationError::Domain(不正な状態遷移), got {:?}",
                e
            ),
        }
    }

    #[tokio::test]
    async fn test_発送を完了する_fail_repo_save_error() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();
        let slip_number = "slip-12345".to_string();

        // find_by_id が返す発送準備中状態を作成 (上と同様)
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id.clone(),
            届け先id.clone(),
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid.clone(),
            金額.clone(),
        )
        .unwrap();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..preparing.base
        };
        let initial_state =
            プレゼント予約状態::発送準備中(domain::発送準備中プレゼント予約型 {
                base: base_with_target_id.clone(),
                梱包担当者id: preparing.梱包担当者id,
            });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));
        mock_repo.expect_save().times(1).returning(|_| {
            Err(DomainError::必須項目不足 {
                field: "DB save error".to_string(),
            })
        }); // save でエラー

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.発送を完了する(&target_id, slip_number).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Repository(_)
        ));
    }

    #[tokio::test]
    async fn test_発送を完了する_fail_repo_find_error() {
        let target_id = 予約ID::new();
        let slip_number = "slip-12345".to_string();
        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Err(DomainError::予約NotFound(target_id))); // find_by_id でエラー
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.発送を完了する(&target_id, slip_number).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Repository(_)
        ));
    }

    // --- 配送完了記録ユースケースのテスト ---

    #[tokio::test]
    async fn test_配送完了を記録する_success() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();
        let slip_number = "slip-12345".to_string();
        let delivered_at = Utc::now().with_timezone(&Tokyo);

        // find_by_id が返す発送済み状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id.clone(),
            届け先id.clone(),
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid.clone(),
            金額.clone(),
        )
        .unwrap();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let shipped = preparing.発送を完了する(slip_number.clone()).unwrap(); // 発送済み状態

        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..shipped.base
        };
        let initial_state =
            プレゼント予約状態::発送済み(domain::発送済みプレゼント予約型 {
                base: base_with_target_id.clone(),
                配送伝票番号: shipped.配送伝票番号.clone(),
            });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save の期待値 (配送完了状態になるはず)
        let expected_delivered_at = delivered_at;
        mock_repo
            .expect_save()
            .withf(move |state: &プレゼント予約状態| match state {
                プレゼント予約状態::配送完了(ref delivered) => {
                    delivered.base == base_with_target_id
                        && delivered.配送伝票番号 == slip_number
                        && delivered.配送完了日時 == expected_delivered_at
                }
                _ => false,
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.配送完了を記録する(&target_id, delivered_at).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_配送完了を記録する_fail_not_found() {
        let target_id = 予約ID::new();
        let delivered_at = Utc::now().with_timezone(&Tokyo);
        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(|_| Ok(None));
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.配送完了を記録する(&target_id, delivered_at).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Domain(DomainError::予約NotFound(_))
        ));
    }

    #[tokio::test]
    async fn test_配送完了を記録する_fail_invalid_state() {
        let target_id = 予約ID::new();
        let delivered_at = Utc::now().with_timezone(&Tokyo);
        // 不正な状態 (例: 発送準備中)
        let handler_id = ユーザーID::new();
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id.clone(),
            届け先id.clone(),
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid.clone(),
            金額.clone(),
        )
        .unwrap();
        let preparing = received.発送準備を開始する(handler_id).unwrap(); // 発送準備中状態
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..preparing.base
        };
        let invalid_state =
            プレゼント予約状態::発送準備中(domain::発送準備中プレゼント予約型 {
                base: base_with_target_id.clone(),
                梱包担当者id: preparing.梱包担当者id,
            });
        let invalid_state_clone = invalid_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(invalid_state_clone.clone())));
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.配送完了を記録する(&target_id, delivered_at).await;

        assert!(result.is_err());
        match result.err().unwrap() {
            ApplicationError::Domain(DomainError::不正な状態遷移 { current_state_type }) => {
                assert!(current_state_type.contains("発送準備中"));
            }
            e => panic!(
                "Expected ApplicationError::Domain(不正な状態遷移), got {:?}",
                e
            ),
        }
    }

    #[tokio::test]
    async fn test_配送完了を記録する_fail_repo_save_error() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();
        let slip_number = "slip-12345".to_string();
        let delivered_at = Utc::now().with_timezone(&Tokyo);

        // find_by_id が返す発送済み状態を作成 (上と同様)
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id.clone(),
            届け先id.clone(),
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid.clone(),
            金額.clone(),
        )
        .unwrap();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let shipped = preparing.発送を完了する(slip_number.clone()).unwrap();
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..shipped.base
        };
        let initial_state =
            プレゼント予約状態::発送済み(domain::発送済みプレゼント予約型 {
                base: base_with_target_id.clone(),
                配送伝票番号: shipped.配送伝票番号.clone(),
            });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));
        mock_repo.expect_save().times(1).returning(|_| {
            Err(DomainError::必須項目不足 {
                field: "DB save error".to_string(),
            })
        }); // save でエラー

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.配送完了を記録する(&target_id, delivered_at).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Repository(_)
        ));
    }

    #[tokio::test]
    async fn test_配送完了を記録する_fail_repo_find_error() {
        let target_id = 予約ID::new();
        let delivered_at = Utc::now().with_timezone(&Tokyo);
        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Err(DomainError::予約NotFound(target_id))); // find_by_id でエラー
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service.配送完了を記録する(&target_id, delivered_at).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Repository(_)
        ));
    }

    // --- 予約キャンセルユースケースのテスト ---

    #[tokio::test]
    async fn test_予約をキャンセルする_success_from_予約受付済み() {
        let target_id = 予約ID::new();
        let reason = Some("気が変わった".to_string());
        let cancelled_at = Some(Utc::now().with_timezone(&Tokyo));

        // find_by_id が返す予約受付済み状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id.clone(),
            届け先id.clone(),
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid.clone(),
            金額.clone(),
        )
        .unwrap();
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..received.base
        };
        let initial_state =
            プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 {
                base: base_with_target_id.clone(),
            });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save の期待値 (キャンセル済み状態になるはず)
        let expected_reason = reason.clone();
        let expected_cancelled_at = cancelled_at;
        mock_repo
            .expect_save()
            .withf(move |state: &プレゼント予約状態| match state {
                プレゼント予約状態::キャンセル済み(ref cancelled) => {
                    cancelled.base == base_with_target_id &&
                    cancelled.キャンセル理由 == expected_reason && // 理由 -> キャンセル理由
                    cancelled.キャンセル日時 == expected_cancelled_at
                }
                _ => false,
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service
            .予約をキャンセルする(&target_id, reason, cancelled_at)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_予約をキャンセルする_success_from_発送準備中() {
        let target_id = 予約ID::new();
        let handler_id = ユーザーID::new();
        let reason = Some("準備中に問題発生".to_string());
        let cancelled_at = Some(Utc::now().with_timezone(&Tokyo));

        // find_by_id が返す発送準備中状態を作成
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id.clone(),
            届け先id.clone(),
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid.clone(),
            金額.clone(),
        )
        .unwrap();
        let preparing = received.発送準備を開始する(handler_id).unwrap(); // 発送準備中状態
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..preparing.base
        };
        let initial_state =
            プレゼント予約状態::発送準備中(domain::発送準備中プレゼント予約型 {
                base: base_with_target_id.clone(),
                梱包担当者id: preparing.梱包担当者id,
            });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));

        // save の期待値 (キャンセル済み状態になるはず)
        let expected_reason = reason.clone();
        let expected_cancelled_at = cancelled_at;
        mock_repo
            .expect_save()
            .withf(move |state: &プレゼント予約状態| match state {
                プレゼント予約状態::キャンセル済み(ref cancelled) => {
                    cancelled.base == base_with_target_id &&
                    cancelled.キャンセル理由 == expected_reason && // 理由 -> キャンセル理由
                    cancelled.キャンセル日時 == expected_cancelled_at
                }
                _ => false,
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service
            .予約をキャンセルする(&target_id, reason, cancelled_at)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_予約をキャンセルする_fail_not_found() {
        let target_id = 予約ID::new();
        let reason = None;
        let cancelled_at = None;
        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(|_| Ok(None));
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service
            .予約をキャンセルする(&target_id, reason, cancelled_at)
            .await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Domain(DomainError::予約NotFound(_))
        ));
    }

    #[tokio::test]
    async fn test_予約をキャンセルする_fail_invalid_state() {
        let target_id = 予約ID::new();
        let reason = None;
        let cancelled_at = None;
        // 不正な状態 (例: 発送済み)
        let handler_id = ユーザーID::new();
        let slip_number = "slip-123".to_string();
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id.clone(),
            届け先id.clone(),
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid.clone(),
            金額.clone(),
        )
        .unwrap();
        let preparing = received.発送準備を開始する(handler_id).unwrap();
        let shipped = preparing.発送を完了する(slip_number.clone()).unwrap(); // 発送済み
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..shipped.base
        };
        let invalid_state =
            プレゼント予約状態::発送済み(domain::発送済みプレゼント予約型 {
                base: base_with_target_id.clone(),
                配送伝票番号: shipped.配送伝票番号,
            });
        let invalid_state_clone = invalid_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(invalid_state_clone.clone())));
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service
            .予約をキャンセルする(&target_id, reason, cancelled_at)
            .await;

        assert!(result.is_err());
        match result.err().unwrap() {
            ApplicationError::Domain(DomainError::不正な状態遷移 { current_state_type }) => {
                assert!(current_state_type.contains("発送済み"));
            }
            e => panic!(
                "Expected ApplicationError::Domain(不正な状態遷移), got {:?}",
                e
            ),
        }
    }

    #[tokio::test]
    async fn test_予約をキャンセルする_fail_repo_save_error() {
        let target_id = 予約ID::new();
        let reason = None;
        let cancelled_at = None;

        // find_by_id が返す予約受付済み状態を作成 (上と同様)
        let (依頼者id, 届け先id, 支払いid, 商品idリスト) = create_dummy_ids();
        let 記念日 = create_dummy_kinenbi();
        let 金額 = create_dummy_kingaku();
        let received = domain::予約を受け付ける(
            依頼者id.clone(),
            届け先id.clone(),
            記念日.clone(),
            None,
            ラッピング種類::なし,
            None,
            商品idリスト.clone(),
            支払いid.clone(),
            金額.clone(),
        )
        .unwrap();
        let base_with_target_id = domain::プレゼント予約ベース {
            id: target_id.clone(),
            ..received.base
        };
        let initial_state =
            プレゼント予約状態::予約受付済み(domain::予約受付済みプレゼント予約型 {
                base: base_with_target_id.clone(),
            });
        let initial_state_clone = initial_state.clone();

        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(Some(initial_state_clone.clone())));
        mock_repo.expect_save().times(1).returning(|_| {
            Err(DomainError::必須項目不足 {
                field: "DB save error".to_string(),
            })
        }); // save でエラー

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service
            .予約をキャンセルする(&target_id, reason, cancelled_at)
            .await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Repository(_)
        ));
    }

    #[tokio::test]
    async fn test_予約をキャンセルする_fail_repo_find_error() {
        let target_id = 予約ID::new();
        let reason = None;
        let cancelled_at = None;
        let mut mock_repo = Mockプレゼント予約Repository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Err(DomainError::予約NotFound(target_id))); // find_by_id でエラー
        mock_repo.expect_save().times(0);

        let service = プレゼント予約サービス::new(Arc::new(mock_repo));
        let result = service
            .予約をキャンセルする(&target_id, reason, cancelled_at)
            .await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ApplicationError::Repository(_)
        ));
    }
}
