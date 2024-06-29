#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input, input_interactive,
    marker::{Chars, Isize1, Usize1},
};
use rand::{thread_rng, Rng};
use std::cmp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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

const DX: [i64; 4] = [0, 0, 1, -1];
const DY: [i64; 4] = [1, -1, 0, 0];

#[allow(non_snake_case)]
fn main() {}

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
fn print_vec<T: std::fmt::Display>(v: &Vec<T>) {
    let mut first = true;
    for value in v {
        if first {
            #[cfg(debug_assertions)]
            eprint!("{}", *value);
            first = false;
        } else {
            #[cfg(debug_assertions)]
            eprint!(" {}", *value);
        }
    }
    #[cfg(debug_assertions)]
    eprintln!();
}

#[allow(dead_code)]
fn print_vec2<T: std::fmt::Display>(v: &Vec<Vec<T>>) {
    for _v in v {
        print_vec(_v);
    }
}

fn is_in(now: (usize, usize), dx: i64, dy: i64, H: usize, W: usize) -> bool {
    let H = H as i64;
    let W = W as i64;
    let new_x = now.0 as i64 + dx;
    let new_y = now.1 as i64 + dy;

    return 0 <= new_x && new_x < H && 0 <= new_y && new_y < W;
}

fn updated_coordinate(x: usize, y: usize, dx: i64, dy: i64) -> (usize, usize) {
    return ((x as i64 + dx) as usize, (y as i64 + dy) as usize);
}

#[derive(Debug)]
struct UnionFind {
    // https://qiita.com/ofutonton/items/c17dfd33fc542c222396
    data: Vec<i64>,
}

impl UnionFind {
    #[allow(dead_code)]
    fn new(size: usize) -> Self {
        UnionFind {
            data: vec![-1; size],
        }
    }

    #[allow(dead_code)]
    fn unite(&mut self, mut x: i64, mut y: i64) -> bool {
        // xと y を結合
        x = self.root(x);
        y = self.root(y);

        if x == y {
            return false;
        }
        if self.data[x as usize] > self.data[y as usize] {
            mem::swap(&mut x, &mut y);
        }

        self.data[x as usize] += self.data[y as usize];
        self.data[y as usize] = x;
        return true;
    }

    #[allow(dead_code)]
    fn root(&mut self, k: i64) -> i64 {
        // k の属する木の根を探索
        if self.data[k as usize] < 0 {
            return k;
        } else {
            self.data[k as usize] = self.root(self.data[k as usize]);
            return self.data[k as usize];
        }
    }

    #[allow(dead_code)]
    fn size(&mut self, k: i64) -> i64 {
        // k の属する木の大きさを返す
        let x: usize = self.root(k) as usize;
        return -self.data[x];
    }

    #[allow(dead_code)]
    fn is_same(&mut self, x: i64, y: i64) -> bool {
        // x と y の属する木が同じかどうか
        return self.root(x) == self.root(y);
    }

    #[allow(dead_code)]
    fn groups(&mut self) -> Vec<Vec<i64>> {
        let n = self.data.len();
        let mut ret: Vec<Vec<i64>> = vec![vec![0; 0]; n];
        for i in 0..n {
            ret[self.root(i as i64) as usize].push(i as i64);
        }

        let mut i = 0;
        while i < ret.len() {
            if ret[i].is_empty() {
                ret.remove(i);
            } else {
                i += 1;
            }
        }

        return ret;
    }
}

#[derive(Clone, Copy, Debug)]
struct ModInt {
    x: i64,
    modulo: i64,
}

impl ModInt {
    #[allow(dead_code)]
    fn new(x: i64, modulo: i64) -> Self {
        let x = if x >= 0 {
            x % modulo
        } else {
            (modulo - (-x) % modulo) % modulo
        };
        ModInt { x, modulo }
    }

    #[allow(dead_code)]
    fn set(&mut self, x: i64) {
        if x >= 0 {
            self.x = x % self.modulo
        } else {
            self.x = (self.modulo - (-x) % self.modulo) % self.modulo
        };
    }

    #[allow(dead_code)]
    fn inv(&self) -> Self {
        // (self.x)^-1
        // https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a
        let mut a = self.x;
        let mut b = self.modulo;
        let mut u: i64 = 1;
        let mut v: i64 = 0;

        while b > 0 {
            let t = a / b;
            a -= t * b;
            mem::swap(&mut a, &mut b);
            u -= t * v;
            mem::swap(&mut u, &mut v);
        }

        u %= self.modulo;
        if u < 0 {
            u += self.modulo;
        }

        return ModInt {
            x: u,
            modulo: self.modulo,
        };
    }

    #[allow(dead_code)]
    fn pow(&self, mut n: i64) -> Self {
        // (self.x)^n
        let mut a = self.x;
        let mut res: i64 = 1;

        while n > 0 {
            if n & 1 == 1 {
                res = (res * a) % self.modulo;
            }
            a = (a * a) % self.modulo;
            n >>= 1;
        }

        return ModInt {
            x: res,
            modulo: self.modulo,
        };
    }
}

