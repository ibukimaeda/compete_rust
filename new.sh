#!/bin/bash

# 引数の数をチェック
if [ $# -ne 2 ]; then
  echo "Usage: $0 {c|vc} <project_name>"
  exit 1
fi

# 引数の処理
case $1 in
  c)
    cargo compete new $2
    cargo member include $2
    cd $2
    ;;
  vc)
    cd virtual_contest
    cargo compete new $2
    cd ..
    cargo member include virtual_contest/$2
    cd virtual_contest/$2
    ;;
  *)
    echo "Invalid option: $1"
    echo "Usage: $0 {c|vc} <project_name>"
    exit 1
    ;;
esac
