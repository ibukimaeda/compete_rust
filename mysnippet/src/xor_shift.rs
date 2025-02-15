use cargo_snippet::snippet;

#[snippet(":Xorshift")]
#[derive(Debug)]
#[allow(dead_code)]
pub struct Xorshift {
    // https://qiita.com/hatoo@github/items/652b81e8e83b0680bc0a#xorshift
    seed: u64,
}

#[snippet(":Xorshift")]
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
        self.rand(r - l) + l
    }

    #[inline]
    #[allow(dead_code)]
    pub fn randf(&mut self) -> f64 {
        // 0.0 ~ 1.0
        use std::mem;
        const UPPER_MASK: u64 = 0x3FF0000000000000;
        const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
        let tmp = UPPER_MASK | (self.next() & LOWER_MASK);
        let result: f64 = unsafe { mem::transmute(tmp) };
        result - 1.0
    }
}
