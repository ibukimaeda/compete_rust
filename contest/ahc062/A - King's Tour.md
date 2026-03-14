### ストーリー

AtCoder 王国は $N \times N$ のグリッド状の区画に区切られており、各区画にはそれぞれ異なる人数の国民が住んでいる。 国王の高橋君は、国民からの好感度を上げるため、縦・横・斜めに隣接する区画を辿りながら、すべての区画を一度ずつ訪問するツアーを計画している。 ツアーは日を追うごとに盛り上がっていくので、後の日に訪れた区画ほど、住人の高橋君への好感度が高くなる。 高橋君が得る好感度の総和がなるべく大きくなるようなツアーを計画してほしい。

### 問題文

$N \times N$ の区画に区切られた王国がある。左上の区画を $(0,0)$ とし、下方向に $i$ 区画、右方向に $j$ 区画進んだ区画を $(i,j)$ とする。 区画 $(i,j)$ には $A_{i,j}$ 人の国民が住んでいる。

$0$ 日目から $N^2-1$ 日目までの $N^2$ 日間で、すべての区画をちょうど一度ずつ訪問するツアーを考える。 $k$ 日目に訪問する区画を $(i_k, j_k)$ とする。

このとき、連続する 2 日に訪問する区画は、 **縦・横・斜め** に隣接していなければならない。 より厳密には、各 $k=0,1,\dots,N^2-2$ について

$\max\bigl(|i_k-i_{k+1}|,\ |j_k-j_{k+1}|\bigr)=1$

が成り立つ必要がある。 ツアーの開始区画と終了区画は任意に選んでよい。

ツアーの $k$ 日目に訪問した区画では、住民 1 人あたり $k$ の好感度が得られる。 このとき、高橋君が得る好感度の総和は

$\sum_{k=0}^{N^2-1} k\cdot A_{i_k, j_k}$

である。

高橋君が得る好感度の総和がなるべく大きくなるような訪問順を求めよ。

### 得点

高橋君が得る好感度の総和を $V$ としたとき、

$\mathrm{round}\left(\frac{V}{N^2}\right)$

をそのテストケースにおける得点とする。

合計で 100 個のテストケースがあり、各テストケースの得点の合計が提出の得点となる。 一つ以上のテストケースで不正な出力や制限時間超過をした場合、提出全体の判定がWAやTLEとなる。 コンテスト時間中に得た最高得点で最終順位が決定され、コンテスト終了後のシステムテストは行われない。 同じ得点を複数の参加者が得た場合、提出時刻に関わらず同じ順位となる。

___

### 入力

入力は以下の形式で標準入力から与えられる。

```
<var><span><span><span id="305e9791-4e5d-45aa-b5cd-e61f5d03938d"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><mi>N</mi></mrow><annotation encoding="application/x-tex">N</annotation></semantics></math></span></span></span></var>
<var><span><span><span id="08f880a0-688f-4e7c-8fe3-b26f098afb6d"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msub><mi>A</mi><mrow><mn>0</mn><mo separator="true">,</mo><mn>0</mn></mrow></msub></mrow><annotation encoding="application/x-tex">A_{0,0}</annotation></semantics></math></span></span></span></var> <var><span><span><span id="e8430a87-d42a-4adf-b055-7d80740741f8"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><mo>⋯</mo></mrow><annotation encoding="application/x-tex">\cdots</annotation></semantics></math></span></span></span></var> <var><span><span><span id="22619e5d-4671-42c5-8875-4531a747752f"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msub><mi>A</mi><mrow><mn>0</mn><mo separator="true">,</mo><mi>N</mi><mo>−</mo><mn>1</mn></mrow></msub></mrow><annotation encoding="application/x-tex">A_{0,N-1}</annotation></semantics></math></span></span></span></var>
<var><span><span><span id="ea85e73c-c86b-4dd6-b1d7-79230340bf32"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><mi><mi mathvariant="normal">⋮</mi><mpadded height="+0em" voffset="0em"><mspace mathbackground="black" width="0em" height="1.5em"></mspace></mpadded></mi></mrow><annotation encoding="application/x-tex">\vdots</annotation></semantics></math></span></span></span></var>
<var><span><span><span id="a5814ab3-07d5-4afa-90e6-0b80fb77045b"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msub><mi>A</mi><mrow><mi>N</mi><mo>−</mo><mn>1</mn><mo separator="true">,</mo><mn>0</mn></mrow></msub></mrow><annotation encoding="application/x-tex">A_{N-1,0}</annotation></semantics></math></span></span></span></var> <var><span><span><span id="7285bc2b-84b1-421b-8a51-430e22a1f3e0"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><mo>⋯</mo></mrow><annotation encoding="application/x-tex">\cdots</annotation></semantics></math></span></span></span></var> <var><span><span><span id="5691ffc0-e8ca-49de-b1c8-77d750857a77"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msub><mi>A</mi><mrow><mi>N</mi><mo>−</mo><mn>1</mn><mo separator="true">,</mo><mi>N</mi><mo>−</mo><mn>1</mn></mrow></msub></mrow><annotation encoding="application/x-tex">A_{N-1,N-1}</annotation></semantics></math></span></span></span></var>
```

