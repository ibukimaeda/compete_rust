#![allow(non_snake_case)]

//! AHC062 の解法本体。
//!
//! 入力として `N x N` の人口配列を受け取り、
//! すべてのマスをちょうど一度ずつ通る king-move 経路を出力する。
//! まず複数のベースライン経路を作り、その後で局所探索により
//! 「後ろの日に重いマスを置く」方向へ訪問順を改善する。

use itertools::Itertools;
use proconio::input;
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

type Coord = (usize, usize);

/// 解全体で使う制限時間。
/// 秒単位で指定し、ブロック段階とセル段階の探索をこの中で配分する。
const TOTAL_TIME_LIMIT_SEC: f64 = 2.8;
/// 全体時間のうち、ブロック順最適化へ優先的に割り当てる割合。
/// 大域的な順序改善を先に行い、その後のセル単位探索へ残り時間を渡す。
const BLOCK_STAGE_RATIO: f64 = 0.3;
/// 焼きなまし開始時の温度。
/// 悪化手でも受理しやすくして、探索の初期に広く状態を動かすために使う。
const START_TEMP: f64 = 2_000_000.0;
/// 焼きなまし終了時の温度。
/// 終盤では改善手を優先しやすくするため、小さめの値に落とす。
const END_TEMP: f64 = 1_000.0;
/// swap 近傍で選ぶ 2 点の最大 index 距離。
/// 経路全体ではなく近い位置だけを入れ替えて局所修正に寄せる。
const SWAP_WINDOW: usize = 64;
/// 区間最適化で扱う最短区間長。
/// これより短い区間は並べ替え候補として選ばない。
const SEGMENT_OPT_MIN_LEN: usize = 5;
/// 区間最適化で扱う最長区間長。
/// DP の計算量が増えすぎないように上限を置く。
const SEGMENT_OPT_MAX_LEN: usize = 8;
/// 1 手の試行で区間最適化近傍を使う確率。
/// 現在の主力近傍なので高めに設定している。
const SEGMENT_OPT_RATE: f64 = 0.8;
/// reverse 近傍で扱う最短区間長。
/// 短い区間だけを反転して小さい順序変更を試す。
const REVERSE_MIN_LEN: usize = 3;
/// reverse 近傍で扱う最長区間長。
/// 大きすぎる反転で隣接制約を壊しやすくなるのを防ぐ。
const REVERSE_MAX_LEN: usize = 8;
/// 区間最適化以外で reverse 近傍を使う確率。
/// 残りの確率では swap を試す。
const REVERSE_RATE: f64 = 0.15;
/// 何回試行したら一度時刻を確認するか。
/// 毎回時計を見るオーバーヘッドを避けるためにまとめて確認する。
const TIME_CHECK_INTERVAL: u64 = 256;

#[derive(Clone)]
/// 現在の巡回経路の状態。
///
/// `path` は「何日目にどのマスへ行くか」を表し、
/// `pos` は「そのマスが何日目に現れるか」を逆引きする。
/// `score` は現在の経路全体のスコアで、差分更新で保つ。
struct State {
    path: Vec<Coord>,
    pos: Vec<usize>,
    score: i64,
}

impl State {
    /// 完全な経路とそのスコアから `State` を構築する。
    ///
    /// 入力は `path`、その経路の総スコア `score`、盤面サイズ `N`。
    /// 出力は `path` と `pos` の両方が整合した探索用状態。
    fn new(path: Vec<Coord>, score: i64, N: usize) -> Self {
        let mut pos = vec![usize::MAX; path.len()];
        for (day, &cell) in path.iter().enumerate() {
            pos[cell_index(cell, N)] = day;
        }
        Self { path, pos, score }
    }

    /// 2 点 swap を状態へ反映する。
    ///
    /// 入力は交換する 2 位置 `i, j` と、その move によるスコア差分 `delta`。
    /// 出力はなく、`path`、`pos`、`score` を破壊的に更新する。
    fn apply_swap(&mut self, i: usize, j: usize, N: usize, delta: i64) {
        let left = self.path[i];
        let right = self.path[j];
        self.path.swap(i, j);
        self.pos[cell_index(left, N)] = j;
        self.pos[cell_index(right, N)] = i;
        self.score += delta;
    }

    /// 連続区間の反転を状態へ反映する。
    ///
    /// 入力は区間 `[l, r]` と差分 `delta`。
    /// 出力はなく、区間を反転した経路と対応する逆引き配列を更新する。
    fn apply_reverse(&mut self, l: usize, r: usize, N: usize, delta: i64) {
        self.path[l..=r].reverse();
        for idx in l..=r {
            self.pos[cell_index(self.path[idx], N)] = idx;
        }
        self.score += delta;
    }

    /// 連続区間を新しい順序で丸ごと置き換える。
    ///
    /// 入力は左端 `l`、置き換え後の区間順序 `order`、差分 `delta`。
    /// 出力はなく、区間最適化で得た新順序を現在状態へ適用する。
    fn apply_segment_order(&mut self, l: usize, order: &[Coord], N: usize, delta: i64) {
        for (offset, &cell) in order.iter().enumerate() {
            let idx = l + offset;
            self.path[idx] = cell;
            self.pos[cell_index(cell, N)] = idx;
        }
        self.score += delta;
    }
}

