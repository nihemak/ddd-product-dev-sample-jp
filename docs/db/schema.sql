-- Base schema for the Anniversary Gift Reservation Service

-- reservations テーブル: 予約の基本情報と状態、状態固有の情報を格納
CREATE TABLE reservations (
    id UUID PRIMARY KEY, -- 予約ID
    requester_id UUID NOT NULL, -- 依頼者ID
    recipient_id UUID NOT NULL, -- 届け先ID
    anniversary_date DATE NOT NULL, -- 記念日
    message TEXT, -- メッセージ内容 (NULL可)
    wrapping_type VARCHAR(50) NOT NULL, -- ラッピング種類
    desired_delivery_date TIMESTAMPTZ, -- 配送希望日時 (NULL可)
    total_amount INTEGER NOT NULL CHECK (total_amount > 0), -- 合計金額 (0より大きい)
    payment_id UUID NOT NULL, -- 支払いID
    status VARCHAR(50) NOT NULL, -- 予約ステータス (例: "Received", "Preparing", "Shipped", "Delivered", "Cancelled")
    preparation_staff_id UUID, -- 梱包担当者ID (発送準備中の担当者ID, NULL可)
    shipping_slip_number VARCHAR(255), -- 配送伝票番号 (発送済みの伝票番号, NULL可)
    delivery_completed_at TIMESTAMPTZ, -- 配送完了日時 (NULL可)
    cancellation_reason TEXT, -- キャンセル理由 (NULL可)
    cancelled_at TIMESTAMPTZ, -- キャンセル日時 (NULL可)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- 作成日時
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW() -- 更新日時
);

-- reservation_products テーブル: 予約と商品の関連 (多対多)
CREATE TABLE reservation_products (
    reservation_id UUID NOT NULL REFERENCES reservations(id) ON DELETE CASCADE, -- 予約ID (FK)
    product_id UUID NOT NULL, -- 商品ID
    PRIMARY KEY (reservation_id, product_id)
);

-- updated_at トリガー用の関数
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- reservations テーブルに updated_at トリガーを設定
CREATE TRIGGER set_timestamp_reservations
BEFORE UPDATE ON reservations
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

-- インデックス (必要に応じてコメント解除または追加)
-- CREATE INDEX idx_reservations_requester_id ON reservations(requester_id);
-- CREATE INDEX idx_reservations_status ON reservations(status);
-- CREATE INDEX idx_reservation_products_product_id ON reservation_products(product_id); 