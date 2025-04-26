# 記念日プレゼント予約 ER図

```mermaid
erDiagram
    "予約テーブル (reservations)" {
        UUID id PK
        UUID requester_id "依頼者ID"
        UUID recipient_id "届け先ID"
        DATE anniversary_date "記念日"
        TEXT message "メッセージ内容 (NULL可)"
        VARCHAR(50) wrapping_type "ラッピング種類"
        TIMESTAMPTZ desired_delivery_date "配送希望日時 (NULL可)"
        INTEGER total_amount "合計金額 (0より大きい)"
        UUID payment_id "支払いID"
        VARCHAR(50) status "予約ステータス"
        UUID preparation_staff_id "梱包担当者ID (NULL可)"
        VARCHAR(255) shipping_slip_number "配送伝票番号 (NULL可)"
        TIMESTAMPTZ delivery_completed_at "配送完了日時 (NULL可)"
        TEXT cancellation_reason "キャンセル理由 (NULL可)"
        TIMESTAMPTZ cancelled_at "キャンセル日時 (NULL可)"
        TIMESTAMPTZ created_at "作成日時"
        TIMESTAMPTZ updated_at "更新日時"
    }

    "予約商品テーブル (reservation_products)" {
        UUID reservation_id PK, FK "予約ID"
        UUID product_id PK "商品ID"
    }

    "予約テーブル (reservations)" ||--o{ "予約商品テーブル (reservation_products)" : "含む"
```

**注記:**

*   この図は記念日プレゼント予約サービスの主要なテーブルとその関連を示します。
*   テーブル名は「日本語名 (実テーブル名)」の形式で表記しています。
*   外部キー関係は `FK` で示されます。
*   主キーは `PK` で示されます。
*   カラムの日本語名は、ドメインモデルやユビキタス言語に対応するものです。
*   NULL許容カラムには注釈で「(NULL可)」と記載しています。
*   `予約商品テーブル` は予約と商品の多対多関係を表します。
*   データ型、CHECK制約、デフォルト値、インデックスなどの詳細な定義は `schema.sql` に記載されています。 