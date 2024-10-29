+++
title = "素数判定（Miller-Rabin 法）"
description = "非負整数 n を受け取り、n が素数かどうかを返します。"
weight = 2
+++

{{ badge(path="src/math/is_prime_fast.rs") }}

## 概要
非負整数 $n$ を受け取り，$n$ が素数かどうかを返す．

## 制約
- $0 \leq n < 2^{64}$
    - [SPRP bases として $\\{ 2, 325, 9375, 28178, 450775, 9780504, 1795265022 \\}$ を用いると少なくとも $2^{64}$ までは決定的である](https://miller-rabin.appspot.com/)，ということによる制約．

## 計算量
- `modpow` の計算量と同じ

## 実装
{{ copy_btn() }}
```rs
{{ cat(path="src/math/is_prime_fast.rs") }}
```

## Verified with
- [Submission #245537](https://judge.yosupo.jp/submission/245537)<br>{{ badge(path="examples/primality_test.rs") }}
