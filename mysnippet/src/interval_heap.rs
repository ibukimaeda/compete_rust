use cargo_snippet::snippet;

#[snippet(":interval_heap")]
#[derive(Clone, Debug)]
struct IntervalHeap<T: Ord + Eq> {
    data: Vec<T>,
}

#[snippet(":interval_heap")]
impl<T: Ord + Eq> IntervalHeap<T> {
    // https://qiita.com/hatoo@github/items/652b81e8e83b0680bc0a#interval-heap
    // 最大・最小の要素を高速に（計算量は計算してない）取り出すことができる
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
#[snippet(":interval_heap")]
struct LimitedIntervalHeap<T: Ord + Eq> {
    heap: IntervalHeap<T>,
    limit: usize,
}

#[snippet(":interval_heap")]
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