/// 1 テストケースを読み込み、最終経路を標準出力へ書き出す。
///
/// 入力は標準入力からの `N` と人口配列 `A`。
/// 出力は `N^2` 行の座標列で、各行が「その日に訪問するマス」を表す。
/// まずセル直列の初期解と 2x2 ブロック単位の大域的初期解を作り、
/// 後者をセル経路へ復元したものも候補に加えたうえで、
/// 最後にセル単位の局所探索で改善して最良経路を出力する。
fn main() {
    input! {
        N: usize,
        A: [[i64; N]; N],
    }

    let start = Instant::now();
    let weights = flatten_weights(&A);
    let mut best_initial = build_initial_state(N, &A);

    if N % 2 == 0 {
        let block_weight_matrix = build_block_weight_matrix(&A);
        let block_weights = flatten_weights(&block_weight_matrix);
        let block_size = N / 2;
        let block_initial = build_initial_state(block_size, &block_weight_matrix);
        let block_stage_limit = TOTAL_TIME_LIMIT_SEC * BLOCK_STAGE_RATIO;
        let block_stage_budget = (block_stage_limit - start.elapsed().as_secs_f64()).max(0.0);
        let block_path = if block_stage_budget > 0.0 {
            anneal(
                block_initial,
                block_size,
                &block_weights,
                block_stage_budget,
            )
        } else {
            block_initial.path
        };
        let expanded_path = expand_block_path(&block_path, &A);
        let expanded_score = calc_score(&expanded_path, &A);
        if expanded_score > best_initial.score {
            best_initial = State::new(expanded_path, expanded_score, N);
        }
    }

    let remaining = (TOTAL_TIME_LIMIT_SEC - start.elapsed().as_secs_f64()).max(0.0);
    let best_path = if remaining > 0.0 {
        anneal(best_initial, N, &weights, remaining)
    } else {
        best_initial.path
    };

    for (i, j) in best_path {
        println!("{i} {j}");
    }
}

/// 制限時間いっぱいまで局所探索を回し、最良経路を返す。
///
/// 入力は初期状態 `state`、盤面サイズ `N`、flatten 済み人口配列 `weights`、
/// そして秒単位の時間制限 `time_limit_sec`。
/// 出力は探索中に見つかったスコア最大の経路 `Vec<Coord>`。
fn anneal(mut state: State, N: usize, weights: &[i64], time_limit_sec: f64) -> Vec<Coord> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9e37_79b9_7f4a_7c15);
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut best_score = state.score;
    let mut best_path = state.path.clone();
    let total = state.path.len();
    let mut iterations = 0_u64;
    let start = Instant::now();
    let mut elapsed = 0.0;

    while elapsed < time_limit_sec {
        let progress = (elapsed / time_limit_sec).clamp(0.0, 1.0);
        let temp = START_TEMP.powf(1.0 - progress) * END_TEMP.powf(progress);
        let move_type = random_f64(&mut rng);

        if move_type < SEGMENT_OPT_RATE {
            if let Some((l, r)) = sample_segment_range(total, &mut rng) {
                try_optimize_segment(&mut state, l, r, N, weights);
            }
        } else if move_type < SEGMENT_OPT_RATE + REVERSE_RATE {
            if let Some((l, r)) = sample_reverse_range(total, &mut rng) {
                try_reverse(&mut state, l, r, N, weights, temp, &mut rng);
            }
        } else {
            let (i, j) = sample_swap_indices(total, &mut rng);
            try_swap(&mut state, i, j, N, weights, temp, &mut rng);
        }

        if state.score > best_score {
            best_score = state.score;
            best_path = state.path.clone();
        }

        iterations += 1;
        if iterations % TIME_CHECK_INTERVAL == 0 {
            elapsed = start.elapsed().as_secs_f64();
        }
    }

    best_path
}

/// 経路中の 2 日を入れ替える swap 近傍を 1 回試す。
///
/// 入力は現在状態、交換候補の 2 位置 `i, j`、人口配列、温度、乱数器。
/// 出力は `bool` で、合法かつ受理されて実際に状態を更新したとき `true` を返す。
fn try_swap(
    state: &mut State,
    i: usize,
    j: usize,
    N: usize,
    weights: &[i64],
    temp: f64,
    rng: &mut SmallRng,
) -> bool {
    if i == j || !is_swap_valid(&state.path, i, j, N) {
        return false;
    }

    let wi = cell_weight(state.path[i], N, weights);
    let wj = cell_weight(state.path[j], N, weights);
    let delta = (j as i64 - i as i64) * (wi - wj);

    if !should_accept(delta, temp, rng) {
        return false;
    }

    state.apply_swap(i, j, N, delta);
    true
}

