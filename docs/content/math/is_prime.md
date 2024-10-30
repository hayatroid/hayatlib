+++
title = "素数判定（試し割り法）"
description = "非負整数 n を受け取り、n が素数かどうかを返します。"
weight = 1
+++

{{ badge(path="docs/content/math/is_prime.md") }}

## 概要
非負整数 $n$ を受け取り，$n$ が素数かどうかを返す．

## 制約
- $n$ は非負整数

## 計算量
- $O(\sqrt{n})$

## 実装
{{ copy_btn() }}
```rs
{{ cat(path="src/math/is_prime.rs") }}
```

## Verified with
- [提出 #59275022 - アルゴリズムと数学　演習問題集](https://atcoder.jp/contests/math-and-algorithm/submissions/59275022)
