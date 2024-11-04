+++
title = "2 次元配列の 90 度回転"
description = "2 次元配列 $A$ を 90 度回転します。"
weight = 2
+++

{{ badge(path="docs/content/util/rotate.md") }}

## 概要
2 次元配列 $A$ を 90 度回転する．\
`rotate_ccw` は反時計回りに，`rotate_cw` は時計回りに 90 度回転する．

## 制約
- $A$ の各行の長さが一致していること．
    - [2 次元配列の転置](../transpose) の制約に合わせた．

## 実装
{{ program(path="src/util/rotate.md") }}

## Verified with
- [提出 #59445479 - AtCoder Beginner Contest 036](https://atcoder.jp/contests/abc036/submissions/59445479)
