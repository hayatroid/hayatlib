+++
title = "3 つの数の和"
weight = 2
+++

## 概要
非負整数 $a, b, c$ を受け取り，$a + b + c$ を返す．

## 制約
- $0 \leq a, b, c < 2^{64}$
- $a + b + c < 2^{64}$

## 計算量
$O(1)$

## 実装
{{ copy_btn() }}
```rs
{{ cat(path="src/sample/add_twice.rs") }}
```
