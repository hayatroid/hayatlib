+++
title = "素因数分解（Pollard の rho 法）"
description = "正の整数 n を受け取り、n の素因数を昇順に列挙します。"
weight = 4
+++

{{ badge(path="docs/content/math/factorize_fast.md") }}

## 概要
正の整数 $n$ を受け取り，$n$ の素因数を昇順に列挙する．

## 制約
- $1 \leq n < 2^{64}$
    - $n = 0$ のとき panic する．
    - 上限は [Miller-Rabin 法](../is-prime-fast) の制約に合わせた．

## 計算量
- 厳密な解析は未解決問題である．

## 実装
{{ program(path="src/math/factorize_fast.md") }}

## Verified with
- [Submission #246461](https://judge.yosupo.jp/submission/246461)

## 参考
- [ポラード・ロー素因数分解法 - Wikipedia](https://ja.wikipedia.org/wiki/%E3%83%9D%E3%83%A9%E3%83%BC%E3%83%89%E3%83%BB%E3%83%AD%E3%83%BC%E7%B4%A0%E5%9B%A0%E6%95%B0%E5%88%86%E8%A7%A3%E6%B3%95)
    - アルゴリズム部分の参考にした．
- [間違ったポラードのロー法の使い方 - よーる](https://lpha-z.hatenablog.com/entry/2023/01/15/231500)
    - 乱数生成部分の参考にした．