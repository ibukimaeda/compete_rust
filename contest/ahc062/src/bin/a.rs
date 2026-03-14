#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

type Coord = (usize, usize);

const TIME_LIMIT_SEC: f64 = 2.8;
const START_TEMP: f64 = 2_000_000.0;
const END_TEMP: f64 = 1_000.0;
const SWAP_WINDOW: usize = 64;
const REVERSE_MIN_LEN: usize = 3;
const REVERSE_MAX_LEN: usize = 8;
const REVERSE_RATE: f64 = 0.2;
const TIME_CHECK_INTERVAL: u64 = 256;

#[derive(Clone)]
struct State {
    path: Vec<Coord>,
    pos: Vec<usize>,
    score: i64,
}

impl State {
    fn new(path: Vec<Coord>, score: i64, N: usize) -> Self {
        let mut pos = vec![usize::MAX; path.len()];
        for (day, &cell) in path.iter().enumerate() {
            pos[cell_index(cell, N)] = day;
        }
        Self { path, pos, score }
    }

    fn apply_swap(&mut self, i: usize, j: usize, N: usize, delta: i64) {
        let left = self.path[i];
        let right = self.path[j];
        self.path.swap(i, j);
        self.pos[cell_index(left, N)] = j;
        self.pos[cell_index(right, N)] = i;
        self.score += delta;
    }

    fn apply_reverse(&mut self, l: usize, r: usize, N: usize, delta: i64) {
        self.path[l..=r].reverse();
        for idx in l..=r {
            self.pos[cell_index(self.path[idx], N)] = idx;
        }
        self.score += delta;
    }
}

fn main() {
    input! {
        N: usize,
        A: [[i64; N]; N],
    }

    let weights = flatten_weights(&A);
    let initial = build_initial_state(N, &A);
    let best_path = anneal(initial, N, &weights, TIME_LIMIT_SEC);

    for (i, j) in best_path {
        println!("{i} {j}");
    }
}

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

        if random_bool(&mut rng, REVERSE_RATE) {
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

fn should_accept(delta: i64, temp: f64, rng: &mut SmallRng) -> bool {
    if delta >= 0 {
        return true;
    }
    random_bool(rng, (delta as f64 / temp).exp().clamp(0.0, 1.0))
}

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

fn sample_reverse_range(total: usize, rng: &mut SmallRng) -> Option<(usize, usize)> {
    if total < REVERSE_MIN_LEN {
        return None;
    }

    let max_len = REVERSE_MAX_LEN.min(total);
    let len = random_range_inclusive_usize(rng, REVERSE_MIN_LEN, max_len);
    let l = random_range_inclusive_usize(rng, 0, total - len);
    Some((l, l + len - 1))
}

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

fn random_range_usize(rng: &mut SmallRng, lo: usize, hi: usize) -> usize {
    debug_assert!(lo < hi);
    lo + sample_below(rng, hi - lo)
}

fn random_range_inclusive_usize(rng: &mut SmallRng, lo: usize, hi: usize) -> usize {
    debug_assert!(lo <= hi);
    lo + sample_below(rng, hi - lo + 1)
}

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

fn cell_after_swap(path: &[Coord], i: usize, j: usize, idx: usize) -> Coord {
    if idx == i {
        path[j]
    } else if idx == j {
        path[i]
    } else {
        path[idx]
    }
}

fn is_reverse_valid(path: &[Coord], l: usize, r: usize, N: usize) -> bool {
    if l > 0 && !is_adjacent(path[l - 1], path[r], N) {
        return false;
    }
    if r + 1 < path.len() && !is_adjacent(path[l], path[r + 1], N) {
        return false;
    }
    true
}

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

fn cell_index((i, j): Coord, N: usize) -> usize {
    i * N + j
}

fn cell_weight(cell: Coord, N: usize, weights: &[i64]) -> i64 {
    weights[cell_index(cell, N)]
}

fn flatten_weights(A: &[Vec<i64>]) -> Vec<i64> {
    A.iter().flat_map(|row| row.iter().copied()).collect()
}

fn is_adjacent(a: Coord, b: Coord, _N: usize) -> bool {
    let di = a.0.abs_diff(b.0);
    let dj = a.1.abs_diff(b.1);
    (di != 0 || dj != 0) && di <= 1 && dj <= 1
}

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

fn calc_score(path: &[Coord], A: &[Vec<i64>]) -> i64 {
    path.iter()
        .enumerate()
        .map(|(day, &(i, j))| day as i64 * A[i][j])
        .sum()
}

fn calc_score_reversed(path: &[Coord], A: &[Vec<i64>]) -> i64 {
    let total_days = path.len() as i64 - 1;
    path.iter()
        .enumerate()
        .map(|(day, &(i, j))| (total_days - day as i64) * A[i][j])
        .sum()
}

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

fn block_cells((bi, bj): Coord) -> [Coord; 4] {
    let r = 2 * bi;
    let c = 2 * bj;
    [(r, c), (r, c + 1), (r + 1, c), (r + 1, c + 1)]
}

fn direction(from: Coord, to: Coord) -> (isize, isize) {
    (
        to.0 as isize - from.0 as isize,
        to.1 as isize - from.1 as isize,
    )
}

fn entry_mask((di, dj): (isize, isize)) -> u8 {
    match (di, dj) {
        (0, 1) => 0b0101,
        (0, -1) => 0b1010,
        (1, 0) => 0b0011,
        (-1, 0) => 0b1100,
        _ => unreachable!(),
    }
}

fn exit_mask((di, dj): (isize, isize)) -> u8 {
    match (di, dj) {
        (0, 1) => 0b1010,
        (0, -1) => 0b0101,
        (1, 0) => 0b1100,
        (-1, 0) => 0b0011,
        _ => unreachable!(),
    }
}

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
