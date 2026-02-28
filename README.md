# minigrep

Rust製のシンプルなコマンドライン検索ツール。指定したファイルからキーワードを検索します。

## 使い方

```sh
cargo run -- <検索文字列> <ファイルパス>
```

### 例

```sh
cargo run -- hello poem.txt
```

大文字小文字を無視して検索する場合は環境変数 `IGNORE_CASE` を設定します。

```sh
IGNORE_CASE=1 cargo run -- hello poem.txt
```

## 機能

- コマンドライン引数から検索文字列とファイルパスを受け取る
- `Config::build`で設定を管理（エラーは`Result`で返す）
- 引数不足時のバリデーション
- 指定されたファイルからマッチした行を検索・表示
- `IGNORE_CASE`環境変数による大文字小文字を無視した検索
- `Box<dyn Error>`によるエラーハンドリング
- `lib.rs` / `main.rs` 分離構成
- エラーメッセージは`eprintln!`で標準エラー出力に出力
- ユニットテスト (`#[cfg(test)]`)
