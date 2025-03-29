#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::{div_ceil, div_floor, gcd, lcm};
use proconio::{
    fastout, input, input_interactive,
    marker::{Chars, Isize1, Usize1},
};
use rand::{thread_rng, Rng};
use rustc_hash::{FxHashMap, FxHashSet};

use std::cmp;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::default;
use std::fmt;
use std::mem;
use std::ops;
use std::time::Instant;
use std::vec;

#[allow(dead_code)]
// const MOD: i64 = 1_000_000_007;
// const MOD : i64 = 1_000_000_009;
const MOD: i64 = 998_244_353;

#[allow(dead_code)]
const INF: i64 = 1_010_000_000_000_000_017;

#[allow(dead_code)]
const DX: [i64; 4] = [0, 0, 1, -1];
#[allow(dead_code)]
const DY: [i64; 4] = [1, -1, 0, 0];

#[allow(non_snake_case)]
fn main() {
    input_interactive!(N:usize, UV:[(Usize1,Usize1);N-1]);

    let mut graph = vec![FxHashSet::default(); N];
    for (u, v) in UV {
        graph[u].insert(v);
        graph[v].insert(u);
    }

    // ２部グラフを作成
    // 頂点 0 をルートとして，0 からの距離が偶数の頂点を left に，奇数の頂点を right に分ける
    let mut left = FxHashSet::default();
    let mut right = FxHashSet::default();

    // 幅優先探索
    let mut que = VecDeque::new();
    que.push_back(0);
    let mut dist = vec![INF; N];
    dist[0] = 0;
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if dist[v] == INF {
                dist[v] = dist[u] + 1;
                que.push_back(v);
            }
        }
    }

    for i in 0..N {
        if dist[i] % 2 == 0 {
            left.insert(i);
        } else {
            right.insert(i);
        }
    }

    // 完全２部グラフになるまでに必要な辺の数
    let mut num_edges_for_complete_bipartite_graph = 0;
    for &l in &left {
        num_edges_for_complete_bipartite_graph += right.difference(&graph[l]).count();
    }

    let mut can_fill_edges = FxHashMap::default();
    for &l in &left {
        let edges_to_add = right.difference(&graph[l]).collect::<FxHashSet<_>>();
        can_fill_edges.insert(l, edges_to_add);
    }

    if num_edges_for_complete_bipartite_graph % 2 == 0 {
        // 偶数の場合は，後手が勝つ
        say("Second");
        input_interactive!(u:i64, v:i64);

        if u == -1 && v == -1 {
            return;
        } else {
            let mut u = (u - 1) as usize;
            let mut v = (v - 1) as usize;

            if right.contains(&u) {
                (u, v) = (v, u);
            }

            can_fill_edges.get_mut(&u).unwrap().remove(&v);

            if can_fill_edges[&u].is_empty() {
                can_fill_edges.remove(&u);
            }
        }
    } else {
        // 奇数の場合は，先手が勝つ
        say("First");
    }

    loop {
        let (&l, rs) = can_fill_edges.iter().next().unwrap();
        let r = *rs.iter().next().unwrap();

        can_fill_edges.get_mut(&l).unwrap().remove(r);
        if can_fill_edges[&l].is_empty() {
            can_fill_edges.remove(&l);
        }

        if l < *r {
            println!("{} {}", l + 1, *r + 1);
        } else {
            println!("{} {}", *r + 1, l + 1);
        }

        input_interactive!(u:i64, v:i64);

        if u == -1 && v == -1 {
            return;
        } else {
            let mut u = (u - 1) as usize;
            let mut v = (v - 1) as usize;

            if right.contains(&u) {
                (u, v) = (v, u);
            }

            can_fill_edges.get_mut(&u).unwrap().remove(&v);

            if can_fill_edges[&u].is_empty() {
                can_fill_edges.remove(&u);
            }
        }
    }
}

#[allow(dead_code)]
fn yes() {
    println!("Yes");
}

#[allow(dead_code)]
fn no() {
    println!("No");
}

#[allow(dead_code)]
fn say<T: std::fmt::Display>(v: T) {
    println!("{}", v);
}

#[allow(dead_code)]
fn say_vec<T: std::fmt::Display>(v: Vec<T>) {
    println!("{}", v.iter().join(" "));
}

#[macro_export]
macro_rules! min {
    // 1 == min!(3, 2, 1)

    // 引数が 1個なら，そのまま返す
    ($a:expr $(,)*) => {{
        $a
    }};

    // 引数が 2個なら std::cmp::min を使用する
    ($a:expr, &b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};

    // 引数が 3個以上なら，再帰的に min! マクロを呼び出す
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

#[macro_export]
macro_rules! max {
    // 3 == max!(3, 2, 1)

    // 引数が 1個なら，そのまま返す
    ($a:expr $(,)*) => {{
        $a
    }};

    // 引数が 2個なら std::cmp::max を使用する
    ($a:expr, &b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};

    // 引数が 3個以上なら，再帰的に max! マクロを呼び出す
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

#[macro_export]
macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        // 第2引数以降の部分に関して、min! を使用して最小値を求める
        let cmp_min = min!($($cmps),+);

        // それが第1引数より小さかったら、更新して true を返す
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            // 更新が不要なので、false を返す
            false
        }
    }};
}

#[macro_export]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        // 第2引数以降の部分に関して、max! を使用して最大値を求める
        let cmp_max = max!($($cmps),+);

        // それが第1引数より大きかったら、更新して true を返す
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            // 更新が不要なので、false を返す
            false
        }
    }};
}

