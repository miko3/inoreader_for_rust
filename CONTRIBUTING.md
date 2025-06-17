# 🤝 Contributing to Inoreader for Rust

## 🎯 開発方針

このプロジェクトは**セキュリティファースト**と**品質重視**の開発方針を採用しています。

---

## 🛡️ セキュリティ・品質管理ルール

### 必須チェック項目

すべてのコントリビューションは以下の品質基準を満たす必要があります：

#### 🔒 セキュリティチェック

```bash
# 1. 脆弱性監査
make audit

# 2. 依存関係セキュリティチェック
make deps-check

# 3. 古い依存関係確認
make deps-update
```

**🚨 重要**: 脆弱性が検出された場合は**必ず修正**してからプルリクエストを作成してください。

#### 🧹 コード品質チェック

```bash
# 1. フォーマットチェック
make format-check

# 2. Clippy lint (警告をエラーとして扱う)
make lint

# 3. テスト実行
make test
```

#### 🔄 CI/CD パイプライン

```bash
# 全チェックを一括実行
make ci

# コミット前チェック
make pre-commit

# リリース準備チェック
make release-check
```

---

## 📋 開発ワークフロー

### 1. 環境セットアップ

```bash
# プロジェクトクローン
git clone https://github.com/miko3/inoreader_for_rust.git
cd inoreader_for_rust

# 開発環境セットアップ
make dev-setup
```

### 2. 開発・テストサイクル

```bash
# コード変更後の品質チェック
make quality

# セキュリティチェック
make security

# 全体チェック
make ci
```

### 3. プルリクエスト作成

1. **ブランチ作成**: `git checkout -b feature/your-feature`
2. **品質チェック**: `make ci` をパス
3. **コミット**: 明確なコミットメッセージ
4. **プッシュ**: `git push origin feature/your-feature`
5. **PR作成**: GitHub でプルリクエスト作成

---

## 🔧 依存関係管理ポリシー

### 新しい依存関係追加時の確認事項

1. **セキュリティ確認**
   ```bash
   # 追加後に必ずチェック
   make audit
   make deps-check
   ```

2. **ライセンス互換性**
   - MIT/Apache-2.0/BSD系のみ許可
   - GPL系はプロジェクトポリシーにより**禁止**

3. **メンテナンス状況**
   - 活発にメンテナンスされているプロジェクトを選択
   - 最終コミットが1年以内であることを確認

4. **最小依存原則**
   - 必要最小限の依存関係のみ追加
   - 機能重複する依存関係は避ける

### 既知のセキュリティ問題への対応

- **dotenv → dotenvy**: RUSTSEC-2021-0141 (unmaintained) 対応済み
- **reqwest 0.11 → 0.12**: 最新版使用でセキュリティ強化

---

## 🧪 TDD (テスト駆動開発) 要件

### 🔄 TDD開発サイクル

このプロジェクトでは **TDD (Test-Driven Development)** を必須とします：

#### 基本TDDサイクル: Red → Green → Refactor

```bash
# 1. Red: 失敗するテストを書く
cargo test test_new_feature  # ❌ 失敗

# 2. Green: テストをパスする最小限のコードを書く
cargo test test_new_feature  # ✅ 成功

# 3. Refactor: コードを改善する
make lint                    # 品質チェック
cargo test                   # 全テスト確認
```

### 📋 機能別TDD要件

#### 🆕 新機能開発時

```bash
# 必須手順:
# 1. テストファイル作成
# 2. 失敗するテスト記述
# 3. cargo test で失敗確認
# 4. 実装コード追加
# 5. cargo test で成功確認
# 6. リファクタリング・品質向上
```

**例: 新しいサービス機能追加**
```rust
// 1. tests/新機能テスト作成
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_service_functionality() {
        // Given: テスト条件設定
        let service = NewService::new();
        
        // When: 機能実行
        let result = service.execute();
        
        // Then: 期待結果検証
        assert_eq!(result, expected_value);
    }
}

// 2. 実装前にテスト実行 → 失敗確認
// 3. 実装コード追加
// 4. テスト実行 → 成功確認
```

#### 🐛 バグ修正時

```bash
# 必須手順:
# 1. バグ再現テスト作成
# 2. cargo test で失敗確認（バグ再現）
# 3. バグ修正コード実装
# 4. cargo test で成功確認
# 5. 回帰テスト実行
```

#### 🔄 リファクタリング時

```bash
# 必須手順:
# 1. 既存テスト全パス確認
# 2. リファクタリング実施
# 3. テスト実行で機能保持確認
# 4. 新しいテストケース追加（必要に応じて）
```

### 🎯 テストカバレッジ基準

- **新機能**: 100% テストカバレッジ必須
- **バグ修正**: 必ず再現テスト追加
- **リファクタリング**: 既存テスト全パス + 追加テスト
- **API変更**: 統合テスト更新必須

### 📂 テスト構成

```
src/
├── services/
│   ├── authentication_service.rs
│   └── mod.rs
└── repositories/
    ├── config_repository.rs
    └── mod.rs

# テストは各モジュール内に配置
src/services/authentication_service.rs:
  #[cfg(test)]
  mod tests {
      // ユニットテスト
  }

# 統合テストは別ディレクトリ
tests/
├── integration_tests.rs
├── api_tests.rs
└── end_to_end_tests.rs
```

