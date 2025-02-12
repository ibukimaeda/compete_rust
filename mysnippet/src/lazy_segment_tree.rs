use std::vec;

use cargo_snippet::snippet;
use num::iter::Range;

pub struct LazySegmentTree<S, F, E, T, G, H, I>
where
    S: Copy + Eq,
    T: Copy + Eq,
    F: Fn(S, S) -> S,
    E: Fn() -> S,
    G: Fn(T, S) -> S,
    H: Fn() -> T,
    I: Fn(T, T) -> T,
{
    value: Vec<S>,
    op: F,
    element: E,
    lazy: Vec<T>,
    mapping: G,
    id: H,
    composite: I,
}

#[snippet(":lazy_segment_tree")]
impl<S, F, E, T, G, H, I> LazySegmentTree<S, F, E, T, G, H, I>
where
    S: Copy + Eq,
    T: Copy + Eq,
    F: Fn(S, S) -> S,
    E: Fn() -> S,
    G: Fn(T, S) -> S,
    H: Fn() -> T,
    I: Fn(T, T) -> T,
{
    pub fn new(init_value: Vec<S>, op: F, element: E, mapping: G, id: H, composite: I) -> Self {
        let n = init_value.len().next_power_of_two();
        let tree_size = 2 * n - 1;
        let mut value = vec![element(); tree_size];
        let lazy = vec![id(); tree_size];

        for i in 0..init_value.len() {
            value[i + n - 1] = init_value[i];
        }
        for i in (0..n - 1).rev() {
            value[i] = (op)(value[i * 2 + 1], value[i * 2 + 2]);
        }

        Self {
            value,
            op,
            element,
            lazy,
            mapping,
            id,
            composite,
        }
    }

    pub fn apply(&mut self, v: T, left: usize, right: usize) {
        // 区間 [left, right) に v を適用
        self._apply(v, left, right, 0, 0, self.value.len() / 2 + 1);
    }

    pub fn prod(&mut self, left: usize, right: usize) -> S {
        // 区間 [left, right) の演算結果を返す
        assert!(left <= right);
        if right == left {
            (self.element)()
        } else {
            self._prod(left, right, 0, 0, self.value.len() / 2 + 1)
        }
    }

    pub fn eval_all(&mut self) {
        self._eval_all(0, 0, self.value.len() / 2 + 1);
    }

    pub fn get(&mut self, index: usize) -> S {
        self.prod(index, index + 1)
    }

    fn _eval(&mut self, tree_index: usize, left: usize, right: usize) {
        // tree_index 番目のノードに対して遅延評価を行う
        if self.lazy[tree_index] == (self.id)() {
            return;
        }

        self.value[tree_index] = (self.mapping)(self.lazy[tree_index], self.value[tree_index]);

        if right - left > 1 {
            let (left_t_index, right_t_index) = self._get_children(tree_index);
            self.lazy[left_t_index] =
                (self.composite)(self.lazy[tree_index], self.lazy[left_t_index]);
            self.lazy[right_t_index] =
                (self.composite)(self.lazy[tree_index], self.lazy[right_t_index]);
        }

        self.lazy[tree_index] = (self.id)();
    }

    fn _eval_all(&mut self, k: usize, l: usize, r: usize) {
        self._eval(k, l, r);
        if r - l > 1 {
            let mid = (l + r) / 2;
            let (left_t_index, right_t_index) = self._get_children(k);
            self._eval_all(left_t_index, l, mid);
            self._eval_all(right_t_index, mid, r);
        }
    }

    fn _apply(
        &mut self,
        v: T,
        search_left: usize,
        search_right: usize,
        tree_index: usize,
        left: usize,
        right: usize,
    ) {
        // [search_left, search_right)： 適用する区間
        // [left, right)： tree_index が担当する区間

        assert!(search_left <= search_right);
        assert!(left <= right);

        self._eval(tree_index, left, right);

        if right <= search_left || search_right <= left {
            // 交差しない
            return;
        }

        if search_left <= left && right <= search_right {
            // 完全に含まれる
            self.lazy[tree_index] = (self.composite)(v, self.lazy[tree_index]);
            self._eval(tree_index, left, right);
        } else {
            // 一部だけ含まれる
            let mid = (left + right) / 2;
            let (left_t_index, right_t_index) = self._get_children(tree_index);
            self._apply(v, search_left, search_right, left_t_index, left, mid);
            self._apply(v, search_left, search_right, right_t_index, mid, right);
            self.value[tree_index] = (self.op)(self.value[left_t_index], self.value[right_t_index]);
        }
    }

    fn _get_children(&self, tree_index: usize) -> (usize, usize) {
        (tree_index * 2 + 1, tree_index * 2 + 2)
    }

    fn _prod(
        &mut self,
        search_left: usize,
        search_right: usize,
        tree_index: usize,
        left: usize,
        right: usize,
    ) -> S {
        // [search_left, search_right)： 検索区間
        // [left, right)： tree_index が担当する区間

        assert!(search_left <= search_right);
        assert!(left <= right);

        if search_right <= left || right <= search_left {
            // 交差しない
            return (self.element)();
        }

        self._eval(tree_index, left, right);
        if search_left <= left && right <= search_right {
            // 完全に含まれる
            self.value[tree_index]
        } else {
            // 一部だけ含まれる
            let mid = (left + right) / 2;
            let (left_t_index, right_t_index) = self._get_children(tree_index);
            let l_value = self._prod(search_left, search_right, left_t_index, left, mid);
            let r_value = self._prod(search_left, search_right, right_t_index, mid, right);
            (self.op)(l_value, r_value)
        }
    }
}

