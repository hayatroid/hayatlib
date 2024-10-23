+++
title = "2 つの数の和"
weight = 1
+++

## 概要
非負整数 $a, b$ を受け取り，$a + b$ を返す．

## 計算量
$O(1)$

## 制約
- $0 \leq a < 2^{64}$
- $0 \leq b < 2^{64}$
- $0 \leq a + b < 2^{64}$

## 実装
{{ copy_btn() }}
```rs
{{ cat(path="src_bundled/sample/add.rs") }}
```