### 🧪 テストタイプ

#### 1. ユニットテスト
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_with_valid_input() {
        // 正常系テスト
    }

    #[test]
    fn test_function_with_invalid_input() {
        // 異常系テスト
    }

    #[test]
    fn test_function_edge_cases() {
        // 境界値テスト
    }
}
```

#### 2. 統合テスト
```rust
// tests/integration_test.rs
use inoreader_house_cleaning::*;

#[tokio::test]
async fn test_complete_workflow() {
    // エンドツーエンドのワークフローテスト
}
```

#### 3. モックテスト
```rust
#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        HttpClient {}
        
        impl HttpClientTrait for HttpClient {
            async fn get(&self, url: &str) -> Result<String, Error>;
        }
    }

    #[tokio::test]
    async fn test_with_mock() {
        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_get()
            .with(eq("test_url"))
            .times(1)
            .returning(|_| Ok("mock response".to_string()));
        
        // テスト実行
    }
}
```

### 🚀 TDD実行コマンド

```bash
# TDD開発サイクル用コマンド
make tdd              # ファイル監視モードでテスト自動実行
make test-watch       # テスト監視モード
make test-coverage    # カバレッジレポート生成

# テスト実行
make test             # 全テスト実行
make test-unit        # ユニットテストのみ
make test-integration # 統合テストのみ
make test-verbose     # 詳細出力

# 特定のテスト
cargo test test_authentication  # 認証関連テスト
cargo test -- --nocapture      # 出力表示付き
```

### 📊 テスト品質基準

#### ✅ 必須要件
- **全新機能**: テストファースト開発
- **PRマージ**: 全テストパス必須
- **テストカバレッジ**: 新機能は100%
- **テスト実行時間**: 30秒以内

#### 🎯 テスト設計原則
- **FIRST原則**:
  - **Fast**: 高速実行
  - **Independent**: 独立性
  - **Repeatable**: 再現可能
  - **Self-Validating**: 自己検証
  - **Timely**: タイムリー

- **Given-When-Then** パターン:
```rust
#[test]
fn test_example() {
    // Given: 事前条件
    let input = "test_data";
    
    // When: 実行
    let result = function_under_test(input);
    
    // Then: 結果検証
    assert_eq!(result, expected_output);
}
```

### 🚫 TDD違反事項

#### ❌ 禁止行為
- **テストなしの新機能実装**
- **テスト後追いコミット**
- **失敗テストの無視・スキップ**
- **テストカバレッジの低下**

#### 🚨 PR拒否条件
- 新機能にテストがない
- 既存テストが失敗している
- テストカバレッジが基準未満
- TDDサイクルを無視した開発

### 🛠️ TDD支援ツール

```bash
# 開発支援ツールのインストール
cargo install cargo-watch     # ファイル監視
cargo install cargo-tarpaulin # カバレッジ計測
cargo install cargo-nextest   # 高速テスト実行
```

---

## 📝 コーディング規約

### Rustコーディングスタイル

```bash
# 自動フォーマット適用
make format

# Clippy lint適用
make lint
```

### コミットメッセージ規約

[Conventional Commits](https://www.conventionalcommits.org/) を採用：

```
feat: 新機能追加
fix: バグ修正
docs: ドキュメント更新
style: フォーマット変更
refactor: リファクタリング
test: テスト追加・修正
chore: その他の変更
security: セキュリティ関連の修正
```

**例**:
```
feat: OAuth2トークンリフレッシュ機能追加
fix: 記事取得時のページネーション不具合修正
security: dotenvからdotenvyへ移行 (RUSTSEC-2021-0141対応)
```

---

## 🚀 リリースプロセス

### リリース前チェックリスト

```bash
# 1. 全チェック実行
make release-check

# 2. 手動テスト
make run-setup
make run-fetch

# 3. ドキュメント更新確認
make docs
```

### セマンティックバージョニング

- **MAJOR**: 破壊的変更
- **MINOR**: 新機能追加（下位互換性あり）
- **PATCH**: バグ修正

---

## 🆘 トラブルシューティング

### よくある問題

**❌ セキュリティ監査エラー**
```bash
# 解決方法: 脆弱な依存関係を更新
cargo update
make audit
```

**❌ Clippy警告**
```bash
# 解決方法: 警告を修正
make lint
# 自動修正可能な場合
cargo clippy --fix
```

**❌ フォーマットエラー**
```bash
# 解決方法: 自動フォーマット適用
make format
```

### ヘルプとサポート

- **Issues**: GitHub Issues で質問・バグ報告
- **Discussions**: GitHub Discussions で技術的な議論
- **Security**: セキュリティ問題は security@project.com に報告

---

## 🏆 コントリビューター行動規範

### 期待される行動

- 🤝 **協力的**: 建設的なフィードバック
- 🎯 **品質重視**: セキュリティ・品質基準の遵守
- 📖 **ドキュメント重視**: 変更には適切なドキュメント更新
- 🧪 **テスト重視**: 十分なテスト追加

### 禁止行為

- ❌ セキュリティチェックをスキップした PR
- ❌ テストのないコード変更
- ❌ フォーマット・lint違反のコード
- ❌ 不適切なライセンスの依存関係追加

---

**🙏 このプロジェクトへの貢献をありがとうございます！**

セキュリティと品質を最優先に、一緒に素晴らしいプロジェクトを作りましょう。