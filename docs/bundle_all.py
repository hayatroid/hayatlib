# hayatlib 専用バンドラ
# rustfmt が必要です！


import os
import subprocess
from pathlib import Path


# bundle するファイルの探索先ディレクトリと，bundle したファイルの保存先ディレクトリ
def bundle(root_from: Path, root_to: Path):
    # bundle するファイルを探索する
    for (dir, _, files) in os.walk(root_from):
        for file in files:
            # bundle するファイルのパスと，bundle したファイルの保存先パス
            path_from = Path(dir) / file
            path_to = root_to / path_from.relative_to(root_from)

            # path_from が lib.rs の場合や，sample ディレクトリが存在するときの sample.rs の場合などは除外する
            if path_from.name == "lib.rs" or (path_from.parent / path_from.name.removesuffix(".rs")).is_dir():
                continue

            # path_to が存在しない場合は新たに作成する
            path_to.parent.mkdir(parents=True, exist_ok=True)
            path_to.touch(exist_ok=True)

            # python3 bundle.py path_from の実行結果を path_to に保存する
            program = subprocess.run(
                ["python3", "bundle.py", path_from], stdout=subprocess.PIPE).stdout
            with open(path_to, "wb") as f:
                f.write(program)


bundle(Path("../src").absolute(), Path("./src").absolute())
bundle(Path("../examples").absolute(), Path("./examples").absolute())
