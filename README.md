# 📚 Inoreader for Rust

**Inoreader API を使用してスター付き記事を取得・管理する Rust CLI アプリケーション**

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## 📋 概要

Inoreader for Rust は、[Inoreader](https://www.inoreader.com/) の API を活用して、スター付きの記事を効率的に取得し、CSV形式で出力するコマンドラインツールです。OAuth2認証フローを完全サポートし、トークンの自動リフレッシュ機能も備えています。

### ✨ 主な機能

- 🔐 **OAuth2 認証システム**: 安全な API アクセス
- 🔄 **自動トークンリフレッシュ**: 長期間の利用に対応
- ⭐ **スター付き記事取得**: 効率的なページネーション
- 📊 **CSV 出力**: データ分析・管理に最適
- 🏗️ **クリーンアーキテクチャ**: 保守性の高い設計

---

## 🚀 クイックスタート

### 前提条件

- **Rust**: 1.70.0 以上
- **Inoreader アカウント**: [無料登録](https://www.inoreader.com/)
- **Inoreader API キー**: [開発者設定](https://www.inoreader.com/developers/oauth)

### インストール

```bash
# リポジトリをクローン
git clone https://github.com/miko3/inoreader_for_rust.git
cd inoreader_for_rust

# ビルド
cargo build --release
```

### 環境設定

1. `.env` ファイルを作成:

```bash
# Inoreader API 設定
INOREADER_CLIENT_ID=your_client_id
INOREADER_CLIENT_SECRET=your_client_secret
INOREADER_REDIRECT_URI=http://localhost:8080/callback
```

2. Inoreader 開発者設定で以下を設定:
   - **Redirect URI**: `http://localhost:8080/callback`
   - **Scope**: `read`

---

## 📖 使用方法

### 🔑 初回認証

```bash
# OAuth2 認証フローを開始
cargo run setup
```

**認証手順:**
1. 表示される URL にアクセス
2. Inoreader にログイン・認可
3. リダイレクト先 URL から認証コードを取得
4. コンソールに認証コードを入力

### 📥 記事取得

```bash
# スター付き記事を取得してCSVに出力
cargo run fetch_stream
```

**出力ファイル:** `articles.csv`
```csv
title,url
"記事タイトル1","https://example.com/article1"
"記事タイトル2","https://example.com/article2"
```

---

## 🏗️ アーキテクチャ

### レイヤー構造

```
src/
├── domain.rs                 # データモデル定義
├── infrastructure.rs         # HTTP通信・外部API
├── services/                 # ビジネスロジック
│   ├── authentication_service.rs
│   ├── token_service.rs
│   └── response_parser_service.rs
├── repositories/            # データ永続化
│   ├── config_repository.rs
│   └── articles_data_repository.rs
└── main.rs                  # エントリーポイント
```

### 設計原則

- 🔄 **依存性逆転**: インターフェース駆動設計
- 🧩 **関心の分離**: レイヤーごとの責務明確化
- 🛡️ **型安全性**: Rustの強力な型システム活用
- ♻️ **再利用性**: モジュール化された設計

---

## 🔧 開発

### 🚀 クイック開発セットアップ

```bash
# 開発環境セットアップ（初回のみ）
make dev-setup

# 開発ツールインストール
make install

# 全品質チェック実行
make ci
```

### 📋 Makeコマンド

```bash
# 基本コマンド
make build              # ビルド
make test               # テスト実行
make run-setup          # 初回認証
make run-fetch          # 記事取得

# TDD開発
make tdd                # TDD監視モード (ファイル変更時自動テスト)
make test-watch         # テスト監視モード
make test-unit          # ユニットテストのみ
make test-integration   # 統合テストのみ
make test-coverage      # カバレッジレポート生成
make test-nextest       # 高速テスト実行

# 品質チェック
make format             # コードフォーマット
make lint               # Clippy lint
make quality            # 全品質チェック

# セキュリティ
make audit              # 脆弱性監査
make deps-check         # 依存関係チェック
make security           # 全セキュリティチェック

# CI/CD
make ci                 # 全チェック (品質+セキュリティ)
make pre-commit         # コミット前チェック
make release-check      # リリース準備チェック
```

### 🧪 テスト実行

```bash
# 全テスト実行
make test

# 詳細出力
make test-verbose

# 特定のテスト
cargo test test_parse_response
```

### 🛡️ セキュリティ・品質管理

#### 自動セキュリティチェック

```bash
# セキュリティ監査（脆弱性検出）
make audit

# 依存関係ライセンス・バージョンチェック
make deps-check

# 古い依存関係確認
make deps-update
```

#### コード品質チェック

```bash
# リント（警告をエラーとして扱う）
make lint

# フォーマットチェック
make format-check

# 全品質チェック
make quality
```

#### CI/CD自動化

- **GitHub Actions**: `.github/workflows/security-audit.yml`
  - 🔒 毎日自動セキュリティ監査
  - 📦 依存関係チェック
  - 🧹 コード品質チェック
  - 🔄 PR時の自動実行

#### セキュリティポリシー

- **依存関係管理**: `deny.toml` で設定
  - ❌ **脆弱性のあるパッケージ拒否**
  - ⚠️ **メンテナンス停止パッケージ警告**
  - 📄 **ライセンス互換性チェック**
  - 🔄 **重複依存関係警告**

- **対応済みセキュリティ改善**:
  - ✅ `dotenv` → `dotenvy` (RUSTSEC-2021-0141対応)
  - ✅ `reqwest` 0.11 → 0.12 (最新版使用)
  - ✅ 自動脆弱性監視体制

#### 🔄 TDD (テスト駆動開発) 強制

- **開発方針**: **TDD必須** - 全新機能はテストファースト
- **自動化**: GitHub Actions でTDD遵守をチェック
- **カバレッジ**: 80%以上必須、新機能は100%
- **支援ツール**: `cargo-watch`、`cargo-tarpaulin`、`cargo-nextest`

```bash
# TDD開発サイクル
make tdd                 # 監視モードでTDD開発
make test-coverage       # カバレッジ確認
```

**TDD違反時のCI失敗**:
- ❌ 新機能にテストがない
- ❌ カバレッジが80%未満
- ❌ テストが失敗している

### デバッグ

```bash
# デバッグビルド
cargo run -- setup

# ログレベル設定
RUST_LOG=debug cargo run fetch_stream
```

---

## 📊 API 仕様

### Inoreader API エンドポイント

| エンドポイント | 説明 | 実装状況 |
|---|---|---|
| `/oauth2/auth` | OAuth2 認証 | ✅ |
| `/oauth2/token` | トークン取得・更新 | ✅ |
| `/stream/contents` | 記事ストリーム取得 | ✅ |

### 取得データ

- **スター付き記事**: `user/-/state/com.google/starred`
- **ページネーション**: 100件/ページ（最大10ページ）
- **フィールド**: タイトル、URL、メタデータ

---

## 🔒 セキュリティ

### 認証・認可

- 🔐 **OAuth2 フロー**: 安全な認証
- 🎫 **JWT トークン**: ステートレス認証
- 🔄 **自動リフレッシュ**: トークン期限管理
- 💾 **ローカル保存**: `.config` ファイル（平文）

### セキュリティ注意事項

⚠️ **重要**: `.config` ファイルには認証トークンが平文で保存されます。
- ファイルのパーミッション管理
- `.gitignore` への追加
- 本番環境での暗号化検討

---

## 🛠️ トラブルシューティング

### よくある問題

**❌ "Token file not found"**
```bash
# 解決方法: 初回認証を実行
cargo run setup
```

**❌ "Authentication failed"**
```bash
# .env ファイルの設定確認
cat .env

# API キーの再確認
# Inoreader 開発者設定を確認
```

**❌ "Failed to fetch contents"**
```bash
# トークン期限確認
cat .config

# 手動でトークン更新
rm .config
cargo run setup
```

### ログ確認

```bash
# 詳細ログ
RUST_LOG=debug cargo run fetch_stream

# エラー詳細
RUST_BACKTRACE=1 cargo run fetch_stream
```

---

## 🤝 貢献

### 開発への参加

1. **フォーク**: リポジトリをフォーク
2. **ブランチ**: `git checkout -b feature/new-feature`
3. **コミット**: `git commit -m "Add new feature"`
4. **プッシュ**: `git push origin feature/new-feature`
5. **プルリクエスト**: GitHub でプルリクエスト作成

### コーディング規約

- 🦀 **Rust標準**: `cargo fmt` + `cargo clippy`
- 📝 **コメント**: パブリック API にドキュメントコメント
- 🧪 **テスト**: 新機能には必ずテスト追加
- 📋 **コミット**: [Conventional Commits](https://www.conventionalcommits.org/)

---

## 📄 ライセンス

MIT License - 詳細は [LICENSE](LICENSE) ファイルを参照

---

## 🔗 関連リンク

- 📚 **Inoreader API**: [公式ドキュメント](https://www.inoreader.com/developers/)
- 🦀 **Rust**: [公式サイト](https://www.rust-lang.org/)
- 📖 **Rust Book**: [The Rust Programming Language](https://doc.rust-lang.org/book/)
- 🛠️ **Cargo**: [Cargo Book](https://doc.rust-lang.org/cargo/)

---

## 📈 ロードマップ

### v0.2.0 (予定)
- [ ] 🔍 記事検索機能
- [ ] 📱 複数出力形式（JSON、YAML）
- [ ] 🔔 通知機能

### v0.3.0 (予定)
- [ ] 🌐 Web UI インターフェース
- [ ] 📊 記事分析機能
- [ ] 🔄 自動同期機能

---

**⭐ このプロジェクトが役に立ったら、ぜひスターを付けてください！**