#[snippet(":lazy_segment_tree")]
type RangeAddMinSegTree = LazySegmentTree<
    i64,
    fn(i64, i64) -> i64,
    fn() -> i64,
    i64,
    fn(i64, i64) -> i64,
    fn() -> i64,
    fn(i64, i64) -> i64,
>;

#[snippet(":lazy_segment_tree")]
impl RangeAddMinSegTree {
    pub fn range_add_min(init_value: Vec<i64>) -> Self {
        // RAQ-RmQ
        // 区間加算，区間最小
        // https://qiita.com/okateim/items/e2f4a734db4e5f90e410
        // 区間最小の方は小文字の m を使用して Max の使い分ける
        let op = |x: i64, y: i64| std::cmp::min(x, y);
        let element = || 1_010_000_000_000_000_017;
        let mapping = |f: i64, x: i64| f + x;
        let id = || 0i64;
        let composite = |f: i64, g: i64| f + g;

        LazySegmentTree::new(init_value, op, element, mapping, id, composite)
    }
}

#[snippet(":lazy_segment_tree")]
type RangeAddMaxSegTree = LazySegmentTree<
    i64,
    fn(i64, i64) -> i64,
    fn() -> i64,
    i64,
    fn(i64, i64) -> i64,
    fn() -> i64,
    fn(i64, i64) -> i64,
>;

#[snippet(":lazy_segment_tree")]
impl RangeAddMaxSegTree {
    pub fn range_add_max(init_value: Vec<i64>) -> Self {
        // RAQ-RMQ
        // 区間加算，区間最大
        let op = |x: i64, y: i64| std::cmp::max(x, y);
        let element = || -1_010_000_000_000_000_017;
        let mapping = |f: i64, x: i64| f + x;
        let id = || 0i64;
        let composite = |f: i64, g: i64| f + g;

        LazySegmentTree::new(init_value, op, element, mapping, id, composite)
    }
}

#[snippet(":lazy_segment_tree")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RangeSum {
    sum: i64,
    count: i64,
}

#[snippet(":lazy_segment_tree")]
type RangeAddSumSegTree = LazySegmentTree<
    RangeSum,
    fn(RangeSum, RangeSum) -> RangeSum,
    fn() -> RangeSum,
    i64,
    fn(i64, RangeSum) -> RangeSum,
    fn() -> i64,
    fn(i64, i64) -> i64,
>;

#[snippet(":lazy_segment_tree")]
impl RangeAddSumSegTree {
    pub fn range_add_sum(init_value: Vec<i64>) -> Self {
        // RAQ-RSQ
        // 区間加算，区間和
        let op = |a: RangeSum, b: RangeSum| RangeSum {
            sum: a.sum + b.sum,
            count: a.count + b.count,
        };
        let element = || RangeSum { sum: 0, count: 0 };
        let mapping = |f: i64, x: RangeSum| RangeSum {
            sum: f * x.count + x.sum,
            count: x.count,
        };
        let id = || 0i64;
        let composite = |f: i64, g: i64| f + g;

        let init_value = init_value
            .iter()
            .map(|&x| RangeSum { sum: x, count: 1 })
            .collect();

        LazySegmentTree::new(init_value, op, element, mapping, id, composite)
    }
}

#[snippet(":lazy_segment_tree")]
type RangeUpdateMinSegTree = LazySegmentTree<
    i64,
    fn(i64, i64) -> i64,
    fn() -> i64,
    Option<i64>,
    fn(Option<i64>, i64) -> i64,
    fn() -> Option<i64>,
    fn(Option<i64>, Option<i64>) -> Option<i64>,
>;

