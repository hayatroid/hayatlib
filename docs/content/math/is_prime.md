+++
title = "素数判定（試し割り）"
description = "非負整数 n を受け取り、n が素数かどうかを返します。"
weight = 1
+++

{{ badge(path="src/math/is_prime.rs") }}

## 概要
非負整数 $n$ を受け取り，$n$ が素数かどうかを返す．

## 制約
$n$ は非負整数

## 計算量
$O(\sqrt{n})$

## 実装
{{ copy_btn() }}
```rs
{{ cat(path="src/math/is_prime.rs") }}
```

## Verified with
- [Run #9796363 < hayatroid < Solutions | Aizu Online Judge](https://onlinejudge.u-aizu.ac.jp/solutions/problem/ALDS1_1_C/review/9796363/hayatroid/Rust)<br>{{ badge(path="examples/ALDS1_1_C.rs") }}
