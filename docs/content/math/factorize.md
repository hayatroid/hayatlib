+++
title = "素因数分解（試し割り法）"
description = "正の整数 n を受け取り、n の素因数を昇順に列挙します。"
weight = 3
+++

{{ badge(path="docs/content/math/factorize.md") }}

## 概要
正の整数 $n$ を受け取り，$n$ の素因数を昇順に列挙する．

## 制約
- $n$ は正の整数
    - $n = 0$ のとき panic する．

## 計算量
- $O(\sqrt{n})$

## 実装
{{ copy_btn() }}
```rs
{{ cat(path="src/math/factorize.rs") }}
```

## Verified with
- [提出 #59341432 - アルゴリズムと数学　演習問題集](https://atcoder.jp/contests/math-and-algorithm/submissions/59341432)
