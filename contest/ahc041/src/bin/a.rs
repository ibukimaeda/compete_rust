#![allow(non_snake_case)]
#![allow(unused_imports)]
use indexmap::IndexSet;
use itertools::Itertools;
use num_integer::{div_ceil, div_floor, gcd, lcm};
use petgraph::graph;
use proconio::{
    fastout, input, input_interactive,
    marker::{Chars, Isize1, Usize1},
};
use rand::{seq::SliceRandom, thread_rng, Rng};

use std::cmp;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::default;
use std::fmt;
use std::mem;
use std::ops;
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
    input!(
        N:usize, M:usize, H:i64,
        A:[i64;N],
        uv: [(usize, usize);M],
        xy: [(i64, i64);N],
    );
    // N = 1000
    // 1000 <= M <= 3000
    // H = 10
    // 1 <= A[i] <= 100
    // 0 <= u[i] < v[i] < N
    // 0<= x[i], y[i] <= 1000
    // (x[i], y[i]) はすべて異なる
    let mut rng = rand::thread_rng();

    let mut graph = vec![vec![]; N];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = vec![!0; N];
    // let mut leafs = (0..N).into_iter().collect_vec();
    // let mut leafs: HashSet<usize> = HashSet::from_iter((0..N).into_iter().collect_vec());
    let mut leafs: IndexSet<usize> = IndexSet::from_iter((0..N).into_iter().collect_vec());
    let mut num_children = vec![0; N];
    let mut hs = vec![0; N];
    let mut point = A.iter().sum::<i64>();

    // 連鎖的に親を変えていく
    let mut new_leaf = !0;

    let mut count = 0;
    let time_keeper = TimeKeeper::new(1.98);
    while !time_keeper.isTimeOver() {
        count += 1;

        if leafs.is_empty() {
            break;
        }
        // let &leaf = leafs.choose(&mut thread_rng()).unwrap();

        // hashset ある程度ランダムらしい
        // let &leaf = leafs.iter().next().unwrap();

        // indexset インデックスによるアクセスができる
        let leaf;
        if new_leaf == !0 {
            leaf = leafs[rng.gen_range(0..leafs.len())];
        } else {
            leaf = new_leaf;
            new_leaf = !0;
        }

        // 現在の木から外した時のポイント
        let (new_point, new_parent) = {
            // 現在の木から外す
            let mut new_point = point - A[leaf] * (hs[leaf] + 1);

            // 順に leaf の辺を見ていき，接続できそうであれば，接続しポイントを計算する
            let mut new_parent = !0;
            for &parent in &graph[leaf] {
                if new_parent == !0 && hs[parent] < H {
                    new_parent = parent;
                } else if hs[parent] < H && hs[new_parent] < hs[parent] {
                    new_parent = parent;
                }
            }

            // debug!(new_parent, hs[new_parent]);
            if new_parent != !0 {
                new_point += A[leaf] * (hs[new_parent] + 2);
                // debug!(hs[leaf] + 1, hs[new_parent] + 2, point < new_point);
            }

            (new_point, new_parent)
        };

        // debug!(leaf, point, new_point, new_parent);

        if point < new_point {
            // 親を変更したほうがポイントが高い

            let old_parent = ans[leaf];
            if old_parent != !0 {
                // すでに親がいる場合は，その親の子供の数を減らす
                num_children[ans[leaf]] -= 1;

                // その親が葉になる可能性があるので，leafs に追加する
                if num_children[ans[leaf]] == 0 {
                    leafs.insert(ans[leaf]);
                    new_leaf = old_parent;
                }
            }

            point = new_point;
            ans[leaf] = new_parent;
            leafs.remove(&new_parent);
            num_children[new_parent] += 1;
            hs[leaf] = hs[new_parent] + 1;
        }

        // if count % 100 == 0 {
        //     say_ans(&ans);
        //     debug!(leafs.len());
        // }
    }

    debug!(count);

    say_ans(&ans);
}

fn say_ans(ans: &Vec<usize>) {
    println!(
        "{}",
        ans.iter()
            .map(|&x| if x != !0 { x as i64 } else { !0 })
            .join(" ")
    );
}

#[derive(Debug)]
struct UnionFind {
    data: Vec<i32>,
}
impl UnionFind {
    #[allow(dead_code)]
    fn new(size: usize) -> Self {
        UnionFind {
            data: vec![-1; size],
        }
    }
    #[allow(dead_code)]
    fn unite(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.data.len());
        assert!(y < self.data.len());
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return false;
        }
        if self.data[x] > self.data[y] {
            mem::swap(&mut x, &mut y);
        }
        self.data[x] += self.data[y];
        self.data[y] = x as i32;
        return true;
    }
    #[allow(dead_code)]
    fn root(&mut self, k: usize) -> usize {
        assert!(k < self.data.len());
        if self.data[k as usize] < 0 {
            return k;
        }
        self.data[k as usize] = self.root(self.data[k] as usize) as i32;
        return self.data[k] as usize;
    }
    #[allow(dead_code)]
    fn size(&mut self, k: usize) -> usize {
        assert!(k < self.data.len());
        let x = self.root(k);
        return -self.data[x] as usize;
    }
    #[allow(dead_code)]
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.data.len());
        assert!(y < self.data.len());
        return self.root(x) == self.root(y);
    }
    #[allow(dead_code)]
    fn groups(&mut self) -> Vec<Vec<usize>> {
        let n = self.data.len();
        let mut root_buf = vec![0; n];
        let mut group_size = vec![0; n];
        for i in 0..n {
            root_buf[i] = self.root(i);
            group_size[root_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); n];
        for i in 0..n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..n {
            result[root_buf[i]].push(i);
        }
        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
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
                write!(output, "{}", val).unwrap();
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
                    write!(output, "{}", val).unwrap();
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
#[derive(Debug, Clone)]
struct TimeKeeper {
    start_time: std::time::Instant,
    time_threshold: f64,
}

impl TimeKeeper {
    fn new(time_threshold: f64) -> Self {
        TimeKeeper {
            start_time: std::time::Instant::now(),
            time_threshold,
        }
    }
    #[inline]
    fn isTimeOver(&self) -> bool {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            elapsed_time * 0.85 >= self.time_threshold
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time >= self.time_threshold
        }
    }
}
