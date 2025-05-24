# 画面フローと構造

このドキュメントは、アプリケーション全体の主要な画面構成と画面間の遷移フローを管理します。

## 主要画面遷移図 (MVP時点)

```mermaid
graph TD
    subgraph "認証フロー"
        direction LR
        UnauthenticatedTop["トップページ（未ログイン時）"] -- "新規登録" --> SignupPage["ユーザー登録ページ"];
        UnauthenticatedTop -- "ログイン" --> LoginPage["ログインページ"];
        SignupPage -- "登録成功" --> LoginPage;
        LoginPage -- "ログイン成功" --> AuthenticatedTop["ダッシュボード/トップ（認証済）"];
        %% AuthenticatedTopはMVP以降の画面へ
    end

    %% TODO: ログイン機能実装後に更新
    subgraph MVP
        %% 既存のMVPフローは残しつつ、認証フローと接続するイメージ
        direction LR
        AuthenticatedTop --> B(プレゼント予約作成画面);
        B --> C(予約完了画面?);
        AuthenticatedTop --> D(届け先一覧?);
        D --> E(届け先登録画面?);
    end

    %% 画面仕様へのリンク (例)
    click UnauthenticatedTop "./unauthenticated_top_page.md" "トップページ（未ログイン時） 仕様"
    click SignupPage "./signup_page.md" "ユーザー登録ページ 仕様"
    click LoginPage "./login_page.md" "ログインページ 仕様"
    click B "./present-reservation-shipping-proxy.md" "プレゼント予約作成 仕様"
    %% 他画面の仕様作成後にリンクを追加
```

## 主要画面一覧 (随時更新)

* [トップページ（未ログイン時）](./unauthenticated_top_page.md)
* [ユーザー登録ページ](./signup_page.md)
* [ログインページ](./login_page.md)
* [プレゼント予約作成（発送代行）](./present-reservation-shipping-proxy.md)
* (ダッシュボード/予約一覧画面)
* (予約完了画面)
* (届け先一覧画面)
* (届け先登録画面)
* ...

---
*最終更新日: 2025-05-04*
