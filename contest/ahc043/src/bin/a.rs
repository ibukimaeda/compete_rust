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

use core::panic;
use std::cmp;
use std::cmp::Ordering;
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
    input!(N:usize, M:usize, K:i64, T:usize, st: [(usize, usize, usize, usize); M]);
    // N = 50：区画の縦・横のマス数
    // 50 <= M <= 1600：人数
    // 11000 <= K <= 20000：初期資金
    // T = 800：ターン数
    // 線路設置：100
    // 駅設置：  5000
    // 各ターン：線路設置 or 駅設置 or 何もしない
    // 距離はマンハッタン距離
    // 会社までの距離は 5 以上
    // 利用されるのは家と会社両方で距離が 2 以下
    //   o
    //  ooo
    // ooHoo
    //  ooo
    //   o

    const RAIL_COST: i64 = 100;
    const STATION_COST: i64 = 5000;

    // 適当に初期の駅を2つ置く
    // 家または会社が駅の範囲にあるものの中で最も遠いものを選ぶ
    // その駅から家または会社までの距離が 2 以下になるように線路を敷く

    let mut income = 0; // 毎ターンの収入（各ターンの終了時に取得）
    let mut funds = K; // 資金
    let mut now_time = 0;
    let mut grid = nested_vec!(GridType::Empty; N; N);
    let mut state = State {
        income,
        funds,
        now_time,
        grid,
        now_place: (0, 0),
        prev_direction: Direction::Nothing,
    };

    // 資金が足りる中で一番遠い所に線路を敷く
    // 初期資金で置けるレールの数
    let num_rails = (K - 2 * STATION_COST) / RAIL_COST;
    let mut farthest_idx = 0;
    let mut farthest_dist = 0;
    for i in 0..st.len() {
        let dist = manhattan_distance(st[i].0, st[i].1, st[i].2, st[i].3);
        if dist > farthest_dist && (dist - 1) <= num_rails as i64 {
            farthest_dist = dist;
            farthest_idx = i;
        }
    }

    let start = (st[farthest_idx].0, st[farthest_idx].1);
    let goal = (st[farthest_idx].2, st[farthest_idx].3);
    let mut now_place = start;
    let mut prev_place = (!0, !0);
    let mut prev_direction = Direction::Nothing;
    while now_time < T {
        // 現在地に移動方向を決めた後に，線路を置く・駅を置く・何もしないかを決める

        // 移動方向
        let direction = if now_place.0 > goal.0 {
            Direction::Up
        } else if now_place.0 < goal.0 {
            Direction::Down
        } else if now_place.1 > goal.1 {
            Direction::Left
        } else if now_place.1 < goal.1 {
            Direction::Right
        } else {
            Direction::Nothing
        };

        if now_place == prev_place {
            say_action(Action::DoNothing, &mut state);
        } else if now_place == start {
            say_action(Action::BuildStation(start.0, start.1), &mut state);
        } else if now_place == goal {
            say_action(Action::BuildStation(goal.0, goal.1), &mut state);
        } else {
            say_action(
                Action::BuildRail(
                    get_rail(&prev_direction, &direction),
                    now_place.0,
                    now_place.1,
                ),
                &mut state,
            );
        }

        prev_place = now_place;
        prev_direction = direction;
        now_place = get_next_place(&now_place, &direction);

        debug!(now_time, now_place, prev_place, prev_direction);
    }
}

