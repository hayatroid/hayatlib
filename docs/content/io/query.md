+++
title = "クエリの受け取り"
description = "クエリを一括で受け取れるような列挙型を実装します。"
weight = 1
+++

{{ badge() }}

## 概要
たとえばクエリ 1 は `1 x y`，クエリ 2 は `2 x` というように，クエリ毎に長さが異なるものを一括で受け取るのは難しい．
しかし `proconio::source::Readable` を実装した列挙型 `Query` を用いると，以下のようにクエリを一括で受け取れる．
```rs
input! {
    q: usize,
    queries: [Query; q],
}
```

## 実装
以下は [鉄則本 A51 - Stack](https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ay) に対する `Query` の実装である．
問題文に合わせて適宜コメント部分を書き換えること．

{{ program() }}

## Verified with
- [提出 #59199965 - 競技プログラミングの鉄則　演習問題集](https://atcoder.jp/contests/tessoku-book/submissions/59199965)
