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

const EMPTY_COST: i64 = 0;
const RAIL_COST: i64 = 100;
const STATION_COST: i64 = 5000;

#[allow(non_snake_case)]
fn main() {
    let mut rng = Xorshift::new();

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

    // 適当に初期の駅を2つ置く
    // 家または会社が駅の範囲にあるものの中で最も遠いものを選ぶ
    // その駅から家または会社までの距離が 2 以下になるように線路を敷く

    let mut state = State::new(N, M, K, T, st.clone());

    let astar = AStar::new();

    // 資金が足りる中で一番遠い所に線路を敷く
    // 初期資金で置けるレールの数
    let num_rails = (K - 2 * STATION_COST) / RAIL_COST;
    let mut farthest = (Point { row: 0, col: 0 }, Point { row: 0, col: 0 });
    let mut farthest_dist = 0;
    for (&home, v) in state.home_to_work_place.iter() {
        for &(work, _) in v {
            let dist = manhattan_distance(home, work);
            if dist > farthest_dist && (dist - 1) <= num_rails as i64 {
                farthest_dist = dist;
                farthest = (home, work);
            }
        }
    }

    debug_assert_ne!(
        farthest,
        (Point { row: 0, col: 0 }, Point { row: 0, col: 0 })
    );

    debug!(farthest);

    let start = farthest.0;
    let goal = farthest.1;

    let path = astar.find_path_direction(start, goal, &state).unwrap();

    state.action(Action::BuildStation(start));

    state.laying(start, goal, &path);

    // state.say_answer();
    // return;

    while state.now_time() < 400 || (state.income > 150 && state.now_time() < 600) {
        // 全探索して，最も効果的なレールを敷く

        let mut candidates = LimitedIntervalHeap::new(5);
        let mut selected_start = Point { row: !0, col: !0 };
        let mut selected_goal = Point { row: !0, col: !0 };
        let mut selected_path = vec![];
        for i in 0..M {
            // 駅からの距離が近い かつ 家・会社の距離が遠いものが良い
            if state.commuters.is_same(i, i + M) {
                continue;
            }

            let (home, work) = state.st[i];
            let tuukin_kyori = manhattan_distance(home, work);

            let exist_home_station = state.near_station.contains_key(&home);
            let exist_work_station = state.near_station.contains_key(&work);

            let goal;
            if exist_home_station && !exist_work_station {
                goal = work;
            } else if !exist_home_station && exist_work_station {
                goal = home;
            } else {
                continue;
            }

            for (&station, _) in &state.stations {
                let dist = manhattan_distance(station, goal);
                let gap = tuukin_kyori - dist;

                let permissible_turn = 400;
                // ターン以内に建築できないものは無視（距離だけで概算）
                if RAIL_COST * (dist - 1) + STATION_COST
                    > state.funds + state.income * permissible_turn
                {
                    debug!(
                        RAIL_COST * (dist - 1) + STATION_COST,
                        state.funds + state.income * permissible_turn,
                        state.funds,
                        state.income
                    );
                    continue;
                }

                candidates.push((gap, station, goal));
            }
        }

        debug!(candidates);
        while let Some((_, start, goal)) = candidates.pop() {
            if let Some(path) = astar.find_path_direction(start, goal, &state) {
                debug!(path.len());

                if selected_path.len() == 0 || path.len() < selected_path.len() {
                    selected_start = start;
                    selected_goal = goal;
                    selected_path = path;
                }
            }
        }

        if selected_path.len() == 0 {
            break;
        }
        state.laying(selected_start, selected_goal, &selected_path);
    }

    state.say_answer();
}

struct State {
    N: usize,                                                  // 区画の縦・横のマス数
    M: usize,                                                  // 人数
    T: usize,                                                  // ターン数
    st: Vec<(Point, Point)>,                                   // 家と会社の位置
    income: i64,               // 毎ターンの収入（各ターンの終了時に取得）
    funds: i64,                // 資金
    grid: Vec<Vec<GridType>>,  // グリッドの状態
    now_place: Point,          // 現在地
    prev_direction: Direction, // 前回の移動方向
    commuters: UnionFind, // 通勤者の情報（家と会社が同じ路線につながっているか） i が家，i+M が会社
    home_to_work_place: FxHashMap<Point, Vec<(Point, usize)>>, // 家 -> (会社, 2i)
    work_to_home_place: FxHashMap<Point, Vec<(Point, usize)>>, // 会社 -> (家, i)
    near_station: FxHashMap<Point, Point>, // 家・会社に最も近い駅の位置
    stations: FxHashMap<Point, usize>, // 駅の番号 k (> 2*M)
    actions: Vec<(Action, i64)>, // 行動
}