/// 短い連続区間の反転近傍を 1 回試す。
///
/// 入力は現在状態、反転区間 `[l, r]`、人口配列、温度、乱数器。
/// 出力は `bool` で、反転が合法で受理されたときだけ `true` を返す。
fn try_reverse(
    state: &mut State,
    l: usize,
    r: usize,
    N: usize,
    weights: &[i64],
    temp: f64,
    rng: &mut SmallRng,
) -> bool {
    if l >= r || !is_reverse_valid(&state.path, l, r, N) {
        return false;
    }

    let delta = reverse_delta(&state.path, l, r, N, weights);
    if !should_accept(delta, temp, rng) {
        return false;
    }

    state.apply_reverse(l, r, N, delta);
    true
}

/// 短い連続区間を「両端と繋がる範囲で最も高得点な順序」に並べ替える。
///
/// 入力は現在状態、対象区間 `[l, r]`、盤面サイズ、人口配列。
/// 出力は `bool` で、より良い順序が見つかって実際に置き換えたとき `true` を返す。
/// この近傍は常に改善手のみ採用する。
fn try_optimize_segment(state: &mut State, l: usize, r: usize, N: usize, weights: &[i64]) -> bool {
    let current: Vec<_> = state.path[l..=r].to_vec();
    let prev = l.checked_sub(1).map(|idx| state.path[idx]);
    let next = state.path.get(r + 1).copied();

    let Some((best_order, best_score)) = best_segment_order(&current, prev, next, l, N, weights)
    else {
        return false;
    };

    let current_score = segment_score(&current, l, N, weights);
    let delta = best_score - current_score;
    if delta <= 0 || best_order == current {
        return false;
    }

    state.apply_segment_order(l, &best_order, N, delta);
    true
}

/// 悪化手を確率受理するかどうかを判定する。
///
/// 入力はスコア差分 `delta`、現在温度 `temp`、乱数器。
/// 出力は `bool` で、改善手なら常に `true`、悪化手なら温度に応じた確率で `true`。
fn should_accept(delta: i64, temp: f64, rng: &mut SmallRng) -> bool {
    if delta >= 0 {
        return true;
    }
    random_bool(rng, (delta as f64 / temp).exp().clamp(0.0, 1.0))
}

/// swap 近傍用に 2 つの経路 index を選ぶ。
///
/// 入力は経路長 `total` と乱数器。
/// 出力は `(i, j)` で、`i` の近傍 window 内から別の 1 点 `j` を選んだもの。
fn sample_swap_indices(total: usize, rng: &mut SmallRng) -> (usize, usize) {
    let i = random_range_usize(rng, 0, total);
    let lo = i.saturating_sub(SWAP_WINDOW);
    let hi = (i + SWAP_WINDOW).min(total - 1);
    let width = hi - lo;
    let offset = random_range_usize(rng, 0, width);
    let mut j = lo + offset;
    if j >= i {
        j += 1;
    }
    (i, j)
}

/// reverse 近傍用に短い連続区間を 1 つ選ぶ。
///
/// 入力は経路長 `total` と乱数器。
/// 出力は `Some((l, r))` で、長さ制約を満たす区間を返す。
/// 十分な長さがないときは `None`。
fn sample_reverse_range(total: usize, rng: &mut SmallRng) -> Option<(usize, usize)> {
    if total < REVERSE_MIN_LEN {
        return None;
    }

    let max_len = REVERSE_MAX_LEN.min(total);
    let len = random_range_inclusive_usize(rng, REVERSE_MIN_LEN, max_len);
    let l = random_range_inclusive_usize(rng, 0, total - len);
    Some((l, l + len - 1))
}

/// 区間最適化近傍用に短い連続区間を 1 つ選ぶ。
///
/// 入力は経路長 `total` と乱数器。
/// 出力は `Some((l, r))` で、DP で最適並べ替えを試す対象区間を返す。
fn sample_segment_range(total: usize, rng: &mut SmallRng) -> Option<(usize, usize)> {
    if total < SEGMENT_OPT_MIN_LEN {
        return None;
    }

    let max_len = SEGMENT_OPT_MAX_LEN.min(total);
    let len = random_range_inclusive_usize(rng, SEGMENT_OPT_MIN_LEN, max_len);
    let l = random_range_inclusive_usize(rng, 0, total - len);
    Some((l, l + len - 1))
}

/// 確率 `p` で真になるベルヌーイ乱数を生成する。
///
/// 入力は乱数器と確率 `p`。
/// 出力は `bool` で、受理判定や move 種別の分岐に使う。
fn random_bool(rng: &mut SmallRng, p: f64) -> bool {
    if p <= 0.0 {
        return false;
    }
    if p >= 1.0 {
        return true;
    }
    let threshold = (p * ((u64::MAX as f64) + 1.0)) as u64;
    rng.next_u64() < threshold
}

/// 区間 `[0, 1)` の一様乱数を生成する。
///
/// 入力は乱数器。
/// 出力は `f64` で、複数近傍の選択確率を決めるときに使う。
fn random_f64(rng: &mut SmallRng) -> f64 {
    const DENOM: f64 = (u64::MAX as f64) + 1.0;
    (rng.next_u64() as f64) / DENOM
}

