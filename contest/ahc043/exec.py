import subprocess
import argparse

parser = argparse.ArgumentParser()

parser.add_argument("--release", action="store_true")
parser.add_argument("--seed", type=int, default=0)
parser.add_argument("--test", action="store_true")

args = parser.parse_args()

is_release = args.release
seed = args.seed

# コマンドと引数
command = ["cargo", "run", "--bin", "ahc043-a", "--color", "never"]

if is_release:
    command.append("--release")


if not args.test:
    # 入力ファイルと出力ファイルの指定
    with open(f"tools/in/{seed:04d}.txt", "r") as input_file, open("out.txt", "w") as output_file:
        subprocess.run(command, stdin=input_file, stdout=output_file)
else :
    failed = []
    for i in range(100) :
        with open(f"tools/in/{i:04d}.txt", "r") as input_file, open("out.txt", "w") as output_file:
            # 成功したかどうかの判定
            result = subprocess.run(command, stdin=input_file, stdout=output_file)
            if result.returncode != 0:
                failed.append((i, result.returncode))

    if failed:
        print(failed)
        exit(1)
    else:
        print("All tests passed")
        exit(0)

