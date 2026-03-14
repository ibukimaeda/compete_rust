#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

type Coord = (usize, usize);

fn main() {
    input! {
        N: usize,
        A: [[i64; N]; N],
    }

    let mut best_path = Vec::new();
    let mut best_score = i64::MIN;

    for sym in 0..8 {
        update_best(
            transformed_row_snake(N, sym),
            &A,
            &mut best_score,
            &mut best_path,
        );
        update_best(
            transformed_diagonal_snake(N, sym),
            &A,
            &mut best_score,
            &mut best_path,
        );

        if N % 2 == 0 {
            update_best(
                transformed_block_snake(N, sym, &A),
                &A,
                &mut best_score,
                &mut best_path,
            );
        }
    }

    for (i, j) in best_path {
        println!("{i} {j}");
    }
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
        let base_day = (path.len()) as i64;
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
