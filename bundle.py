# hayatlib 専用バンドラ
# rustfmt が必要です！


import subprocess
import sys
from pathlib import Path


# "use crate::a::b::c;" を Path("./src/a/b.rs") にして返す
def crate_to_path(line: bytes) -> Path:
    # "use hayatlib" または "use crate" で始まるかチェック
    assert line.startswith(b"use hayatlib") or line.startswith(b"use crate")

    # "use crate::a::b::c;" を Path("./src/a/b.rs") に
    filepath = Path("./src", *map(lambda x: x.decode(),
                    line.split(b"::")[1: -1])).with_suffix(".rs").absolute()

    # ファイルが存在するかチェック
    if not filepath.is_file():
        print(f"No such file: '{filepath}'")
        sys.exit(1)

    return filepath


# ("use super::a::b::c;", Path("./src/d/e.rs")) を Path("./src/d/a/b.rs") にして返す
def super_to_path(line: bytes, path: Path) -> Path:
    # "use super" で始まるかチェック
    assert line.startswith(b"use super")

    # ("use super::a::b::c;", Path("./src/d/e.rs")) を Path("./src/d/a/b.rs") に
    filepath = path.parent / \
        Path(*map(lambda x: x.decode(), line.split(b"::")
             [1: -1])).with_suffix(".rs")

    # ファイルが存在するかチェック
    if not filepath.is_file():
        print(f"No such file: '{filepath}'")
        sys.exit(1)

    return filepath


# program が依存するすべてのファイル（use crate と use super が参照しているファイル）のパスを集めたものと，
# そのすべてのファイルから use の行（use crate の行と use super の行は除く）を抜き出して集めたものを返す
def collect_dependencies(program: list[bytes], path: Path) -> tuple[set[Path], set[bytes]]:
    # program が依存するファイルのパスと use の行を集める
    crate_paths = set()
    others = set()
    for line in program:
        if line.startswith(b"use hayatlib") or line.startswith(b"use crate") or line.startswith(b"use super"):
            if line.startswith(b"use hayatlib") or line.startswith(b"use crate"):
                crate_path = crate_to_path(line)
            else:
                crate_path = super_to_path(line, path)
            crate_paths.add(crate_path)
            # 依存するファイルからも再帰的に集めてきて，和集合をとる
            with open(crate_path, "rb") as f:
                c, o = collect_dependencies(f.readlines(), crate_path)
                crate_paths |= c
                others |= o
        elif line.startswith(b"use"):
            others.add(line)

    return crate_paths, others


# program から use の行を取り除いたものを返す
# ついでにいらないもの（コメント等）も消しておく
def program_without_use(program: list[bytes]) -> list[bytes]:
    # use の行を取り除く
    program = [line for line in program if not line.startswith(b"use")]

    # ドキュメンテーションコメントの行を取り除く
    program = [line for line in program if b"//!" not in line]
    program = [line for line in program if b"///" not in line]

    # pub を消す
    program = [line.replace(b"pub ", b"") for line in program]

    return program


def main():
    # 引数の長さが正しいかチェック
    if len(sys.argv) == 1:
        print(f"Usage: python3 {sys.argv[0]} <filepath>")
        sys.exit(1)

    filepath = Path(sys.argv[1]).absolute()

    # ファイルが存在するかチェック
    if not filepath.is_file():
        print(f"No such file: '{filepath}'")
        sys.exit(1)

    # ファイルを読み込む
    with open(filepath, "rb") as f:
        program = f.read().splitlines(keepends=True)

    # use の行を解析するため，imports_granularity=Item で整形
    p = subprocess.run(["rustfmt", "--config", "imports_granularity=Item"],
                       input=b"".join(program), stdout=subprocess.PIPE)
    program = p.stdout.splitlines(keepends=True)

    # program が依存するファイルのパスと use の行を集めてくる
    crate_paths, others = collect_dependencies(program, filepath)

    # いい感じに合体
    result = list(others)
    result += program_without_use(program)
    for crate_path in sorted(crate_paths):
        with open(crate_path, "rb") as f:
            result += program_without_use(f.readlines())

    # imports_granularity=Crate,group_imports=StdExternalCrate で整形
    p = subprocess.run(["rustfmt", "--config", "imports_granularity=Crate,group_imports=StdExternalCrate"],
                       input=b"".join(result), stdout=subprocess.PIPE)
    result = p.stdout.splitlines(keepends=True)

    # 結果を出力
    sys.stdout.buffer.write(b"".join(result))


if __name__ == "__main__":
    main()
