+++
title = "素数判定（Miller-Rabin 法）"
description = "非負整数 n を受け取り、n が素数かどうかを返します。"
weight = 2
+++

{{ badge(path="docs/content/math/is_prime_fast.md") }}

## 概要
非負整数 $n$ を受け取り，$n$ が素数かどうかを返す．

## 制約
- $0 \leq n < 2^{64}$
    - [SPRP bases として $\\{ 2, 325, 9375, 28178, 450775, 9780504, 1795265022 \\}$ を用いると少なくとも $2^{64}$ までは決定的である](https://miller-rabin.appspot.com/)，という事実に基づく制約．

## 計算量
- $O(\log n)$
    - $ab \bmod n$ が $O(1)$ で求まるとみなした場合の計算量．実際には定数倍重め．

## 実装
{{ copy_btn() }}
```rs
{{ cat(path="src/math/is_prime_fast.rs") }}
```

## Verified with
- [Submission #245735](https://judge.yosupo.jp/submission/245735)

## 参考
- [ミラー–ラビン素数判定法 - Wikipedia](https://ja.wikipedia.org/wiki/%E3%83%9F%E3%83%A9%E3%83%BC%E2%80%93%E3%83%A9%E3%83%93%E3%83%B3%E7%B4%A0%E6%95%B0%E5%88%A4%E5%AE%9A%E6%B3%95)
    - アルゴリズム部分の参考にした．
- [Deterministic variants of the Miller-Rabin primality test. Miller-Rabin SPRP bases records](https://miller-rabin.appspot.com/)
    - 基底部分の参考にした．
    - Remarks に注意．「単に $a < n$ なる $a$ でテストするだけでなく，$a \geq n$ なる $a$ に対しても $a \bmod n$ でテストする必要がある」（意訳）．
