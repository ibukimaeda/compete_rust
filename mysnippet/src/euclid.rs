use cargo_snippet::snippet;

#[snippet(":extgcd")]
fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    // ax + by = gcd(a, b) を満たす x, y, gcd(a, b) を返す
    fn _extgcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
        let mut g = a;
        *x = 1;
        *y = 0;
        if b != 0 {
            g = _extgcd(b, a % b, y, x);
            *y -= (a / b) * *x;
        }
        g
    }

    let mut x = 0;
    let mut y = 0;
    let g = _extgcd(a, b, &mut x, &mut y);

    (x, y, g)
}

#[snippet(name = ":extgcd")]
fn positive_extgcd(a: i64, b: i64) -> Option<(i64, i64, i64)> {
    // ax + by = gcd(a, b) を満たす正の x, y が存在すれば x, y, gcd(x, y) を返す
    let (x, y, g) = extgcd(a, b);
    let (a_g, b_g) = (a / g, b / g);
    let r = x % b_g;
    let px = if r >= 0 { r } else { r + b_g };
    let py = y + (x - px) / b_g * a_g;

    if py >= 0 {
        Some((px, py, g))
    } else {
        None
    }
}