impl fmt::Display for ModInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl ops::Neg for ModInt {
    type Output = ModInt;
    fn neg(self) -> Self::Output {
        ModInt::new(-self.x, self.modulo)
    }
}

impl ops::Add<ModInt> for ModInt {
    type Output = ModInt;
    fn add(self, rhs: Self) -> Self::Output {
        return ModInt::new(self.x + rhs.x, self.modulo);
    }
}

impl ops::Add<i64> for ModInt {
    type Output = ModInt;
    fn add(self, rhs: i64) -> Self::Output {
        return ModInt::new(self.x + rhs, self.modulo);
    }
}

impl<'a> ops::AddAssign<&'a Self> for ModInt {
    fn add_assign(&mut self, rhs: &Self) {
        self.set(self.x + rhs.x);
    }
}

impl ops::AddAssign<i64> for ModInt {
    fn add_assign(&mut self, rhs: i64) {
        self.set(self.x + rhs);
    }
}

impl ops::Sub<ModInt> for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: Self) -> Self::Output {
        return ModInt::new(self.x - rhs.x, self.modulo);
    }
}

impl ops::Sub<i64> for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: i64) -> Self::Output {
        return ModInt::new(self.x - rhs, self.modulo);
    }
}

impl<'a> ops::SubAssign<&'a Self> for ModInt {
    fn sub_assign(&mut self, rhs: &Self) {
        self.set(self.x - rhs.x);
    }
}

impl ops::SubAssign<i64> for ModInt {
    fn sub_assign(&mut self, rhs: i64) {
        self.set(self.x - rhs);
    }
}

impl ops::Mul<ModInt> for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: Self) -> Self::Output {
        ModInt::new(self.x * rhs.x, self.modulo)
    }
}

impl ops::Mul<i64> for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: i64) -> Self::Output {
        ModInt::new(self.x * rhs, self.modulo)
    }
}

impl<'a> ops::MulAssign<&'a Self> for ModInt {
    fn mul_assign(&mut self, rhs: &Self) {
        self.set(self.x * rhs.x);
    }
}

impl ops::MulAssign<i64> for ModInt {
    fn mul_assign(&mut self, rhs: i64) {
        self.set(self.x * rhs);
    }
}

impl ops::Div<ModInt> for ModInt {
    type Output = ModInt;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl ops::Div<i64> for ModInt {
    type Output = ModInt;
    fn div(self, rhs: i64) -> Self::Output {
        self * ModInt::new(rhs, self.modulo).inv()
    }
}

impl<'a> ops::DivAssign<&'a Self> for ModInt {
    fn div_assign(&mut self, rhs: &Self) {
        self.x *= rhs.inv().x;
    }
}

impl ops::DivAssign<i64> for ModInt {
    fn div_assign(&mut self, rhs: i64) {
        self.x *= ModInt::new(rhs, self.modulo).inv().x;
    }
}

impl cmp::PartialEq<ModInt> for ModInt {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x
    }
}

impl cmp::PartialEq<i64> for ModInt {
    fn eq(&self, other: &i64) -> bool {
        let other = ModInt::new(*other, self.modulo);
        self.x == other.x
    }

    fn ne(&self, other: &i64) -> bool {
        let other = ModInt::new(*other, self.modulo);
        self.x != other.x
    }
}

impl cmp::PartialOrd<ModInt> for ModInt {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.x == other.x {
            Some(cmp::Ordering::Equal)
        } else if self.x > other.x {
            Some(cmp::Ordering::Greater)
        } else {
            Some(cmp::Ordering::Less)
        }
    }
}

impl cmp::PartialOrd<i64> for ModInt {
    fn partial_cmp(&self, other: &i64) -> Option<cmp::Ordering> {
        let other = ModInt::new(*other, self.modulo);
        if self.x == other.x {
            Some(cmp::Ordering::Equal)
        } else if self.x > other.x {
            Some(cmp::Ordering::Greater)
        } else {
            Some(cmp::Ordering::Less)
        }
    }
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

#[allow(unused)]
fn rotated<T: Default + Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    // 2次配列を左回りに90度回転したものを返す
    let mut ret = nested_vec!(Default::default(); grid[0].len(); grid.len());
    for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            ret[i][j] = grid[grid.len() - 1 - j][i].clone();
        }
    }
    return ret;
}

#[allow(unused)]
fn shifted<T: Default + Clone>(grid: &Vec<Vec<T>>, dx: i64, dy: i64, default: T) -> Vec<Vec<T>> {
    // 2次元配列を下に dx 右に dy 動かしたものを返す
    let mut ret = nested_vec!(Default::default(); grid.len(); grid[0].len());
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut value = default.clone();

            let nx = i as i64 - dx;
            let ny = j as i64 - dy;

            if 0 <= nx && nx < grid.len() as i64 && 0 <= ny && ny < grid[0].len() as i64 {
                value = grid[nx as usize][ny as usize].clone();
            }

            ret[i][j] = value;
        }
    }
    return ret;
}