/// 半開区間 `[lo, hi)` から一様に整数を 1 つ選ぶ。
///
/// 入力は下限 `lo`、上限 `hi`、乱数器。
/// 出力は `usize` で、各種近傍の index 選択に使う。
fn random_range_usize(rng: &mut SmallRng, lo: usize, hi: usize) -> usize {
    debug_assert!(lo < hi);
    lo + sample_below(rng, hi - lo)
}

/// 閉区間 `[lo, hi]` から一様に整数を 1 つ選ぶ。
///
/// 入力は下限 `lo`、上限 `hi`、乱数器。
/// 出力は `usize` で、区間長や左端を inclusive に選ぶために使う。
fn random_range_inclusive_usize(rng: &mut SmallRng, lo: usize, hi: usize) -> usize {
    debug_assert!(lo <= hi);
    lo + sample_below(rng, hi - lo + 1)
}

/// 区間 `[0, n)` から modulo bias なしで整数を 1 つ選ぶ。
///
/// 入力は上限 `n` と乱数器。
/// 出力は `usize` で、乱数生成の基礎部品として使う。
fn sample_below(rng: &mut SmallRng, n: usize) -> usize {
    debug_assert!(n > 0);
    let n = n as u64;
    let zone = u64::MAX - (u64::MAX % n);
    loop {
        let x = rng.next_u64();
        if x < zone {
            return (x % n) as usize;
        }
    }
}

/// swap 後も局所の隣接制約が保たれるかを判定する。
///
/// 入力は現在経路 `path`、交換したい位置 `i, j`、盤面サイズ `N`。
/// 出力は `bool` で、交換で影響する前後数辺だけを見て合法性を返す。
fn is_swap_valid(path: &[Coord], i: usize, j: usize, N: usize) -> bool {
    let mut edges = [usize::MAX; 4];
    let mut edge_count = 0;

    add_edge(&mut edges, &mut edge_count, path.len(), i);
    add_edge(&mut edges, &mut edge_count, path.len(), j);

    for &edge_start in &edges[..edge_count] {
        let a = cell_after_swap(path, i, j, edge_start);
        let b = cell_after_swap(path, i, j, edge_start + 1);
        if !is_adjacent(a, b, N) {
            return false;
        }
    }
    true
}

/// ある位置 `idx` の前後で影響を受ける辺候補を追加する。
///
/// 入力は辺開始 index の配列、現在数、経路長 `len`、注目位置 `idx`。
/// 出力はなく、重複を避けつつ `idx-1 -> idx` と `idx -> idx+1` を登録する。
fn add_edge(edges: &mut [usize; 4], edge_count: &mut usize, len: usize, idx: usize) {
    for candidate in [idx.checked_sub(1), (idx + 1 < len).then_some(idx)] {
        let Some(edge_start) = candidate else {
            continue;
        };
        if edges[..*edge_count].iter().all(|&x| x != edge_start) {
            edges[*edge_count] = edge_start;
            *edge_count += 1;
        }
    }
}

/// swap 後の仮想的な経路で、位置 `idx` に来るマスを返す。
///
/// 入力は元の経路 `path`、交換位置 `i, j`、問い合わせ位置 `idx`。
/// 出力は `Coord` で、実際に swap せずに局所辺を検証するために使う。
fn cell_after_swap(path: &[Coord], i: usize, j: usize, idx: usize) -> Coord {
    if idx == i {
        path[j]
    } else if idx == j {
        path[i]
    } else {
        path[idx]
    }
}

/// 区間反転後も両端の接続が保たれるかを判定する。
///
/// 入力は現在経路 `path`、反転区間 `[l, r]`、盤面サイズ `N`。
/// 出力は `bool` で、外側との接続 2 辺だけを見て合法性を返す。
fn is_reverse_valid(path: &[Coord], l: usize, r: usize, N: usize) -> bool {
    if l > 0 && !is_adjacent(path[l - 1], path[r], N) {
        return false;
    }
    if r + 1 < path.len() && !is_adjacent(path[l], path[r + 1], N) {
        return false;
    }
    true
}

/// 区間 `[l, r]` を反転したときのスコア差分を求める。
///
/// 入力は現在経路 `path`、対象区間、盤面サイズ `N`、人口配列 `weights`。
/// 出力は `i64` の差分で、正なら反転でスコアが上がる。
fn reverse_delta(path: &[Coord], l: usize, r: usize, N: usize, weights: &[i64]) -> i64 {
    let mut delta = 0_i64;
    for offset in 0..=(r - l) {
        let day = (l + offset) as i64;
        let old = cell_weight(path[l + offset], N, weights);
        let new = cell_weight(path[r - offset], N, weights);
        delta += day * (new - old);
    }
    delta
}

