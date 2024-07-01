use cargo_snippet::snippet;
use num::PrimInt;

// see https://tubo28.me/compprog/algorithm/comb/

#[snippet(":comb")]
struct CombUtil<Int>
where
    Int: PrimInt,
{
    fc: Vec<Int>,
    ifc: Vec<Int>,
    mod_value: Int,
}

#[snippet(":comb")]
impl<Int> CombUtil<Int>
where
    Int: PrimInt + Default,
{
    fn new(size: usize, mod_value: Int) -> Self {
        let mut fc = vec![Int::default(); size + 1];
        let mut ifc = vec![Int::default(); size + 1];

        fc[0] = Int::from(1).unwrap();
        for i in 1..=size {
            fc[i] = fc[i - 1] * Int::from(i).unwrap() % mod_value;
        }

        ifc[size] = Self::inv(fc[size], mod_value);
        for i in (0..size).rev() {
            ifc[i] = ifc[i + 1] * Int::from(i + 1).unwrap() % mod_value;
        }

        Self { fc, ifc, mod_value }
    }

    fn fact(&self, n: usize) -> Int {
        self.fc[n]
    }

    fn inv_fact(&self, n: usize) -> Int {
        self.ifc[n]
    }

    fn inv(n: Int, mod_value: Int) -> Int {
        Self::pow(n, mod_value - Int::from(2).unwrap(), mod_value)
    }

    fn pow(n: Int, mut a: Int, mod_value: Int) -> Int {
        let mut res = Int::from(1).unwrap();
        let mut exp = n % mod_value;
        while a > Int::from(0).unwrap() {
            if a % Int::from(2).unwrap() == Int::from(1).unwrap() {
                res = res * exp % mod_value;
            }
            exp = exp * exp % mod_value;
            a = a / Int::from(2).unwrap();
        }
        res
    }

    fn npr(&self, n: usize, r: usize) -> Int {
        if r > n {
            Int::from(0).unwrap()
        } else {
            self.fc[n] * self.ifc[n - r] % self.mod_value
        }
    }

    fn ncr(&self, n: i64, r: i64) -> Int {
        if n < 0 || r < 0 || n < r {
            Int::from(0).unwrap()
        } else {
            self.fc[n as usize] * self.ifc[r as usize] % self.mod_value * self.ifc[(n - r) as usize]
                % self.mod_value
        }
    }

    fn homo(&self, n: i64, r: i64) -> Int {
        if r == 0 {
            Int::from(1).unwrap()
        } else if n < 0 || r < 0 {
            Int::from(0).unwrap()
        } else {
            self.ncr(n + r - 1, r)
        }
    }
}

#[snippet(":comb")]
type Comb = CombUtil<i64>;

#[test]
fn main() {
    type Comb = CombUtil<i64>;
    let mod_value = 1_000_000_007;
    let comb = Comb::new(100, mod_value);

    assert_eq!(comb.fact(10), 3628800);
    assert_eq!(comb.fact(10) * comb.inv_fact(10) % mod_value, 1);
    assert_eq!(comb.npr(10, 1), 10);
    assert_eq!(comb.npr(10, 5), 30240);
    assert_eq!(comb.ncr(10, 5), 252);
    assert_eq!(comb.ncr(10, 7), 120);
}
