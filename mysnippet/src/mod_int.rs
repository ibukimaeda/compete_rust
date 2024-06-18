use cargo_snippet::snippet;
use std::cmp;
use std::fmt;
use std::mem;
use std::ops;

#[snippet(":mod_int")]
#[derive(Clone, Copy, Debug)]
struct ModInt {
    x: i64,
    modulo: i64,
}

#[snippet(":mod_int")]
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

#[snippet(":mod_int")]
impl fmt::Display for ModInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

#[snippet(":mod_int")]
impl ops::Neg for ModInt {
    type Output = ModInt;
    fn neg(self) -> Self::Output {
        ModInt::new(-self.x, self.modulo)
    }
}

#[snippet(":mod_int")]
impl ops::Add<ModInt> for ModInt {
    type Output = ModInt;
    fn add(self, rhs: Self) -> Self::Output {
        return ModInt::new(self.x + rhs.x, self.modulo);
    }
}

#[snippet(":mod_int")]
impl ops::Add<i64> for ModInt {
    type Output = ModInt;
    fn add(self, rhs: i64) -> Self::Output {
        return ModInt::new(self.x + rhs, self.modulo);
    }
}

#[snippet(":mod_int")]
impl<'a> ops::AddAssign<&'a Self> for ModInt {
    fn add_assign(&mut self, rhs: &Self) {
        self.set(self.x + rhs.x);
    }
}

#[snippet(":mod_int")]
impl ops::AddAssign<i64> for ModInt {
    fn add_assign(&mut self, rhs: i64) {
        self.set(self.x + rhs);
    }
}

#[snippet(":mod_int")]
impl ops::Sub<ModInt> for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: Self) -> Self::Output {
        return ModInt::new(self.x - rhs.x, self.modulo);
    }
}

#[snippet(":mod_int")]
impl ops::Sub<i64> for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: i64) -> Self::Output {
        return ModInt::new(self.x - rhs, self.modulo);
    }
}

#[snippet(":mod_int")]
impl<'a> ops::SubAssign<&'a Self> for ModInt {
    fn sub_assign(&mut self, rhs: &Self) {
        self.set(self.x - rhs.x);
    }
}

#[snippet(":mod_int")]
impl ops::SubAssign<i64> for ModInt {
    fn sub_assign(&mut self, rhs: i64) {
        self.set(self.x - rhs);
    }
}

#[snippet(":mod_int")]
impl ops::Mul<ModInt> for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: Self) -> Self::Output {
        ModInt::new(self.x * rhs.x, self.modulo)
    }
}

#[snippet(":mod_int")]
impl ops::Mul<i64> for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: i64) -> Self::Output {
        ModInt::new(self.x * rhs, self.modulo)
    }
}

#[snippet(":mod_int")]
impl<'a> ops::MulAssign<&'a Self> for ModInt {
    fn mul_assign(&mut self, rhs: &Self) {
        self.set(self.x * rhs.x);
    }
}

#[snippet(":mod_int")]
impl ops::MulAssign<i64> for ModInt {
    fn mul_assign(&mut self, rhs: i64) {
        self.set(self.x * rhs);
    }
}

#[snippet(":mod_int")]
impl ops::Div<ModInt> for ModInt {
    type Output = ModInt;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

#[snippet(":mod_int")]
impl ops::Div<i64> for ModInt {
    type Output = ModInt;
    fn div(self, rhs: i64) -> Self::Output {
        self * ModInt::new(rhs, self.modulo).inv()
    }
}

#[snippet(":mod_int")]
impl<'a> ops::DivAssign<&'a Self> for ModInt {
    fn div_assign(&mut self, rhs: &Self) {
        self.set(self.x * rhs.inv().x);
    }
}

#[snippet(":mod_int")]
impl ops::DivAssign<i64> for ModInt {
    fn div_assign(&mut self, rhs: i64) {
        self.set(self.x * ModInt::new(rhs, self.modulo).inv().x);
    }
}

#[snippet(":mod_int")]
impl cmp::PartialEq<ModInt> for ModInt {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x
    }
}

#[snippet(":mod_int")]
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

#[snippet(":mod_int")]
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

#[snippet(":mod_int")]
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

#[cfg(test)]
mod tests_mod_int {
    use super::ModInt;

    #[test]
    fn test_1() {
        let modulo = 1000000007;
        let mut a = ModInt::new(1, modulo);
        let b = ModInt::new(2, modulo);
        a += &b;
        assert_eq!(a.x, 3);
        a -= &b;
        assert_eq!(a.x, 1);
        a *= &b;
        assert_eq!(a.x, 2);
        a /= &b;
        assert_eq!(a.x, 1);
        a += 2;
        assert_eq!(a.x, 3);
        a -= 2;
        assert_eq!(a.x, 1);
        a *= 2;
        assert_eq!(a.x, 2);
        a /= 2;
        assert_eq!(a.x, 1);
        a += &b;
        assert_eq!(a.x, 3);
        a -= &b;
        assert_eq!(a.x, 1);
        a *= &b;
        assert_eq!(a.x, 2);
        a /= &b;
        assert_eq!(a.x, 1);
        a += 2;
        assert_eq!(a.x, 3);
        a -= 2;
        assert_eq!(a.x, 1);
        a *= 2;
        assert_eq!(a.x, 2);
        a /= 2;
        assert_eq!(a.x, 1);
        assert_eq!(a.inv().x, 1);

        a.set(2);
        assert_eq!(a.x, 2);
        assert_eq!(a.inv().x, 500000004);
        assert_eq!((a * a.inv()).x, 1);
    }

    #[test]
    fn test_2() {
        let modulo = 1000000007;
        let a = ModInt::new(111, modulo);
        let b = ModInt::new(222, modulo);
        let c = ModInt::new(333, modulo);
        let d = ModInt::new(444, modulo);

        assert_eq!((a * b + c - d).x, 24531);
    }

    #[test]
    fn test_3() {
        let modulo = 1000000007;
        let a = ModInt::new(111111111, modulo);
        let b = ModInt::new(222222222, modulo);
        let c = ModInt::new(333333333, modulo);
        let d = ModInt::new(444444444, modulo);

        assert_eq!((a * b + c - d).x, 691358032);
    }

    #[test]
    fn test_4() {
        let modulo = 1000000007;

        let a = ModInt::new(3, modulo);
        assert_eq!(a.pow(0).x, 1);
        assert_eq!(a.pow(1).x, 3);
        assert_eq!(a.pow(2).x, 9);
        assert_eq!(a.pow(3).x, 27);
        assert_eq!(a.pow(4).x, 81);
        assert_eq!(a.pow(5).x, 243);
        assert_eq!(a.pow(modulo - 1).x, 1);
        assert_eq!(a.pow(modulo - 2), a.inv());
    }
}