/// 固定された入口・出口条件の下で、短区間内の最良順序を求める。
///
/// 入力は対象区間 `segment`、区間の直前 `prev`、直後 `next`、
/// 区間が始まる日 `base_day`、盤面サイズ `N`、人口配列 `weights`。
/// 出力は `Some((best_order, best_score))` で、
/// 区間内だけを並べ替えたときの最良順序とその区間寄与スコアを返す。
/// 入口と出口に繋がる Hamiltonian path が存在しないときは `None`。
fn best_segment_order(
    segment: &[Coord],
    prev: Option<Coord>,
    next: Option<Coord>,
    base_day: usize,
    N: usize,
    weights: &[i64],
) -> Option<(Vec<Coord>, i64)> {
    let m = segment.len();
    if m == 0 {
        return None;
    }

    let states = 1_usize << m;
    let neg_inf = i64::MIN / 4;
    let mut can_follow = vec![false; m * m];
    let mut start_ok = vec![false; m];
    let mut end_ok = vec![false; m];
    let mut cell_weights = vec![0_i64; m];

    for i in 0..m {
        let cell = segment[i];
        cell_weights[i] = cell_weight(cell, N, weights);
        start_ok[i] = prev.map_or(true, |p| is_adjacent(p, cell, N));
        end_ok[i] = next.map_or(true, |p| is_adjacent(cell, p, N));
        for j in 0..m {
            can_follow[i * m + j] = i != j && is_adjacent(cell, segment[j], N);
        }
    }

    let mut dp = vec![neg_inf; states * m];
    let mut parent = vec![usize::MAX; states * m];

    for i in 0..m {
        if start_ok[i] {
            dp[(1 << i) * m + i] = base_day as i64 * cell_weights[i];
        }
    }

    for mask in 0..states {
        let used = mask.count_ones() as usize;
        if used == 0 || used == m {
            continue;
        }
        let day = (base_day + used) as i64;
        let remaining = (states - 1) ^ mask;
        for last in 0..m {
            let cur = dp[mask * m + last];
            if cur == neg_inf {
                continue;
            }

            let mut bits = remaining;
            while bits != 0 {
                let lb = bits & bits.wrapping_neg();
                let next_idx = lb.trailing_zeros() as usize;
                if can_follow[last * m + next_idx] {
                    let next_mask = mask | (1 << next_idx);
                    let next_pos = next_mask * m + next_idx;
                    let cand = cur + day * cell_weights[next_idx];
                    if cand > dp[next_pos] {
                        dp[next_pos] = cand;
                        parent[next_pos] = last;
                    }
                }
                bits ^= lb;
            }
        }
    }

    let full_mask = states - 1;
    let mut best_score = neg_inf;
    let mut best_last = usize::MAX;
    for last in 0..m {
        if !end_ok[last] {
            continue;
        }
        let score = dp[full_mask * m + last];
        if score > best_score {
            best_score = score;
            best_last = last;
        }
    }

    if best_last == usize::MAX {
        return None;
    }

    let mut order = vec![(0, 0); m];
    let mut mask = full_mask;
    let mut last = best_last;
    for pos in (0..m).rev() {
        order[pos] = segment[last];
        let parent_idx = parent[mask * m + last];
        mask ^= 1 << last;
        if mask == 0 {
            break;
        }
        last = parent_idx;
    }

    Some((order, best_score))
}

/// 区間がある日付から置かれたときの区間寄与スコアを計算する。
///
/// 入力は区間 `segment`、その左端が対応する日 `base_day`、
/// 盤面サイズ `N`、人口配列 `weights`。
/// 出力は `i64` で、その区間だけのスコア合計。
fn segment_score(segment: &[Coord], base_day: usize, N: usize, weights: &[i64]) -> i64 {
    segment
        .iter()
        .enumerate()
        .map(|(offset, &cell)| (base_day + offset) as i64 * cell_weight(cell, N, weights))
        .sum()
}

/// 2 次元座標を 1 次元 index に変換する。
///
/// 入力はマス座標 `(i, j)` と盤面サイズ `N`。
/// 出力は `i * N + j` で、`weights` や `pos` の添字として使う。
fn cell_index((i, j): Coord, N: usize) -> usize {
    i * N + j
}

/// flatten 済み人口配列から、そのマスの人口値を返す。
///
/// 入力はマス座標 `cell`、盤面サイズ `N`、1 次元配列 `weights`。
/// 出力は `i64` の人口値で、スコア差分計算に使う。
fn cell_weight(cell: Coord, N: usize, weights: &[i64]) -> i64 {
    weights[cell_index(cell, N)]
}

/// 2 次元人口配列を行優先の 1 次元配列に変換する。
///
/// 入力は `A[i][j]` 形式の人口配列。
/// 出力は `weights[i * N + j]` で参照できる 1 次元ベクタ。
fn flatten_weights(A: &[Vec<i64>]) -> Vec<i64> {
    A.iter().flat_map(|row| row.iter().copied()).collect()
}

/// 元のセル人口配列から、2x2 ブロックごとの人口和を作る。
///
/// 入力はセル単位の人口配列 `A`。
/// 出力は `N/2 x N/2` の配列で、各要素が対応する 2x2 ブロックの人口和になる。
/// ブロック順の大域探索ではこの和を重みとして用いる。
fn build_block_weight_matrix(A: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let N = A.len();
    let B = N / 2;
    let mut block_weights = vec![vec![0_i64; B]; B];
    for bi in 0..B {
        for bj in 0..B {
            let cells = block_cells((bi, bj));
            block_weights[bi][bj] = cells.iter().map(|&(i, j)| A[i][j]).sum();
        }
    }
    block_weights
}

