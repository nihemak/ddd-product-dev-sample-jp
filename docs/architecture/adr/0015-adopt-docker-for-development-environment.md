# ADR 0015: 開発環境に Docker Compose を採用

*   **Status**: Accepted
*   **Date**: 2025-04-20
*   **Deciders**: (議論参加者)

## Context and Problem Statement

バックエンド (Rust) および依存サービス (PostgreSQL) の開発環境を構築する必要がある。ローカルマシンへの直接インストールは環境汚染やバージョン競合、環境再現性の問題を引き起こす可能性がある。ポータブルで再現性があり、ローカル環境を汚さない開発環境構築方法を選定したい。

## Decision Drivers

*   **環境分離**: ローカルマシン環境への影響を最小限に抑えたい。
*   **再現性**: 異なる開発者やCI/CD環境でも同じ開発環境を容易に構築できるようにしたい。
*   **依存関係管理**: データベース等のミドルウェアのバージョンを固定・管理したい。
*   **セットアップ容易性**: 新しい開発者が容易に開発を開始できるようにしたい。

## Considered Options

1.  **Docker Compose**: 複数のDockerコンテナ（バックエンド、DB）を定義し、一括で管理・起動するツール。
    *   Pros: 環境構成をコード化 (`docker-compose.yml`) できる、複数サービスの連携が容易、再現性が高い、環境分離が可能、広く使われている。
    *   Cons: Dockerの基本的な知識が必要、若干のリソース（CPU/メモリ）を消費する。
2.  **ローカルマシンへの直接インストール**: PostgreSQLやRustツールチェインなどを直接ローカルマシンにインストールする。
    *   Pros: Dockerの知識不要、セットアップが単純な場合もある。
    *   Cons: 環境汚染のリスク、バージョン競合、他のプロジェクトとの環境分離が困難、再現性の担保が難しい。
3.  **仮想マシン (Vagrantなど)**: 仮想マシン上に開発環境を構築する。
    *   Pros: 完全な環境分離。
    *   Cons: Dockerよりリソース消費が大きい、起動・操作が比較的遅い、設定が複雑になる場合がある。

## Decision Outcome

**選択肢1「Docker Compose」を採用する。**

理由:
*   **環境分離と再現性**: `docker-compose.yml` と `Dockerfile` によって開発環境（Rustツールチェイン、PostgreSQLバージョン等）をコードとして定義でき、ローカル環境を汚さずに誰でも同じ環境を再現できる。
*   **依存関係の管理**: PostgreSQLなどのサービスとそのバージョンを簡単に定義・管理できる。
*   **複数サービスの連携**: バックエンドサービスとデータベースサービス間のネットワーク接続や依存関係を容易に設定できる。
*   **業界標準**: コンテナ技術とDocker Composeは現代の開発において広く普及しており、学習コストに見合う価値がある。

**具体的な構成:**
*   プロジェクトルートに `docker-compose.yml` を配置。
*   `backend` ディレクトリに `Dockerfile` を配置。
*   `docker-compose.yml` で `backend` (Rustアプリ) と `db` (PostgreSQL) サービスを定義。
*   開発時のホットリロードのためにボリュームマウントを利用。
*   DBデータ永続化のために名前付きボリュームを利用。

## Consequences

### Positive:
*   ローカル環境をクリーンに保てる。
*   開発環境のセットアップが `docker compose up` コマンドで容易になる。
*   環境差異による問題を削減できる。
*   本番環境に近い構成での開発・テストが可能になる場合がある。

### Negative:
*   DockerおよびDocker Composeの基本的な知識が必要となる。
*   Dockerデーモンの実行により、ローカルマシンのリソース（CPU、メモリ、ディスク）を消費する。
*   初期のDockerfileやdocker-compose.ymlの作成・設定に手間がかかる。

## References

*   [Overview of Docker Compose](https://docs.docker.com/compose/)
*   `docker-compose.yml`
*   `backend/Dockerfile`
*   ADR 0007: データストアとして PostgreSQL を採用
*   ADR 0014: シンプルなモノレポ構成を採用 