#[snippet(":lazy_segment_tree")]
impl RangeUpdateMinSegTree {
    pub fn range_update_min(init_value: Vec<i64>) -> Self {
        // RUQ-RmQ
        // 区間更新，区間最小
        let op = |x: i64, y: i64| std::cmp::min(x, y);
        let element = || 1_010_000_000_000_000_017;
        let mapping = |f: Option<i64>, x: i64| match f {
            Some(val) => val,
            None => x,
        };
        let id = || None::<i64>;
        let composite = |f: Option<i64>, g: Option<i64>| match f {
            Some(_) => f,
            None => g,
        };

        LazySegmentTree::new(init_value, op, element, mapping, id, composite)
    }
}

#[snippet(":lazy_segment_tree")]
type RangeUpdateMaxSegTree = LazySegmentTree<
    i64,
    fn(i64, i64) -> i64,
    fn() -> i64,
    Option<i64>,
    fn(Option<i64>, i64) -> i64,
    fn() -> Option<i64>,
    fn(Option<i64>, Option<i64>) -> Option<i64>,
>;

#[snippet(":lazy_segment_tree")]
impl RangeUpdateMaxSegTree {
    pub fn range_update_max(init_value: Vec<i64>) -> Self {
        // RUQ-RMQ
        // 区間更新，区間最大
        let op = |x: i64, y: i64| std::cmp::max(x, y);
        let element = || -1_010_000_000_000_000_017;
        let mapping = |f: Option<i64>, x: i64| match f {
            Some(val) => val,
            None => x,
        };
        let id = || None::<i64>;
        let composite = |f: Option<i64>, g: Option<i64>| match f {
            Some(_) => f,
            None => g,
        };

        LazySegmentTree::new(init_value, op, element, mapping, id, composite)
    }
}

#[snippet(":lazy_segment_tree")]
type RangeUpdateSumSegTree = LazySegmentTree<
    RangeSum,
    fn(RangeSum, RangeSum) -> RangeSum,
    fn() -> RangeSum,
    Option<i64>,
    fn(Option<i64>, RangeSum) -> RangeSum,
    fn() -> Option<i64>,
    fn(Option<i64>, Option<i64>) -> Option<i64>,
>;

#[snippet(":lazy_segment_tree")]
impl RangeUpdateSumSegTree {
    pub fn range_update_sum(init_value: Vec<i64>) -> Self {
        // RUQ-RMQ
        // 区間更新，区間和
        let op = |x: RangeSum, y: RangeSum| RangeSum {
            sum: x.sum + y.sum,
            count: x.count + y.count,
        };
        let element = || RangeSum { sum: 0, count: 0 };
        let mapping = |f: Option<i64>, x: RangeSum| RangeSum {
            sum: match f {
                Some(val) => val * x.count,
                None => x.sum,
            },
            count: x.count,
        };
        let id = || None::<i64>;
        let composite = |f: Option<i64>, g: Option<i64>| f.or(g);

        let init_value = init_value
            .iter()
            .map(|&x| RangeSum { sum: x, count: 1 })
            .collect();

        LazySegmentTree::new(init_value, op, element, mapping, id, composite)
    }
}

#[test]
fn test_lazy_segtree() {
    let lr = vec![(27, 100), (8, 39), (83, 97), (24, 75)];
    let size = 100;
    let op = |x: usize, y: usize| std::cmp::max(x, y);
    let element = || 0usize;
    let mapping = |l: Option<usize>, v: usize| match l {
        Some(val) => val,
        None => v,
    };
    let id = || None::<usize>;
    let composite = |f: Option<usize>, g: Option<usize>| match f {
        Some(_) => f,
        None => g,
    };

    let init_value = vec![0; size];
    let mut segtree = LazySegmentTree::new(init_value, op, element, mapping, id, composite);
    let mut ans = Vec::new();
    for i in 0..lr.len() {
        let (l, r) = lr[i];
        let (l, r) = (l - 1, r - 1);
        let next_height = segtree.prod(l, r + 1) + 1;
        ans.push(next_height);
        segtree.apply(Some(next_height), l, r + 1);
    }

    assert_eq!(ans, vec![1, 2, 2, 3]);
}

#[test]
fn test_lazy_segtree_range_add_min() {
    let size = 10;

    let init_value = vec![0; size];
    let mut segtree = RangeAddMinSegTree::range_add_min(init_value);

    segtree.apply(1, 0, 2);
    segtree.apply(2, 3, 7);
    segtree.apply(3, 5, 10);
    assert_eq!(segtree.prod(0, 10), 0);
    assert_eq!(segtree.prod(0, 5), 0);
    assert_eq!(segtree.prod(5, 10), 3);
    assert_eq!(segtree.prod(3, 7), 2);
    assert_eq!(segtree.prod(0, 3), 0);
    assert_eq!(segtree.prod(7, 10), 3);

    assert_eq!(segtree.get(2), 0);
    assert_eq!(segtree.get(5), 5);
    assert_eq!(segtree.get(7), 3);
}

