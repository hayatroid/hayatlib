+++
title = "クエリを受け取る列挙型"
description = "クエリを一括で受け取れるような列挙型を実装します。"
weight = 1
+++

{{ badge(path="src/template/query.rs") }}

## 概要
```rs
input! {
    q: usize,
    queries: [Query; q],
}
```
のような書き方を可能とする．

## 実装
下記は [鉄則本 A51 - Stack](https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ay) に対する `Query` の実装である．
問題文に合わせて適宜コメント部分を書き換えること．

{{ copy_btn() }}
```rs
{{ cat(path="src/template/query.rs") }}
```

## Verified with
- [提出 #59199965 - 競技プログラミングの鉄則　演習問題集](https://atcoder.jp/contests/tessoku-book/submissions/59199965)<br>{{ badge(path="examples/tessoku_book_ay.rs") }}
