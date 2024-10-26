+++
title = "クエリを受け取る列挙型"
description = "クエリを一括で受け取れるような列挙型を実装します。"
weight = 1
+++

{{ badge(path="src/template/query.rs") }}

## 概要
クエリは，その種類によって型が異なったり長さが異なったりして，入力を受け取るのが面倒です．それらを一括で受け取れるような列挙型 `Query` を実装します．

```rs
input! {
    q: usize,
    queries: [Query; q],
}
```

## 実装
下記は [鉄則本 A51 - Stack](https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ay) に対して `Query` を実装した例です．クエリは `1 x`，`2`，`3` のいずれかで与えられます（`x` は `String`）．

他の問題を解く際には，下記のコメント部分（2 箇所）を書き換えてください．

{{ copy_btn() }}
```rs
{{ cat(path="src/template/query.rs") }}
```

## Verified with
- [提出 #59144917 - 競技プログラミングの鉄則　演習問題集](https://atcoder.jp/contests/tessoku-book/submissions/59144917) {{ badge(path="examples/template/query.rs") }}