#[test]
fn test_lazy_segtree_range_add_max() {
    let size = 10;

    let init_value = vec![0; size];
    let mut segtree = RangeAddMaxSegTree::range_add_max(init_value);

    segtree.apply(1, 0, 2);
    segtree.apply(2, 3, 7);
    segtree.apply(3, 5, 10);
    assert_eq!(segtree.prod(0, 10), 5);
    assert_eq!(segtree.prod(0, 5), 2);
    assert_eq!(segtree.prod(5, 10), 5);
    assert_eq!(segtree.prod(3, 7), 5);
    assert_eq!(segtree.prod(0, 3), 1);
    assert_eq!(segtree.prod(7, 10), 3);

    assert_eq!(segtree.get(2), 0);
    assert_eq!(segtree.get(5), 5);
    assert_eq!(segtree.get(7), 3);
}

#[test]
fn test_lazy_segtree_range_add_sum() {
    let size = 10;

    let init_value = vec![0; size];
    let mut segtree = RangeAddSumSegTree::range_add_sum(init_value);

    segtree.apply(1, 0, 2);
    segtree.apply(2, 3, 7);
    segtree.apply(3, 5, 10);
    assert_eq!(segtree.prod(0, 10).sum, 25);
    assert_eq!(segtree.prod(0, 5).sum, 6);
    assert_eq!(segtree.prod(5, 10).sum, 19);
    assert_eq!(segtree.prod(3, 7).sum, 14);
    assert_eq!(segtree.prod(0, 3).sum, 2);
    assert_eq!(segtree.prod(7, 10).sum, 9);

    assert_eq!(segtree.get(2), RangeSum { sum: 0, count: 1 });
    assert_eq!(segtree.get(5), RangeSum { sum: 5, count: 1 });
    assert_eq!(segtree.get(7), RangeSum { sum: 3, count: 1 });
}

#[test]
fn test_lazy_segtree_range_update_min() {
    let size = 10;

    let init_value = vec![0; size];
    let mut segtree = RangeUpdateMinSegTree::range_update_min(init_value);

    segtree.apply(Some(1), 0, 2);
    segtree.apply(Some(2), 3, 7);
    segtree.apply(Some(3), 5, 10);
    assert_eq!(segtree.prod(0, 10), 0);
    assert_eq!(segtree.prod(0, 5), 0);
    assert_eq!(segtree.prod(5, 10), 3);
    assert_eq!(segtree.prod(3, 7), 2);
    assert_eq!(segtree.prod(0, 3), 0);
    assert_eq!(segtree.prod(7, 10), 3);

    assert_eq!(segtree.get(2), 0);
    assert_eq!(segtree.get(5), 3);
    assert_eq!(segtree.get(7), 3);
}

#[test]
fn test_lazy_segtree_range_update_max() {
    let size = 10;

    let init_value = vec![0; size];
    let mut segtree = RangeUpdateMaxSegTree::range_update_max(init_value);

    segtree.apply(Some(1), 0, 2);
    segtree.apply(Some(2), 3, 7);
    segtree.apply(Some(3), 5, 10);
    assert_eq!(segtree.prod(0, 10), 3);
    assert_eq!(segtree.prod(0, 5), 2);
    assert_eq!(segtree.prod(5, 10), 3);
    assert_eq!(segtree.prod(3, 7), 3);
    assert_eq!(segtree.prod(0, 3), 1);
    assert_eq!(segtree.prod(7, 10), 3);

    assert_eq!(segtree.get(2), 0);
    assert_eq!(segtree.get(5), 3);
    assert_eq!(segtree.get(7), 3);
}

#[test]
fn test_lazy_segtree_range_update_sum() {
    let size = 10;

    let init_value = vec![0; size];
    let mut segtree = RangeUpdateSumSegTree::range_update_sum(init_value);

    segtree.apply(Some(1), 0, 2);
    segtree.apply(Some(2), 3, 7);
    segtree.apply(Some(3), 5, 10);
    assert_eq!(segtree.prod(0, 10).sum, 21);
    assert_eq!(segtree.prod(0, 5).sum, 6);
    assert_eq!(segtree.prod(5, 10).sum, 15);
    assert_eq!(segtree.prod(3, 7).sum, 10);
    assert_eq!(segtree.prod(0, 3).sum, 2);
    assert_eq!(segtree.prod(7, 10).sum, 9);

    assert_eq!(segtree.get(2), RangeSum { sum: 0, count: 1 });
    assert_eq!(segtree.get(5), RangeSum { sum: 3, count: 1 });
    assert_eq!(segtree.get(7), RangeSum { sum: 3, count: 1 });
}