#[macro_export]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[macro_export]
macro_rules! debug_with_message {
    ($msg:expr $(, $a:expr)* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!("| ", $msg, " |", $(" ", stringify!($a), "={:?} |"),*), $(&$a),*);
    };
}

#[macro_export]
macro_rules! debug_vec {
    ($vec:expr) => {
        #[cfg(debug_assertions)]
        {
            use std::fmt::Write;
            let mut output = String::new();
            write!(output, "[").unwrap();
            for (i, val) in $vec.iter().enumerate() {
                if i > 0 {
                    write!(output, ", ").unwrap();
                }
                write!(output, "{:?}", val).unwrap();
            }
            write!(output, "]").unwrap();
            eprintln!("{}={}", stringify!($vec), output);
        }
    };
}

#[macro_export]
macro_rules! debug_vec2 {
    ($vec2:expr) => {
        #[cfg(debug_assertions)]
        {
            use std::fmt::Write;
            let mut output = String::new();
            write!(output, "[\n").unwrap();
            for vec in $vec2.iter() {
                write!(output, "   [").unwrap();
                for (j, val) in vec.iter().enumerate() {
                    if j > 0 {
                        write!(output, ", ").unwrap();
                    }
                    write!(output, "{:?}", val).unwrap();
                }
                write!(output, "]\n").unwrap();
            }
            write!(output, "]").unwrap();
            eprintln!("{}={}", stringify!($vec2), output);
        }
    };
}

#[macro_export]
macro_rules! nested_vec {
    ($e:expr; $n:expr) => {
        vec![$e; $n]
    };
    ($e:expr; $n:expr $(; $m:expr)+) => {
        vec![nested_vec!($e $(; $m)+); $n]
    };
}

// https://zenn.dev/qnighy/articles/a62e5c2a6ba8ef#swap%E3%81%AB%E9%96%A2%E3%81%97%E3%81%A6%E8%A9%B3%E3%81%97%E3%81%8F
#[macro_export]
macro_rules! swap {
    ($x: expr, $y: expr) => {
        $crate::rotate!($x, $y)
    };
    ($x: expr, $y: expr,) => {
        $crate::rotate!($x, $y)
    };
}

#[macro_export]
macro_rules! rotate {
    ($x: expr, $($y: expr),*) => {
        {
            let value = $crate::take!($x);
            $(
                let value = $crate::Replace::replace($y, value);
            )*
            let _ = $crate::Replace::replace($x, value);
        }
    };
    ($x: expr) => {
        $crate::rotate!($x,)
    };
    ($x: expr, $($y: expr),*,) => {
        $crate::rotate!($x, $($y),*)
    };
}

use core::cell::{Cell, RefCell};

pub trait Replace<T> {
    fn replace(self, value: T) -> T;
}

impl<'a, T> Replace<T> for &'a mut T {
    fn replace(self, value: T) -> T {
        mem::replace(self, value)
    }
}

impl<'a, T> Replace<T> for &'a Cell<T> {
    fn replace(self, value: T) -> T {
        self.replace(value)
    }
}

impl<'a, T> Replace<T> for &'a RefCell<T> {
    fn replace(self, value: T) -> T {
        let mut r = self.borrow_mut();
        mem::replace(&mut *r, value)
    }
}

use core::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! take {
    ($p: expr) => {
        $crate::TakeHelper::new($p).take()
    };
    ($p: expr,) => {
        $crate::take!($p)
    };
}

pub struct TakeHelper<T>(TakeHelper2<T>);
pub struct TakeHelper2<T>(TakeHelper3<T>);
pub struct TakeHelper3<T>(T);

impl<T> TakeHelper<T> {
    pub fn new(inner: T) -> Self {
        TakeHelper(TakeHelper2(TakeHelper3(inner)))
    }
}

impl<'a, T> TakeHelper<&'a mut T>
where
    T: Copy,
{
    pub fn take(&mut self) -> T {
        *self.0 .0 .0
    }
}

impl<'a, T> TakeHelper<&'a Cell<T>>
where
    T: Copy,
{
    pub fn take(&mut self) -> T {
        self.0 .0 .0.get()
    }
}

impl<'a, T> TakeHelper<&'a RefCell<T>>
where
    T: Copy,
{
    pub fn take(&mut self) -> T {
        let r = self.0 .0 .0.borrow();
        *r
    }
}

impl<T> Deref for TakeHelper<T> {
    type Target = TakeHelper2<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TakeHelper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> TakeHelper2<&'a mut T>
where
    T: Default,
{
    pub fn take(&mut self) -> T {
        mem::take(self.0 .0)
    }
}

impl<'a, T> TakeHelper2<&'a Cell<T>>
where
    T: Default,
{
    pub fn take(&mut self) -> T {
        self.0 .0.take()
    }
}

impl<'a, T> TakeHelper2<&'a RefCell<T>>
where
    T: Default,
{
    pub fn take(&mut self) -> T {
        let mut r = self.0 .0.borrow_mut();
        mem::take(&mut *r)
    }
}

impl<T> Deref for TakeHelper2<T> {
    type Target = TakeHelper3<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TakeHelper2<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> TakeHelper3<&'a mut T>
where
    T: Clone,
{
    pub fn take(&mut self) -> T {
        self.0.clone()
    }
}

impl<'a, T> TakeHelper3<&'a RefCell<T>>
where
    T: Clone,
{
    pub fn take(&mut self) -> T {
        let r = self.0.borrow();
        r.clone()
    }
}
