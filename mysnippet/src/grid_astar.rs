use cargo_snippet::snippet;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[snippet(":grid_astar")]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

#[snippet(":grid_astar")]
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
struct AstarNode {
    cost: usize,
    position: Point,
    priority: usize,
}

#[snippet(":grid_astar")]
#[allow(dead_code)]
impl Ord for AstarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

#[snippet(":grid_astar")]
#[allow(dead_code)]
impl PartialOrd for AstarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[snippet(":grid_astar")]
#[allow(dead_code)]
struct AStar {
    grid: Vec<Vec<u8>>,
}

#[snippet(":grid_astar")]
#[allow(dead_code)]
impl AStar {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        AStar { grid }
    }

    fn heuristic(a: Point, b: Point) -> usize {
        (a.row as isize - b.row as isize).abs() as usize
            + (a.col as isize - b.col as isize).abs() as usize
    }

    fn find_path(&self, start: Point, goal: Point) -> Option<Vec<Point>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from: FxHashMap<Point, Point> = FxHashMap::default();
        let mut g_score: FxHashMap<Point, usize> = FxHashMap::default();

        g_score.insert(start, 0);
        open_set.push(AstarNode {
            cost: 0,
            position: start,
            priority: Self::heuristic(start, goal),
        });

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some(AstarNode { position, .. }) = open_set.pop() {
            if position == goal {
                let mut path = vec![position];
                while let Some(&prev) = came_from.get(&path[0]) {
                    path.insert(0, prev);
                }
                return Some(path);
            }

            let current_cost = *g_score.get(&position).unwrap_or(&usize::MAX);

            for (dx, dy) in directions.iter() {
                let new_x = position.row as isize + dx;
                let new_y = position.col as isize + dy;

                if new_x < 0 || new_y < 0 {
                    continue;
                }
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if new_x >= self.grid.len()
                    || new_y >= self.grid[0].len()
                    || self.grid[new_x][new_y] == 1
                {
                    continue;
                }

                let new_pos = Point {
                    row: new_x,
                    col: new_y,
                };
                let tentative_g_score = current_cost + 1;

                if tentative_g_score < *g_score.get(&new_pos).unwrap_or(&usize::MAX) {
                    came_from.insert(new_pos, position);
                    g_score.insert(new_pos, tentative_g_score);
                    let priority = tentative_g_score + Self::heuristic(new_pos, goal);
                    open_set.push(AstarNode {
                        cost: tentative_g_score,
                        position: new_pos,
                        priority,
                    });
                }
            }
        }

        None
    }
}

#[test]
fn test_astar() {
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let astar = AStar::new(grid);
    let start = Point { row: 0, col: 0 };
    let goal = Point { row: 4, col: 4 };

    let path = astar.find_path(start, goal).unwrap();
    assert_eq!(path.len(), 9);
    assert_eq!(path[0], start);
    assert_eq!(path[path.len() - 1], goal);
}
