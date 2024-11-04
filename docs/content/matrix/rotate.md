+++
title = "行列の 90 度回転"
description = "行列 $A$ を受け取り、$A$ を 90 度回転したものを返します。"
weight = 2
+++

{{ badge(path="docs/content/matrix/rotate.md") }}

## 概要
行列 $A$ を受け取り，$A$ を 90 度回転したものを返す．\
`rotate_ccw` は反時計回りに，`rotate_cw` は時計回りに 90 度回転する．

## 制約
- $A$ の各行の長さが一致していること．
    - [行列の転置](../transpose) の制約に合わせた．

## 実装
{{ program(path="src/matrix/rotate.md") }}

## Verified with
- [提出 #59445479 - AtCoder Beginner Contest 036](https://atcoder.jp/contests/abc036/submissions/59445479)
