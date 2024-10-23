+++
title = "3 つの数の和"
weight = 2
+++

## 概要
非負整数 $a, b, c$ を受け取り，$a + b + c$ を返す．

## 計算量
$O(1)$

## 制約
- $0 \leq a < 2^{64}$
- $0 \leq b < 2^{64}$
- $0 \leq c < 2^{64}$
- $0 \leq a + b + c < 2^{64}$

## 実装
```rs
{{ cat(path="src_bundled/sample/add_twice.rs") }}
```