struct State {
    income: i64,
    funds: i64,
    now_time: usize,
    grid: Vec<Vec<GridType>>,
    now_place: (usize, usize),
    prev_direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rail {
    LR = 1,
    UD = 2,
    LD = 3,
    LU = 4,
    UR = 5,
    RD = 6,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Nothing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridType {
    Empty,
    Rail(Rail),
    Station,
}

enum Action {
    BuildRail(Rail, usize, usize),
    BuildStation(usize, usize),
    DoNothing,
}

fn get_rail(prevdirection: &Direction, direction: &Direction) -> Rail {
    match (prevdirection, direction) {
        (Direction::Up, Direction::Up) => Rail::UD,
        (Direction::Down, Direction::Down) => Rail::UD,
        (Direction::Left, Direction::Left) => Rail::LR,
        (Direction::Right, Direction::Right) => Rail::LR,
        // 方向を変える際は prev_direction は逆方向のレールを置く
        (Direction::Up, Direction::Left) => Rail::LD,
        (Direction::Up, Direction::Right) => Rail::RD,
        (Direction::Down, Direction::Left) => Rail::LU,
        (Direction::Down, Direction::Right) => Rail::UR,
        (Direction::Left, Direction::Up) => Rail::UR,
        (Direction::Left, Direction::Down) => Rail::RD,
        (Direction::Right, Direction::Up) => Rail::LU,
        (Direction::Right, Direction::Down) => Rail::LD,
        // (Direction::Nothing, Direction::Up) => Rail::UD,
        // (Direction::Nothing, Direction::Down) => Rail::UD,
        // (Direction::Nothing, Direction::Left) => Rail::LR,
        // (Direction::Nothing, Direction::Right) => Rail::LR,
        // (Direction::Up, Direction::Nothing) => Rail::UD,
        // (Direction::Down, Direction::Nothing) => Rail::UD,
        // (Direction::Left, Direction::Nothing) => Rail::LR,
        // (Direction::Right, Direction::Nothing) => Rail::LR,
        _ => panic!("Invalid direction"),
    }
}

fn get_next_place(now_place: &(usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (now_place.0 - 1, now_place.1),
        Direction::Down => (now_place.0 + 1, now_place.1),
        Direction::Left => (now_place.0, now_place.1 - 1),
        Direction::Right => (now_place.0, now_place.1 + 1),
        Direction::Nothing => *now_place,
    }
}

fn say_action(action: Action, state: &mut State) {
    state.now_time += 1;
    match action {
        Action::BuildRail(rail, x, y) => {
            println!("{} {} {}", rail as usize, x, y);
        }
        Action::BuildStation(x, y) => {
            println!("0 {} {}", x, y);
        }
        Action::DoNothing => {
            println!("-1");
        }
    }
}

fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> i64 {
    (x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
struct AstarNode {
    cost: usize,
    position: Point,
    priority: usize,
}
#[allow(dead_code)]
impl Ord for AstarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}
#[allow(dead_code)]
impl PartialOrd for AstarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[allow(dead_code)]
struct AStar {}

#[allow(dead_code)]
impl AStar {
    fn new() -> Self {
        AStar {}
    }

    fn _heuristic(a: Point, b: Point) -> usize {
        (a.row as isize - b.row as isize).abs() as usize
            + (a.col as isize - b.col as isize).abs() as usize
    }

    fn astar(&self, start: Point, goal: Point, state: &State) -> Option<FxHashMap<Point, Point>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from: FxHashMap<Point, Point> = FxHashMap::default();
        let mut g_score: FxHashMap<Point, usize> = FxHashMap::default();
        g_score.insert(start, 0);
        open_set.push(AstarNode {
            cost: 0,
            position: start,
            priority: Self::_heuristic(start, goal),
        });
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some(AstarNode { position, .. }) = open_set.pop() {
            if position == goal {
                // let mut path = vec![position];
                // while let Some(&prev) = came_from.get(&path[0]) {
                //     path.insert(0, prev);
                // }
                // return Some(path);
                return Some(came_from);
            }
            let current_cost = *g_score.get(&position).unwrap_or(&usize::MAX);
            for (dx, dy) in directions.iter() {
                let new_x = position.row as isize + dx;
                let new_y = position.col as isize + dy;
                if new_x < 0 || new_y < 0 {
                    continue;
                }
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if new_x >= state.grid.len()
                    || new_y >= state.grid[0].len()
                    || state.grid[new_x][new_y] != GridType::Empty
                {
                    continue;
                }
                let new_pos = Point {
                    row: new_x,
                    col: new_y,
                };
                let tentative_g_score = current_cost + 1;
                if tentative_g_score < *g_score.get(&new_pos).unwrap_or(&usize::MAX) {
                    came_from.insert(new_pos, position);
                    g_score.insert(new_pos, tentative_g_score);
                    let priority = tentative_g_score + Self::_heuristic(new_pos, goal);
                    open_set.push(AstarNode {
                        cost: tentative_g_score,
                        position: new_pos,
                        priority,
                    });
                }
            }
        }
        None
    }

    fn find_path(&self, start: Point, goal: Point, state: &State) -> Option<Vec<Point>> {
        let came_from = self.astar(start, goal, state)?;
        let mut path = vec![goal];
        let mut current = goal;
        // TODO もう少し効率的に書く
        while current != start {
            current = *came_from.get(&current).unwrap();
            path.push(current);
        }
        path.reverse();
        Some(path)
    }

    fn find_path_direction(
        &self,
        start: Point,
        goal: Point,
        state: &State,
    ) -> Option<Vec<Direction>> {
        let path = self.find_path(start, goal, state)?;
        let mut directions = vec![];
        for i in 0..path.len() - 1 {
            let current = path[i];
            let next = path[i + 1];
            if current.row < next.row {
                directions.push(Direction::Down);
            } else if current.row > next.row {
                directions.push(Direction::Up);
            } else if current.col < next.col {
                directions.push(Direction::Right);
            } else if current.col > next.col {
                directions.push(Direction::Left);
            }
        }
        Some(directions)
    }
}

// ###########################################################################################################

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
