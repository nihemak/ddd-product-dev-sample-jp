# ユビキタス言語集

このドキュメントは、プロジェクト内で使用される主要なドメイン用語（ユビキタス言語）を定義します。
コード (`src/domain.rs` など) と一貫性を保つようにしてください。

## エンティティ (Entity)

- **注文 (Order)**
  - 説明: 顧客からの商品の注文を表すエンティティ。注文ID、商品リスト、合計金額、注文状態を持つ。
  - 対応コード: `struct 注文`
- **商品 (Item)**
  - 説明: 販売される商品を表すエンティティ。商品ID、名前、価格を持つ。
  - 対応コード: `struct 商品`

## 値オブジェクト (Value Object)

- **注文ID (OrderID)**
  - 説明: 注文を一意に識別するID。UUIDに基づいています。
  - 対応コード: `struct 注文ID(Uuid)`
- **商品ID (ItemID)**
  - 説明: 商品を一意に識別するID。UUIDに基づいています。
  - 対応コード: `struct 商品ID(Uuid)`
- **注文状態 (OrderStatus)**
  - 説明: 注文が現在どの段階にあるかを示す状態。
  - 取りうる値:
    - `受付済み (Received)`
    - `発送準備中 (Preparing)`
    - `発送済み (Shipped)`
    - `キャンセル済み (Cancelled)`
  - 対応コード: `enum 注文状態`

## ドメインサービス / ロジック関数 (Domain Logic)

- **合計金額計算 (calculate_total_price)**
  - 説明: 商品IDリストと価格取得関数を受け取り、注文の合計金額を計算する。
  - 対応コード: `fn 合計金額計算`
- **注文作成 (create_order)**
  - 説明: 商品IDリストと価格取得関数を受け取り、新しい注文エンティティを作成する（状態は「受付済み」）。
  - 対応コード: `fn 注文作成`
- **発送準備中へ変更 (mark_as_preparing)**
  - 説明: 注文の状態を「受付済み」から「発送準備中」へ変更する。
  - 対応コード: `fn 発送準備中へ変更`
- **発送済みへ変更 (mark_as_shipped)**
  - 説明: 注文の状態を「発送準備中」から「発送済み」へ変更する。
  - 対応コード: `fn 発送済みへ変更`
- **注文キャンセル (cancel_order)**
  - 説明: 注文の状態を「キャンセル済み」に変更する（発送済みでなければ）。
  - 対応コード: `fn 注文キャンセル`

## ドメインエラー (Domain Error)

- **DomainError**
  - 説明: ドメインルール違反や不整合が発生した場合のエラー。
  - 対応コード: `enum DomainError`
  - バリアント:
    - `注文商品空エラー (OrderHasNoItems)`: 注文に商品が一つも含まれていない。
    - `不正な状態遷移エラー (InvalidStateTransition)`: 許可されていない状態遷移を行おうとした。
    - `注文NotFound (OrderNotFound)`: 指定された注文IDの注文が見つからない。
    - `商品NotFound (ItemNotFound)`: 指定された商品IDの商品が見つからない。
    - `価格計算オーバーフロー (PriceCalculationOverflow)`: 合計金額計算中にオーバーフローが発生した。

## リポジトリインターフェース (Repository Interface)

- **注文Repository (OrderRepository)**
  - 説明: 注文エンティティの永続化（保存、検索）操作を定義するインターフェース（トレイト）。
  - 対応コード: `trait 注文Repository`
- **商品Repository (ItemRepository)**
  - 説明: 商品エンティティの検索操作を定義するインターフェース（トレイト）。
  - 対応コード: `trait 商品Repository`

## アプリケーション層の用語 (Application Layer Terms)

- **注文サービス (OrderService)**
  - 説明: 注文に関するユースケース（注文受付、発送準備など）を実装するサービスクラス。ドメインオブジェクトやリポジトリを操作する。
  - 対応コード: `struct 注文サービス`
- **ApplicationError**
  - 説明: アプリケーション層で発生するエラー（ドメインエラーのラップ、リポジトリエラーなど）。
  - 対応コード: `enum ApplicationError`
