use cargo_snippet::snippet;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

const INF: i64 = 1_010_000_000_000_000_017;

#[allow(dead_code)]
#[snippet(":dijkstra")]
fn dijkstra(graph: &Vec<Vec<(usize, i64)>>, start: usize) -> Vec<i64> {
    let mut dist = vec![INF; graph.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push((Reverse(0), start));

    while let Some((Reverse(cost), position)) = heap.pop() {
        // すでに見つけている経路が現経路より短い場合はスキップ
        if cost > dist[position] {
            continue;
        }

        for &(to, to_cost) in &graph[position] {
            let next_cost = cost + to_cost;

            // より短ければ更新
            if next_cost < dist[to] {
                heap.push((Reverse(next_cost), to));
                dist[to] = next_cost;
            }
        }
    }

    return dist;
}

#[test]
fn test_dijkstra_typical90_m() {
    // https://atcoder.jp/contests/typical90/tasks/typical90_m

    fn solve(N: usize, M: usize, ABC: Vec<(usize, usize, i64)>, ans: Vec<i64>) {
        let mut graph = vec![vec![]; N];

        for &(a, b, c) in &ABC {
            graph[a - 1].push((b - 1, c));
            graph[b - 1].push((a - 1, c));
        }

        let dist0 = dijkstra(&graph, 0);
        let distN = dijkstra(&graph, N - 1);

        for k in 0..N {
            assert_eq!(ans[k], dist0[k] + distN[k]);
        }
    }

    // sample 1
    let (N, M) = (7, 9);
    let ABC = vec![
        (1, 2, 2),
        (1, 3, 3),
        (2, 5, 2),
        (3, 4, 1),
        (3, 5, 4),
        (4, 7, 5),
        (5, 6, 1),
        (5, 7, 6),
        (6, 7, 3),
    ];
    let ans = vec![8, 8, 9, 9, 8, 8, 8];
    solve(N, M, ABC, ans);

    // sample 2
    let (N, M) = (4, 3);
    let ABC = vec![(1, 2, 1), (2, 3, 10), (3, 4, 100)];
    let ans = vec![111, 111, 111, 111];
    solve(N, M, ABC, ans);

    // sample 3
    let (N, M) = (4, 3);
    let ABC = vec![(1, 2, 314), (1, 3, 159), (1, 4, 265)];
    let ans = vec![265, 893, 583, 265];
    solve(N, M, ABC, ans);
}