/// ブロック順を受け取り、それに一致するセル順の経路へ厳密に展開する。
///
/// 入力は 2x2 ブロック単位の訪問順 `block_path` とセル人口配列 `A`。
/// 出力は `Vec<Coord>` で、各ブロックを 4 連続日で訪問する具体的なセル経路。
/// 各ブロック内の開始セル・終了セルは、前後ブロックとの接続を守る範囲で
/// 動的計画法により最良の組合せを選ぶ。
fn expand_block_path(block_path: &[Coord], A: &[Vec<i64>]) -> Vec<Coord> {
    let block_count = block_path.len();
    let neg_inf = i64::MIN / 4;
    let block_cells_list: Vec<_> = block_path.iter().copied().map(block_cells).collect();
    let mut dp = vec![neg_inf; block_count * 4];
    let mut parent_exit = vec![usize::MAX; block_count * 4];
    let mut chosen_start = vec![usize::MAX; block_count * 4];

    for end_idx in 0..4 {
        for start_idx in 0..4 {
            if start_idx == end_idx {
                continue;
            }
            let score = block_order_score(&block_cells_list[0], start_idx, end_idx, 0, A);
            let pos = end_idx;
            if score > dp[pos] {
                dp[pos] = score;
                chosen_start[pos] = start_idx;
            }
        }
    }

    for block_idx in 1..block_count {
        let mut next_dp = vec![neg_inf; 4];
        let current_cells = &block_cells_list[block_idx];
        let prev_cells = &block_cells_list[block_idx - 1];
        let base_day = (4 * block_idx) as i64;

        for prev_end_idx in 0..4 {
            let prev_score = dp[(block_idx - 1) * 4 + prev_end_idx];
            if prev_score == neg_inf {
                continue;
            }
            let prev_exit_cell = prev_cells[prev_end_idx];
            for end_idx in 0..4 {
                for start_idx in 0..4 {
                    if start_idx == end_idx {
                        continue;
                    }
                    let start_cell = current_cells[start_idx];
                    if !is_adjacent(prev_exit_cell, start_cell, A.len()) {
                        continue;
                    }
                    let cand = prev_score
                        + block_order_score(current_cells, start_idx, end_idx, base_day, A);
                    if cand > next_dp[end_idx] {
                        next_dp[end_idx] = cand;
                        let pos = block_idx * 4 + end_idx;
                        parent_exit[pos] = prev_end_idx;
                        chosen_start[pos] = start_idx;
                    }
                }
            }
        }

        for end_idx in 0..4 {
            dp[block_idx * 4 + end_idx] = next_dp[end_idx];
        }
    }

    let mut best_end_idx = 0;
    let mut best_score = neg_inf;
    for end_idx in 0..4 {
        let score = dp[(block_count - 1) * 4 + end_idx];
        if score > best_score {
            best_score = score;
            best_end_idx = end_idx;
        }
    }

    let mut start_indices = vec![0_usize; block_count];
    let mut end_indices = vec![0_usize; block_count];
    let mut current_end = best_end_idx;
    for block_idx in (0..block_count).rev() {
        let pos = block_idx * 4 + current_end;
        start_indices[block_idx] = chosen_start[pos];
        end_indices[block_idx] = current_end;
        if block_idx > 0 {
            current_end = parent_exit[pos];
        }
    }

    let mut path = Vec::with_capacity(block_count * 4);
    for block_idx in 0..block_count {
        let order = block_order_from_endpoints(
            &block_cells_list[block_idx],
            start_indices[block_idx],
            end_indices[block_idx],
            A,
        );
        path.extend(order);
    }
    path
}

/// 1 つの 2x2 ブロックについて、開始セルと終了セルを固定した最良順序を返す。
///
/// 入力はブロック内 4 マス `cells`、開始セル index `start_idx`、終了セル index `end_idx`、
/// そして人口配列 `A`。
/// 出力は長さ 4 の配列で、最初が開始セル、最後が終了セルとなる最良順序。
/// 中間 2 マスは「軽い方を先、重い方を後」に置く。
fn block_order_from_endpoints(
    cells: &[Coord; 4],
    start_idx: usize,
    end_idx: usize,
    A: &[Vec<i64>],
) -> [Coord; 4] {
    let mut middle = [usize::MAX; 2];
    let mut count = 0;
    for idx in 0..4 {
        if idx != start_idx && idx != end_idx {
            middle[count] = idx;
            count += 1;
        }
    }
    if A[cells[middle[0]].0][cells[middle[0]].1] > A[cells[middle[1]].0][cells[middle[1]].1] {
        middle.swap(0, 1);
    }
    [
        cells[start_idx],
        cells[middle[0]],
        cells[middle[1]],
        cells[end_idx],
    ]
}

