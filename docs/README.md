# hayatlib/docs
`hayatlib/src` と `hayatlib/examples` をいい感じに bundle して，それを埋め込んだドキュメントを [Zola](https://www.getzola.org/) で生成しています．

## ドキュメント生成の流れ
GitHub Actions で以下を行っています．

1. `bundle.py` はライブラリをいい感じに bundle してくれるやつです（雑に書いており，完全に自分用となってしまっています）．
```
❯ cat ../src/sample/add_twice.rs
use super::add::add;

pub fn add_twice(a: u64, b: u64, c: u64) -> u64 {
    add(add(a, b), c)
}

❯ python3 bundle.py ../src/sample/add_twice.rs
fn add_twice(a: u64, b: u64, c: u64) -> u64 {
    add(add(a, b), c)
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}
```

2. `python3 bundle_all.py` で `hayatlib/src` 以下と `hayatlib/examples` 以下のファイルに対して `bundle.py` を呼び出し，出力結果を `hayatlib/docs` 以下（e.g. `hayatlib/docs/src/sample/add_twice.rs`）に保存しています．
3. Zola 側で `{{ load_data(path="src/sample/add_twice.rs") }}` などとするとファイルの埋め込みができます．
4. 最後に `zola build` をしています．
