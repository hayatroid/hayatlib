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
- [Submission #245535](https://judge.yosupo.jp/submission/245535)<br>{{ badge(path="examples/primality_test.rs") }}

## 補足
`is_witness` の内部は下記のようにワンライナーでも書けて，こちらの方がわかりやすいと思う．
しかし都度 `modpow` を呼び出すため 2 倍程度遅くなってしまった（[Submission #245537](https://judge.yosupo.jp/submission/245537)）．
```rs
a != 0
&& modpow(a, d, n) != 1
&& (0..s).all(|i| modpow(a, d << i, n) != n - 1)
```