/// 開始セル・終了セルを固定した 1 ブロック分の寄与スコアを計算する。
///
/// 入力はブロック内 4 マス `cells`、開始セル index `start_idx`、終了セル index `end_idx`、
/// ブロック左端に対応する日 `base_day`、人口配列 `A`。
/// 出力は `i64` で、その 4 日分のスコア寄与を返す。
fn block_order_score(
    cells: &[Coord; 4],
    start_idx: usize,
    end_idx: usize,
    base_day: i64,
    A: &[Vec<i64>],
) -> i64 {
    let order = block_order_from_endpoints(cells, start_idx, end_idx, A);
    order
        .iter()
        .enumerate()
        .map(|(offset, &(i, j))| (base_day + offset as i64) * A[i][j])
        .sum()
}

/// 2 つのマスが king move で隣接しているかを判定する。
///
/// 入力は 2 マス `a, b` と盤面サイズ `N`。
/// 出力は `bool` で、縦横斜めのいずれか 1 手なら `true`。
fn is_adjacent(a: Coord, b: Coord, _N: usize) -> bool {
    let di = a.0.abs_diff(b.0);
    let dj = a.1.abs_diff(b.1);
    (di != 0 || dj != 0) && di <= 1 && dj <= 1
}

/// 手作りのベースライン候補群から初期解を組み立てる。
///
/// 入力は盤面サイズ `N` と人口配列 `A`。
/// 出力は `State` で、row snake / diagonal snake / block snake と
/// それらの反転を比較したうえで最良のものを保持する。
fn build_initial_state(N: usize, A: &[Vec<i64>]) -> State {
    let mut best_path = Vec::new();
    let mut best_score = i64::MIN;

    for sym in 0..8 {
        update_best(
            transformed_row_snake(N, sym),
            A,
            &mut best_score,
            &mut best_path,
        );
        update_best(
            transformed_diagonal_snake(N, sym),
            A,
            &mut best_score,
            &mut best_path,
        );

        if N % 2 == 0 {
            update_best(
                transformed_block_snake(N, sym, A),
                A,
                &mut best_score,
                &mut best_path,
            );
        }
    }

    State::new(best_path, best_score, N)
}

/// 候補経路を評価し、必要なら現在の最良解を更新する。
///
/// 入力は候補経路 `path`、人口配列 `A`、現在の最良スコアと最良経路への参照。
/// 出力はなく、順方向と逆方向の両方を試して、より良ければ最良解を上書きする。
fn update_best(path: Vec<Coord>, A: &[Vec<i64>], best_score: &mut i64, best_path: &mut Vec<Coord>) {
    let score = calc_score(&path, A);
    if score > *best_score {
        *best_score = score;
        *best_path = path.clone();
    }

    let rev_score = calc_score_reversed(&path, A);
    if rev_score > *best_score {
        *best_score = rev_score;
        *best_path = path.into_iter().rev().collect();
    }
}

/// 完全な経路 1 本の総スコアをそのまま計算する。
///
/// 入力は訪問順 `path` と人口配列 `A`。
/// 出力は `sum(day * A[i][j])` の値で、候補比較や検証に使う。
fn calc_score(path: &[Coord], A: &[Vec<i64>]) -> i64 {
    path.iter()
        .enumerate()
        .map(|(day, &(i, j))| day as i64 * A[i][j])
        .sum()
}

/// 同じ経路を逆順にたどった場合の総スコアを計算する。
///
/// 入力は訪問順 `path` と人口配列 `A`。
/// 出力は逆順訪問時のスコアで、初期解候補の向き選択に使う。
fn calc_score_reversed(path: &[Coord], A: &[Vec<i64>]) -> i64 {
    let total_days = path.len() as i64 - 1;
    path.iter()
        .enumerate()
        .map(|(day, &(i, j))| (total_days - day as i64) * A[i][j])
        .sum()
}

/// 行方向の snake 経路を作り、指定した対称変換をかけて返す。
///
/// 入力は盤面サイズ `N` と対称変換番号 `sym`。
/// 出力は `Vec<Coord>` で、全マスを一度ずつ通る row snake 経路。
fn transformed_row_snake(N: usize, sym: usize) -> Vec<Coord> {
    let mut path = Vec::with_capacity(N * N);
    for i in 0..N {
        if i % 2 == 0 {
            for j in 0..N {
                path.push(apply_symmetry(N, sym, (i, j)));
            }
        } else {
            for j in (0..N).rev() {
                path.push(apply_symmetry(N, sym, (i, j)));
            }
        }
    }
    path
}

/// 対角線ごとの snake 経路を作り、指定した対称変換をかけて返す。
///
/// 入力は盤面サイズ `N` と対称変換番号 `sym`。
/// 出力は `Vec<Coord>` で、対角線単位に往復するベースライン経路。
fn transformed_diagonal_snake(N: usize, sym: usize) -> Vec<Coord> {
    let mut path = Vec::with_capacity(N * N);
    for s in 0..=2 * (N - 1) {
        let lo = s.saturating_sub(N - 1);
        let hi = s.min(N - 1);
        if s % 2 == 0 {
            for i in lo..=hi {
                let j = s - i;
                path.push(apply_symmetry(N, sym, (i, j)));
            }
        } else {
            for i in (lo..=hi).rev() {
                let j = s - i;
                path.push(apply_symmetry(N, sym, (i, j)));
            }
        }
    }
    path
}