impl State {
    #[inline(always)]
    fn now_time(&self) -> usize {
        self.actions.len()
    }
}

impl State {
    const STATION_NEAR: [(i64, i64); 13] = [
        (-2, 0),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -2),
        (0, -1),
        (0, 0),
        (0, 1),
        (0, 2),
        (1, -1),
        (1, 0),
        (1, 1),
        (2, 0),
    ];

    fn new(
        N: usize,
        M: usize,
        funds: i64,
        T: usize,
        st: Vec<(usize, usize, usize, usize)>,
    ) -> Self {
        // i 番目の通勤者の対応番号は i, i+M
        // commuters.is_same(i, i+M) が true ならば i 番目の通勤者は電車を利用する
        // k 番目(0 始まり)に作られた駅は M + k - 1

        let commuters = UnionFind::new(2 * M + 100); // 100 駅以上作るようになれば，変更する
        let mut home_to_work_place = FxHashMap::default();
        let mut work_to_home_place = FxHashMap::default();
        for (i, &(x1, y1, x2, y2)) in st.iter().enumerate() {
            home_to_work_place
                .entry(Point { row: x1, col: y1 })
                .or_insert(vec![])
                .push((Point { row: x2, col: y2 }, i));
            work_to_home_place
                .entry(Point { row: x2, col: y2 })
                .or_insert(vec![])
                .push((Point { row: x1, col: y1 }, i));
        }

        State {
            N,
            M,
            T,
            st: st
                .iter()
                .map(|&(x, y, z, w)| (Point { row: x, col: y }, Point { row: z, col: w }))
                .collect(),
            income: 0,
            funds: funds,
            grid: nested_vec!(GridType::Empty; N; N),
            now_place: Point { row: !0, col: !0 },
            prev_direction: Direction::Nothing,
            commuters,
            home_to_work_place,
            work_to_home_place,
            near_station: FxHashMap::default(),
            stations: FxHashMap::default(),
            actions: vec![],
        }
    }

    fn laying(&mut self, start: Point, goal: Point, path: &Vec<Direction>) {
        // start -> goal までのパスを敷設する
        // start には駅が設置されてあるとして敷設する

        debug_with_message!("laying: ", start, goal, path);
        debug_assert!(path.len() > 0);

        self.prev_direction = path[0];
        self.now_place = get_next_place(&start, &self.prev_direction);

        for i in 1..path.len() {
            let direction = path[i];
            let next_place = get_next_place(&self.now_place, &direction);

            self.action_with_wait(Action::BuildRail(
                get_rail(&self.prev_direction, &direction),
                self.now_place,
            ));

            self.prev_direction = direction;
            self.now_place = next_place;
        }
        debug_with_message!("laying: ", self.now_place, goal);

        debug_assert_eq!(self.now_place, goal);
        self.action_with_wait(Action::BuildStation(self.now_place));

        let start_station_number = self.stations.get(&start).unwrap();
        let goal_station_number = self.stations.get(&goal).unwrap();
        self.commuters
            .unite(*start_station_number, *goal_station_number);
        debug_with_message!("unite", *start_station_number, *goal_station_number);

        self.update_income();
    }

    fn require_wait(&self, path: &Vec<Direction>) -> i64 {
        let mut require_turn = 0;
        let require_cost = (path.len() as i64 - 1) * RAIL_COST + STATION_COST;

        let mut funds = self.funds;
        while funds < require_cost {
            debug_assert!(self.income > 0, "No income");
            require_turn += 1;
            funds += self.income;
        }

        require_turn
    }

    fn update_income(&mut self) {
        self.income = 0;
        for (&home, works) in &self.home_to_work_place {
            for &(work, i) in works {
                if self.commuters.is_same(i, i + self.M) {
                    self.income += manhattan_distance(home, work);
                }
            }
        }
        debug_with_message!("update_income: ", self.income);
    }

    fn action_with_wait(&mut self, action: Action) {
        debug_with_message!(
            "start action_with_wait: ",
            self.funds,
            self.income,
            self.now_time()
        );
        let require_funds = match action {
            Action::BuildRail(_, _) => RAIL_COST,
            Action::BuildStation(_) => STATION_COST,
            Action::DoNothing => EMPTY_COST,
        };

        while require_funds > self.funds {
            assert!(self.income > 0, "No income");
            self.action(Action::DoNothing);
        }

        debug_with_message!(
            "action_with_wait: ",
            require_funds,
            self.funds,
            self.income,
            self.now_time()
        );

        self.action(action);
    }

    fn action(&mut self, action: Action) {
        match action {
            Action::BuildRail(rail, Point { row, col }) => {
                debug_assert!(self.funds >= RAIL_COST);
                self.funds -= RAIL_COST;
                self.grid[row][col] = GridType::Rail(rail);
            }
            Action::BuildStation(Point { row, col }) => {
                debug_assert!(self.funds >= STATION_COST);
                self.funds -= STATION_COST;
                self.grid[row][col] = GridType::Station;

                let station_number = 2 * self.M + self.stations.len();
                self.stations.insert(Point { row, col }, station_number);

                for &(drow, dcol) in &Self::STATION_NEAR {
                    // TODO 駅の範囲がかぶっている場合，上書きされるのをどうするか
                    debug_with_message!("action: ", row, col, drow, dcol);
                    if let Some((nx, ny)) = updated_coordinate(row, col, drow, dcol, self.N, self.N)
                    {
                        if let Some(works) =
                            self.home_to_work_place.get(&Point { row: nx, col: ny })
                        {
                            for &(_, i) in works {
                                self.near_station
                                    .insert(Point { row: nx, col: ny }, Point { row, col });
                                self.commuters.unite(i, station_number);
                                debug_with_message!("unite: ", i, station_number);
                            }
                        }

                        if let Some(homes) =
                            self.work_to_home_place.get(&Point { row: nx, col: ny })
                        {
                            for &(_, i) in homes {
                                debug_with_message!("home: ", i, station_number);
                                self.near_station
                                    .insert(Point { row: nx, col: ny }, Point { row, col });
                                self.commuters.unite(i + self.M, station_number);
                                debug_with_message!("unite: ", i + self.M, station_number, i);
                            }
                        }
                    }
                }

                // self.update_income();
            }
            Action::DoNothing => {}
        }

        self.funds += self.income;
        self.actions.push((action, self.funds));
    }

    fn say_answer(&mut self) {
        debug_assert!(
            self.now_time() <= self.T,
            "Too many actions: {}",
            self.now_time()
        );

        for (action, funds) in &self.actions {
            match *action {
                Action::BuildRail(rail, Point { row, col }) => {
                    println!("{} {} {}", rail as usize, row, col);
                }
                Action::BuildStation(Point { row, col }) => {
                    println!("0 {} {}", row, col);
                }
                Action::DoNothing => {
                    println!("-1");
                }
            }

            println!("# funds = {}", funds);
        }

        for _ in self.now_time()..self.T {
            println!("-1");
        }
    }
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
    BuildRail(Rail, Point),
    BuildStation(Point),
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

