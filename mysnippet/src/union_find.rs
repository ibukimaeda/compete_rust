use cargo_snippet::snippet;
use std::mem;

#[snippet(":union_find")]
#[derive(Debug)]
struct UnionFind {
    // https://qiita.com/ofutonton/items/c17dfd33fc542c222396
    data: Vec<i64>,
}

#[snippet(":union_find")]
impl UnionFind {
    #[allow(dead_code)]
    fn new(size: usize) -> Self {
        UnionFind {
            data: vec![-1; size],
        }
    }

    #[allow(dead_code)]
    fn unite(&mut self, mut x: i64, mut y: i64) -> bool {
        // x と y を結合
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

#[test]
fn test_union_find() {
    let mut uf = UnionFind::new(5);
    assert_eq!(uf.unite(0, 1), true);
    assert_eq!(uf.unite(1, 2), true);
    assert_eq!(uf.unite(3, 4), true);
    assert_eq!(uf.unite(0, 2), false);
    assert_eq!(uf.size(0), 3);
    assert_eq!(uf.size(2), 3);
    assert_eq!(uf.size(3), 2);
    assert_eq!(uf.is_same(0, 1), true);
    assert_eq!(uf.is_same(0, 3), false);
    assert_eq!(uf.groups(), vec![vec![0, 1, 2], vec![3, 4]]);
}
