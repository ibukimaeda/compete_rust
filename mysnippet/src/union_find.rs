use cargo_snippet::snippet;
use std::{mem, vec};

#[snippet(":union_find")]
#[derive(Debug)]
struct UnionFind {
    // https://qiita.com/ofutonton/items/c17dfd33fc542c222396
    parent: Vec<i32>,
}

#[snippet(":union_find")]
impl UnionFind {
    #[allow(dead_code)]
    fn new(size: usize) -> Self {
        UnionFind {
            parent: vec![-1; size],
        }
    }

    #[allow(dead_code)]
    fn unite(&mut self, x: usize, y: usize) -> bool {
        // x と y を結合
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
        // k の属する木の根を探索
        assert!(k < self.parent.len());
        if self.parent[k as usize] < 0 {
            return k;
        }

        self.parent[k as usize] = self.root(self.parent[k] as usize) as i32;
        return self.parent[k] as usize;
    }

    #[allow(dead_code)]
    fn size(&mut self, k: usize) -> usize {
        // k の属する木の大きさを返す
        assert!(k < self.parent.len());
        let x = self.root(k);
        return -self.parent[x] as usize;
    }

    #[allow(dead_code)]
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        // x と y の属する木が同じかどうか
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
