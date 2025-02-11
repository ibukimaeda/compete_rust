use cargo_snippet::snippet;

#[allow(dead_code)]
#[snippet(":updated_coordinate")]
#[allow(non_snake_case)]
fn is_in(now: (usize, usize), dx: i64, dy: i64, H: usize, W: usize) -> bool {
    let H = H as i64;
    let W = W as i64;
    let new_x = now.0 as i64 + dx;
    let new_y = now.1 as i64 + dy;

    return 0 <= new_x && new_x < H && 0 <= new_y && new_y < W;
}

#[test]
fn test_is_in() {
    assert_eq!(is_in((0, 0), 0, 0, 3, 3), true);
    assert_eq!(is_in((0, 0), 0, 1, 3, 3), true);
    assert_eq!(is_in((0, 0), 1, 0, 3, 3), true);
    assert_eq!(is_in((0, 0), 1, 1, 3, 3), true);
    assert_eq!(is_in((0, 0), -1, 0, 3, 3), false);
    assert_eq!(is_in((0, 0), 0, -1, 3, 3), false);
    assert_eq!(is_in((0, 0), -1, -1, 3, 3), false);
    assert_eq!(is_in((0, 0), 1, -1, 3, 3), false);
    assert_eq!(is_in((0, 0), -1, 1, 3, 3), false);
    assert_eq!(is_in((0, 0), 0, 3, 3, 3), false);
    assert_eq!(is_in((0, 0), 3, 0, 3, 3), false);
}

#[allow(dead_code)]
#[snippet(":updated_coordinate")]
#[allow(non_snake_case)]
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

#[test]
fn test_updated_coordinate() {
    assert_eq!(updated_coordinate(0, 0, 0, 0, 3, 3), Some((0, 0)));
    assert_eq!(updated_coordinate(0, 0, 0, 1, 3, 3), Some((0, 1)));
    assert_eq!(updated_coordinate(0, 0, 1, 0, 3, 3), Some((1, 0)));
    assert_eq!(updated_coordinate(0, 0, 1, 1, 3, 3), Some((1, 1)));
    assert_eq!(updated_coordinate(0, 0, -1, 0, 3, 3), None);
    assert_eq!(updated_coordinate(0, 0, 0, -1, 3, 3), None);
    assert_eq!(updated_coordinate(0, 0, -1, -1, 3, 3), None);
    assert_eq!(updated_coordinate(0, 0, 1, -1, 3, 3), None);
}

#[allow(dead_code)]
#[snippet(":rotated")]
fn rotated<T: Default + Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    // 2次配列を右回りに90度回転したものを返す
    let mut ret = vec![vec![Default::default(); grid.len()]; grid[0].len()];
    for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            ret[i][j] = grid[grid.len() - 1 - j][i].clone();
        }
    }
    return ret;
}

#[test]
fn test_rotated() {
    let grid = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let rotated_grid = rotated(&grid);
    assert_eq!(rotated_grid, vec![vec![5, 3, 1], vec![6, 4, 2]]);
}

#[allow(dead_code)]
#[snippet(":shifted")]
fn shifted<T: Default + Clone>(grid: &Vec<Vec<T>>, dx: i64, dy: i64, default: T) -> Vec<Vec<T>> {
    // 2次元配列を下に dx 右に dy 動かしたものを返す
    let mut ret = vec![vec![Default::default(); grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut value = default.clone();

            let nx = i as i64 - dx;
            let ny = j as i64 - dy;

            if 0 <= nx && nx < grid.len() as i64 && 0 <= ny && ny < grid[0].len() as i64 {
                value = grid[nx as usize][ny as usize].clone();
            }

            ret[i][j] = value;
        }
    }
    return ret;
}

#[test]
fn test_shifted() {
    let grid = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let shifted_grid = shifted(&grid, 1, 1, 0);
    assert_eq!(shifted_grid, vec![vec![0, 0], vec![0, 1], vec![0, 3]]);
}
