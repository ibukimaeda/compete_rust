use std::vec;

use cargo_snippet::snippet;

#[snippet(":lazy_segment_tree")]
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
    pub fn new(size: usize, op: F, element: E, mapping: G, id: H, composite: I) -> Self {
        let tree_size = size.next_power_of_two() * 2 - 1;
        let value = vec![element(); tree_size];
        let lazy = vec![id(); tree_size];
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

    pub fn prod(&mut self, left: usize, right: usize) -> S {
        // 区間 [left, right) の演算結果を返す
        assert!(left <= right);
        if right == left {
            (self.element)()
        } else {
            self._prod(0, left, right, 0, self.value.len() / 2 + 1)
        }
    }

    fn _prod(
        &mut self,
        tree_index: usize,
        search_left: usize,
        search_right: usize,
        left: usize,
        right: usize,
    ) -> S {
        if search_left <= left && right <= search_right {
            self.value[tree_index]
        } else if right <= search_left || search_right <= left {
            (self.element)()
        } else {
            if self.lazy[tree_index] != (self.id)() {
                self.propagate(tree_index, left, right, left, right);
            }
            let mid = (left + right) / 2;
            let (left_t_index, right_t_index) = self.get_children(tree_index);
            let l_value = self._prod(left_t_index, search_left, search_right, left, mid);
            let r_value = self._prod(right_t_index, search_left, search_right, mid, right);
            (self.op)(l_value, r_value)
        }
    }

    fn get_children(&self, tree_index: usize) -> (usize, usize) {
        (tree_index * 2 + 1, tree_index * 2 + 2)
    }

    pub fn apply(&mut self, v: T, left: usize, right: usize) {
        // 区間 [left, right) に v を適用
        self._apply(v, 0, left, right, 0, self.value.len() / 2 + 1);
    }

    fn _apply(
        &mut self,
        v: T,
        tree_index: usize,
        search_left: usize,
        search_right: usize,
        left: usize,
        right: usize,
    ) {
        if right <= search_left || search_right <= left {
            return;
        }
        self.lazy[tree_index] = (self.composite)(v, self.lazy[tree_index]);
        if search_left <= left && right <= search_right {
            self.value[tree_index] = (self.mapping)(v, self.value[tree_index]);
        } else {
            self.propagate(tree_index, search_left, search_right, left, right);
            let (left_t_index, right_t_index) = self.get_children(tree_index);
            self.value[tree_index] = (self.op)(self.value[left_t_index], self.value[right_t_index]);
        }
    }

    fn propagate(
        &mut self,
        tree_index: usize,
        search_left: usize,
        search_right: usize,
        left: usize,
        right: usize,
    ) {
        let lazy = self.lazy[tree_index];
        self.lazy[tree_index] = (self.id)();
        let mid = (left + right) / 2;
        let (left_t_index, right_t_index) = self.get_children(tree_index);
        self._apply(lazy, left_t_index, search_left, search_right, left, mid);
        self._apply(lazy, right_t_index, search_left, search_right, mid, right);
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
    pub fn range_add_min(size: usize) -> Self {
        // 区間加算，区間最小
        let op = |x: i64, y: i64| std::cmp::min(x, y);
        let element = || 1_010_000_000_000_000_017;
        let id = || 0i64;
        let mapping = |f: i64, x: i64| f + x;
        let composite = |f: i64, g: i64| f + g;

        LazySegmentTree::new(size, op, element, mapping, id, composite)
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
    let mut segtree = LazySegmentTree::new(size, op, element, mapping, id, composite);
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

    let mut segtree = RangeAddMinSegTree::range_add_min(size);

    segtree.apply(1, 0, 5);
    // segtree.apply(2, 3, 7);
    // segtree.apply(3, 5, 10);
    // assert_eq!(segtree.prod(0, 10), 1);
    // assert_eq!(segtree.prod(0, 5), 1);
    // assert_eq!(segtree.prod(5, 10), 3);
    // assert_eq!(segtree.prod(3, 7), 3);
    // assert_eq!(segtree.prod(0, 3), 1);
    // assert_eq!(segtree.prod(7, 10), 3);
}