fn get_next_place(now_place: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::Up => Point {
            row: now_place.row - 1,
            col: now_place.col,
        },
        Direction::Down => Point {
            row: now_place.row + 1,
            col: now_place.col,
        },
        Direction::Left => Point {
            row: now_place.row,
            col: now_place.col - 1,
        },
        Direction::Right => Point {
            row: now_place.row,
            col: now_place.col + 1,
        },
        Direction::Nothing => Point {
            row: now_place.row,
            col: now_place.col,
        },
    }
}

fn manhattan_distance(p1: Point, p2: Point) -> i64 {
    (p1.row as i64 - p2.row as i64).abs() + (p1.col as i64 - p2.col as i64).abs()
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

                // TODO Empty 以外にも駅を設置する等の操作ができる
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
#[allow(non_snake_case)]
#[inline(always)]
fn is_in(now: (usize, usize), dx: i64, dy: i64, H: usize, W: usize) -> bool {
    let H = H as i64;
    let W = W as i64;
    let new_x = now.0 as i64 + dx;
    let new_y = now.1 as i64 + dy;
    return 0 <= new_x && new_x < H && 0 <= new_y && new_y < W;
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[inline(always)]
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

#[derive(Debug)]
struct UnionFind {
    parent: Vec<i32>,
}
impl UnionFind {
    #[allow(dead_code)]
    fn new(size: usize) -> Self {
        UnionFind {
            parent: vec![-1; size],
        }
    }
    #[allow(dead_code)]
    fn unite(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.parent.len());
        assert!(y < self.parent.len());
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return false;
        }
        if self.parent[x] > self.parent[y] {
            mem::swap(&mut x, &mut y);
        }
        self.parent[x] += self.parent[y];
        self.parent[y] = x as i32;
        return true;
    }
    #[allow(dead_code)]
    fn root(&mut self, k: usize) -> usize {
        assert!(k < self.parent.len());
        if self.parent[k as usize] < 0 {
            return k;
        }
        self.parent[k as usize] = self.root(self.parent[k] as usize) as i32;
        return self.parent[k] as usize;
    }
    #[allow(dead_code)]
    fn size(&mut self, k: usize) -> usize {
        assert!(k < self.parent.len());
        let x = self.root(k);
        return -self.parent[x] as usize;
    }
    #[allow(dead_code)]
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.parent.len());
        assert!(y < self.parent.len());
        return self.root(x) == self.root(y);
    }
    #[allow(dead_code)]
    fn groups(&mut self) -> Vec<Vec<usize>> {
        let n = self.parent.len();
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct Xorshift {
    seed: u64,
}
impl Xorshift {
    #[allow(dead_code)]
    pub fn new() -> Xorshift {
        Xorshift {
            seed: 0xf0fb588ca2196dac,
        }
    }
    #[allow(dead_code)]
    pub fn with_seed(seed: u64) -> Xorshift {
        Xorshift { seed: seed }
    }
    #[inline]
    #[allow(dead_code)]
    pub fn next(&mut self) -> u64 {
        self.seed = self.seed ^ (self.seed << 13);
        self.seed = self.seed ^ (self.seed >> 7);
        self.seed = self.seed ^ (self.seed << 17);
        self.seed
    }
    #[inline]
    #[allow(dead_code)]
    pub fn rand(&mut self, m: u64) -> u64 {
        self.next() % m
    }
    #[inline]
    #[allow(dead_code)]
    pub fn rand_range(&mut self, l: u64, r: u64) -> u64 {
        // [l, r) の整数を返す
        self.rand(r - l) + l
    }
    #[inline]
    #[allow(dead_code)]
    pub fn randf(&mut self) -> f64 {
        use std::mem;
        const UPPER_MASK: u64 = 0x3FF0000000000000;
        const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
        let tmp = UPPER_MASK | (self.next() & LOWER_MASK);
        let result: f64 = unsafe { mem::transmute(tmp) };
        result - 1.0
    }
}

#[derive(Clone, Debug)]
struct IntervalHeap<T: Ord + Eq> {
    data: Vec<T>,
}
impl<T: Ord + Eq> IntervalHeap<T> {
    #[allow(dead_code)]
    fn new() -> IntervalHeap<T> {
        IntervalHeap { data: Vec::new() }
    }
    #[allow(dead_code)]
    fn with_capacity(n: usize) -> IntervalHeap<T> {
        IntervalHeap {
            data: Vec::with_capacity(n),
        }
    }
    #[allow(dead_code)]
    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
    #[allow(dead_code)]
    #[inline]
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    #[allow(dead_code)]
    #[inline]
    fn push(&mut self, x: T) {
        let i = self.data.len();
        self.data.push(x);
        self.up(i);
    }
    #[allow(dead_code)]
    #[inline]
    fn peek_min(&self) -> Option<&T> {
        self.data.first()
    }
    #[allow(dead_code)]
    #[inline]
    fn peek_max(&self) -> Option<&T> {
        if self.data.len() > 1 {
            self.data.get(1)
        } else {
            self.data.first()
        }
    }
    #[allow(dead_code)]
    #[inline]
    fn pop_min(&mut self) -> Option<T> {
        if self.data.len() == 1 {
            return self.data.pop();
        }
        if self.data.is_empty() {
            return None;
        }
        let len = self.data.len();
        self.data.swap(0, len - 1);
        let res = self.data.pop();
        self.down(0);
        res
    }
    #[allow(dead_code)]
    #[inline]
    fn pop_max(&mut self) -> Option<T> {
        if self.data.len() <= 2 {
            return self.data.pop();
        }
        if self.data.is_empty() {
            return None;
        }
        let len = self.data.len();
        self.data.swap(1, len - 1);
        let res = self.data.pop();
        self.down(1);
        res
    }
    #[allow(dead_code)]
    #[inline]
    fn parent(i: usize) -> usize {
        ((i >> 1) - 1) & !1
    }
    #[allow(dead_code)]
    #[inline]
    fn down(&mut self, i: usize) {
        let mut i = i;
        let n = self.data.len();
        if i & 1 == 0 {
            while (i << 1) + 2 < n {
                let mut k = (i << 1) + 2;
                if k + 2 < n
                    && unsafe { self.data.get_unchecked(k + 2) }
                        < unsafe { self.data.get_unchecked(k) }
                {
                    k = k + 2;
                }
                if unsafe { self.data.get_unchecked(i) } > unsafe { self.data.get_unchecked(k) } {
                    self.data.swap(i, k);
                    i = k;
                    if i + 1 < self.data.len()
                        && unsafe { self.data.get_unchecked(i) }
                            > unsafe { self.data.get_unchecked(i + 1) }
                    {
                        self.data.swap(i, i + 1);
                    }
                } else {
                    break;
                }
            }
        } else {
            while (i << 1) + 1 < n {
                let mut k = (i << 1) + 1;
                if k + 2 < n
                    && unsafe { self.data.get_unchecked(k + 2) }
                        > unsafe { self.data.get_unchecked(k) }
                {
                    k = k + 2;
                }
                if unsafe { self.data.get_unchecked(i) } < unsafe { self.data.get_unchecked(k) } {
                    self.data.swap(i, k);
                    i = k;
                    if i > 0
                        && unsafe { self.data.get_unchecked(i) }
                            < unsafe { self.data.get_unchecked(i - 1) }
                    {
                        self.data.swap(i, i - 1);
                    }
                } else {
                    break;
                }
            }
        }
    }
    #[allow(dead_code)]
    #[inline]
    fn up(&mut self, i: usize) {
        let mut i = i;
        if i & 1 == 1
            && unsafe { self.data.get_unchecked(i) } < unsafe { self.data.get_unchecked(i - 1) }
        {
            self.data.swap(i, i - 1);
            i -= 1;
        }
        while i > 1
            && unsafe { self.data.get_unchecked(i) }
                < unsafe { self.data.get_unchecked(Self::parent(i)) }
        {
            let p = Self::parent(i);
            self.data.swap(i, p);
            i = p;
        }
        while i > 1
            && unsafe { self.data.get_unchecked(i) }
                > unsafe { self.data.get_unchecked(Self::parent(i) + 1) }
        {
            let p = Self::parent(i) + 1;
            self.data.swap(i, p);
            i = p;
        }
    }
    #[allow(dead_code)]
    #[inline]
    fn clear(&mut self) {
        self.data.clear();
    }
}
#[derive(Clone, Debug)]
struct LimitedIntervalHeap<T: Ord + Eq> {
    heap: IntervalHeap<T>,
    limit: usize,
}
impl<T: Ord + Eq> LimitedIntervalHeap<T> {
    #[allow(dead_code)]
    fn new(limit: usize) -> LimitedIntervalHeap<T> {
        LimitedIntervalHeap {
            heap: IntervalHeap::with_capacity(limit),
            limit: limit,
        }
    }
    #[allow(dead_code)]
    #[inline]
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    #[allow(dead_code)]
    #[inline]
    fn push(&mut self, x: T) -> Option<T> {
        if self.heap.len() < self.limit {
            self.heap.push(x);
            None
        } else {
            if self.heap.data[0] < x {
                let mut x = x;
                std::mem::swap(&mut x, &mut self.heap.data[0]);
                if self.heap.len() >= 2 && self.heap.data[0] > self.heap.data[1] {
                    self.heap.data.swap(0, 1);
                }
                self.heap.down(0);
                Some(x)
            } else {
                Some(x)
            }
        }
    }
    #[allow(dead_code)]
    #[inline]
    fn pop(&mut self) -> Option<T> {
        self.heap.pop_max()
    }
    #[allow(dead_code)]
    #[inline]
    fn clear(&mut self) {
        self.heap.clear();
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
macro_rules! debug_with_message {
    ($msg:expr $(, $a:expr)* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!("| ", $msg, " |", $(" ", stringify!($a), "={:?} |"),*), $(&$a),*);
    };
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

#[macro_export]
macro_rules! timer {
    ($ x : expr ) => {{
        let start = Instant::now();
        let result = $x;
        let end = start.elapsed();
        eprintln!(
            "計測開始から{}.{:03}秒経過しました。",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
        result
    }};
}