-   すべてのテストケースにおいて、$N=200$ で固定である。
-   区画 $(i,j)$ の住民数 $A_{i,j}$ は、$1\leq A_{i,j}\leq N^2$ を満たす整数である。

### 出力

ツアーの $k$ 日目に訪問する区画を $(i_k, j_k)$ とするとき、以下の形式で標準出力に出力せよ。

```
<var><span><span><span id="9c9c33e8-8300-47c1-9400-937550675d3e"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msub><mi>i</mi><mn>0</mn></msub></mrow><annotation encoding="application/x-tex">i_0</annotation></semantics></math></span></span></span></var> <var><span><span><span id="143cfdf2-e209-44ac-8d75-00ec23935f89"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msub><mi>j</mi><mn>0</mn></msub></mrow><annotation encoding="application/x-tex">j_0</annotation></semantics></math></span></span></span></var>
<var><span><span><span id="a4adecc7-9050-4554-b94e-50d2dbe90d50"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><mi><mi mathvariant="normal">⋮</mi><mpadded height="+0em" voffset="0em"><mspace mathbackground="black" width="0em" height="1.5em"></mspace></mpadded></mi></mrow><annotation encoding="application/x-tex">\vdots</annotation></semantics></math></span></span></span></var>
<var><span><span><span id="41ea8c2f-be59-4fc6-94e4-2d7313e84bc0"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msub><mi>i</mi><mrow><msup><mi>N</mi><mn>2</mn></msup><mo>−</mo><mn>1</mn></mrow></msub></mrow><annotation encoding="application/x-tex">i_{N^2-1}</annotation></semantics></math></span></span></span></var> <var><span><span><span id="f5ff6bdc-a957-406e-b945-f9764bf8f3b4"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msub><mi>j</mi><mrow><msup><mi>N</mi><mn>2</mn></msup><mo>−</mo><mn>1</mn></mrow></msub></mrow><annotation encoding="application/x-tex">j_{N^2-1}</annotation></semantics></math></span></span></span></var>
```

ただし、出力は以下の条件をすべて満たさなければならない。

-   $k=0,1,\cdots,N^2-1$ について、$i_k$ および $j_k$ は $0$ 以上 $N-1$ 以下の整数である。
-   $(i_k, j_k)$ $(k=0,1,\cdots,N^2-1)$ は互いに異なる。
-   $k=0,1,\cdots,N^2-2$ について、$\max(|i_k-i_{k+1}|,\ |j_k-j_{k+1}|)=1$ である。

[例を見る](https://img.atcoder.jp/ahc062/u5OpcTjC.html?lang=ja&seed=0&output=sample)

### 入力生成方法

-   すべてのテストケースにおいて $N=200$ である。
-   $1$ 以上 $N^2$ 以下の整数を一様ランダムに並び替え、それらを順に $A_{0,0}, A_{0,1}, \cdots, A_{0,N-1}, A_{1,0}, \cdots, A_{N-1,N-1}$ とする。

### ツール(入力ジェネレータ・ビジュアライザ)

-   [Web版](https://img.atcoder.jp/ahc062/u5OpcTjC.html?lang=ja): ローカル版より高性能でアニメーション表示が可能です。
-   [ローカル版](https://img.atcoder.jp/ahc062/u5OpcTjC.zip): 使用するには[Rust言語](https://www.rust-lang.org/ja)のコンパイル環境をご用意下さい。
    -   [Windows用のコンパイル済みバイナリ](https://img.atcoder.jp/ahc062/u5OpcTjC_windows.zip): Rust言語の環境構築が面倒な方は代わりにこちらをご利用下さい。

コンテスト期間中に、ビジュアライズ結果の共有や、解法・考察に関する言及は禁止されています。ご注意下さい。