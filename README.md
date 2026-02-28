# minigrep

Rust製のシンプルなコマンドライン検索ツール。指定したファイルからキーワードを検索します。

[The Rust Programming Language 第12章](https://doc.rust-jp.rs/book-ja/ch12-00-an-io-project.html)のチュートリアルプロジェクト。

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

## このプロジェクトで学んだこと

### 1. コマンドライン引数の受け取り

`std::env::args()` でコマンドライン引数をイテレータとして取得し、`collect()` でベクタに変換する。

```rust
let args: Vec<String> = env::args().collect();
```

### 2. 関心の分離 — `main.rs` と `lib.rs` の分割

`main.rs` にはエントリポイントのみを残し、ロジックは `lib.rs` に移す。こうすることで：

- ロジックをユニットテストできるようになる（`main` 関数はテストしづらい）
- コードの責務が明確になる

### 3. `Config` 構造体によるデータのまとめ

バラバラな変数をひとつの構造体にまとめることで、関数のシグネチャがシンプルになり、意図が伝わりやすくなる。

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
```

### 4. `panic!` から `Result` へのエラーハンドリングの改善

最初は引数不足時に `panic!` を使っていたが、`Result` を返す `Config::build` に変えることでエラーを呼び出し元でハンドリングできるようになった。

```rust
// Before: パニックする
panic!("not enough arguments");

// After: Result で返す
pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    // ...
}
```

### 5. `Box<dyn Error>` — 複数のエラー型をまとめて返す

`Box<dyn Error>` は「`Error` トレイトを実装した何らかの型」を動的ディスパッチで扱う。サイズが不定なためヒープ（`Box`）に置く。ファイルI/Oエラーなど複数の種類のエラーを同じ型で返せる。

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // ...
    Ok(())
}
```

### 6. `?` 演算子

`Result` や `Option` を返す関数内で、エラーなら即座に `Err` を返し、成功なら値を取り出す。`unwrap` と違いパニックしない。

```rust
let contents = fs::read_to_string(config.file_path)?;
```

### 7. `unwrap_or_else` でエラー時の処理を記述

`Result` がエラーのときにクロージャを実行する。`main` でのエラー処理に使う。

```rust
let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
});
```

### 8. ライフタイム注釈

`search` 関数の戻り値（`Vec<&str>`）は、引数 `contents` と同じライフタイムを持つことをコンパイラに伝える必要がある。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
```

`'a` は「`contents` が生きている間だけ、戻り値の参照も有効」という意味。

### 9. 環境変数の読み取り

`std::env::var` で環境変数を読む。変数が存在すれば `Ok`、なければ `Err` を返す。

```rust
let ignore_case = env::var("IGNORE_CASE").is_ok();
```

### 10. `eprintln!` — 標準エラー出力への書き出し

エラーメッセージは `println!`（標準出力）ではなく `eprintln!`（標準エラー出力）に出すことで、通常の出力とエラーを分離できる。

```sh
# 標準出力だけをファイルにリダイレクトし、エラーはターミナルに表示させる
cargo run -- query file.txt > output.txt
```

### 11. テスト駆動開発（TDD）

先にテストを書いてから実装する流れを体験した。`#[cfg(test)]` ブロック内にテストを書くことで、`cargo test` 時だけコンパイルされる。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        assert_eq!(vec!["safe, fast, productive."], search("duct", contents));
    }
}
```