/// 2x2 ブロック単位で蛇行しつつ、各ブロック内部の順序も最適化して返す。
///
/// 入力は盤面サイズ `N`、対称変換番号 `sym`、人口配列 `A`。
/// 出力は `Vec<Coord>` で、ブロック間の接続を保った block snake 経路。
fn transformed_block_snake(N: usize, sym: usize, A: &[Vec<i64>]) -> Vec<Coord> {
    let B = N / 2;
    let block_order = row_snake_blocks(B);
    let mut path = Vec::with_capacity(N * N);

    for block_index in 0..block_order.len() {
        let block = block_order[block_index];
        let prev = block_index.checked_sub(1).map(|idx| block_order[idx]);
        let next = block_order.get(block_index + 1).copied();

        let entry_mask = prev
            .map(|prev_block| entry_mask(direction(prev_block, block)))
            .unwrap_or(0b1111);
        let exit_mask = next
            .map(|next_block| exit_mask(direction(block, next_block)))
            .unwrap_or(0b1111);

        let cells = block_cells(block);
        let base_day = path.len() as i64;
        let mut best_perm = None;
        let mut best_value = i64::MIN;

        for perm in (0..4).permutations(4) {
            if (entry_mask & (1 << perm[0])) == 0 {
                continue;
            }
            if (exit_mask & (1 << perm[3])) == 0 {
                continue;
            }

            let mut value = 0_i64;
            for (offset, &idx) in perm.iter().enumerate() {
                let cell = apply_symmetry(N, sym, cells[idx]);
                value += (base_day + offset as i64) * A[cell.0][cell.1];
            }

            if value > best_value {
                best_value = value;
                best_perm = Some(perm);
            }
        }

        let perm = best_perm.expect("block permutation must exist");
        for idx in perm {
            path.push(apply_symmetry(N, sym, cells[idx]));
        }
    }

    path
}

/// ブロック格子上での row snake 順序を返す。
///
/// 入力はブロック格子の一辺 `B`。
/// 出力は `Vec<Coord>` で、各 2x2 ブロックをどの順番で訪れるかを表す。
fn row_snake_blocks(B: usize) -> Vec<Coord> {
    let mut order = Vec::with_capacity(B * B);
    for i in 0..B {
        if i % 2 == 0 {
            for j in 0..B {
                order.push((i, j));
            }
        } else {
            for j in (0..B).rev() {
                order.push((i, j));
            }
        }
    }
    order
}

/// ある 2x2 ブロックに含まれる 4 マスを返す。
///
/// 入力はブロック座標 `(bi, bj)`。
/// 出力は左上、右上、左下、右下の順の 4 マス配列。
fn block_cells((bi, bj): Coord) -> [Coord; 4] {
    let r = 2 * bi;
    let c = 2 * bj;
    [(r, c), (r, c + 1), (r + 1, c), (r + 1, c + 1)]
}

/// ブロック格子上で、あるブロックから次ブロックへの方向を返す。
///
/// 入力は始点ブロック `from` と終点ブロック `to`。
/// 出力は `(di, dj)` で、上下左右のどちらへ進むかを表す。
fn direction(from: Coord, to: Coord) -> (isize, isize) {
    (
        to.0 as isize - from.0 as isize,
        to.1 as isize - from.1 as isize,
    )
}

/// 前のブロックから入ってくる向きに応じて、入れるマス集合を返す。
///
/// 入力はブロック間方向 `(di, dj)`。
/// 出力は 4bit のマスクで、各 bit が 2x2 ブロック内の 1 マスに対応する。
fn entry_mask((di, dj): (isize, isize)) -> u8 {
    match (di, dj) {
        (0, 1) => 0b0101,
        (0, -1) => 0b1010,
        (1, 0) => 0b0011,
        (-1, 0) => 0b1100,
        _ => unreachable!(),
    }
}

/// 次のブロックへ抜ける向きに応じて、出られるマス集合を返す。
///
/// 入力はブロック間方向 `(di, dj)`。
/// 出力は 4bit のマスクで、block snake の接続条件判定に使う。
fn exit_mask((di, dj): (isize, isize)) -> u8 {
    match (di, dj) {
        (0, 1) => 0b1010,
        (0, -1) => 0b0101,
        (1, 0) => 0b1100,
        (-1, 0) => 0b0011,
        _ => unreachable!(),
    }
}

/// 盤面の 8 通りの対称変換のいずれかを 1 マスへ適用する。
///
/// 入力は盤面サイズ `N`、対称変換番号 `sym`、元の座標 `(i, j)`。
/// 出力は変換後の座標で、ベースライン候補を 8 方向へ展開するために使う。
fn apply_symmetry(N: usize, sym: usize, (i, j): Coord) -> Coord {
    match sym {
        0 => (i, j),
        1 => (i, N - 1 - j),
        2 => (N - 1 - i, j),
        3 => (N - 1 - i, N - 1 - j),
        4 => (j, i),
        5 => (j, N - 1 - i),
        6 => (N - 1 - j, i),
        7 => (N - 1 - j, N - 1 - i),
        _ => unreachable!(),
    }
}
