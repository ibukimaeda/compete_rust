import subprocess
import argparse

parser = argparse.ArgumentParser()

parser.add_argument("--release", action="store_true")
parser.add_argument("--seed", type=int, default=0)

args = parser.parse_args()

is_release = args.release
seed = args.seed

# コマンドと引数
command = ["cargo", "run", "--bin", "ahc041-a"]

if is_release:
    command.append("--release")

# 入力ファイルと出力ファイルの指定
with open(f"tools/in/{seed:04d}.txt", "r") as input_file, open("out.txt", "w") as output_file:
    subprocess.run(command, stdin=input_file, stdout=output_file)
