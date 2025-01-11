#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::{div_ceil, div_floor, gcd, lcm};
use proconio::{
    fastout, input, input_interactive,
    marker::{Chars, Isize1, Usize1},
};
use rand::{thread_rng, Rng};

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
    input!(mut board: [Chars; 8]);

    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == '#' {
                // 4方向をすべて 'a' に変更する
                for k in 0..4 {
                    let mut x = i;
                    let mut y = j;

                    loop {
                        let new = updated_coordinate(x, y, DX[k], DY[k], 8, 8);
                        if new.is_none() {
                            break;
                        }
                        let new = new.unwrap();
                        x = new.0;
                        y = new.1;

                        if board[x][y] == '#' {
                            continue;
                        }

                        board[x][y] = 'a';
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == '.' {
                ans += 1;
            }
        }
    }

    say(ans);
}

#[allow(non_snake_case)]
fn is_in(now: (usize, usize), dx: i64, dy: i64, H: usize, W: usize) -> bool {
    let H = H as i64;
    let W = W as i64;
    let new_x = now.0 as i64 + dx;
    let new_y = now.1 as i64 + dy;
    return 0 <= new_x && new_x < H && 0 <= new_y && new_y < W;
}
#[allow(non_snake_case)]
fn updated_coordinate(
    x: usize,
    y: usize,
    dx: i64,
    dy: i64,
    H: usize,
    W: usize,
) -> Option<(usize, usize)> {
    if is_in((x, y), dx, dy, H, W) {
        return Some(((x as i64 + dx) as usize, (y as i64 + dy) as usize));
    } else {
        return None;
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
