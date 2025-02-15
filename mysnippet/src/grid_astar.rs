use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    cost: usize,
    position: Point,
    priority: usize, // f = g + h
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Reverse order for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: Point, b: Point) -> usize {
    (a.row as isize - b.row as isize).abs() as usize
        + (a.col as isize - b.col as isize).abs() as usize
}

fn astar(grid: &Vec<Vec<u8>>, start: Point, goal: Point) -> Option<Vec<Point>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_score: HashMap<Point, usize> = HashMap::new();

    g_score.insert(start, 0);
    open_set.push(Node {
        cost: 0,
        position: start,
        priority: heuristic(start, goal),
    });

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(Node { position, .. }) = open_set.pop() {
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

            if new_x >= grid.len() || new_y >= grid[0].len() || grid[new_x][new_y] == 1 {
                continue; // 壁を避ける
            }

            let new_pos = Point {
                row: new_x,
                col: new_y,
            };
            let tentative_g_score = current_cost + 1;

            if tentative_g_score < *g_score.get(&new_pos).unwrap_or(&usize::MAX) {
                came_from.insert(new_pos, position);
                g_score.insert(new_pos, tentative_g_score);
                let priority = tentative_g_score + heuristic(new_pos, goal);
                open_set.push(Node {
                    cost: tentative_g_score,
                    position: new_pos,
                    priority,
                });
            }
        }
    }

    None
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0],
    ];

    let start = Point { row: 0, col: 0 };
    let goal = Point { row: 4, col: 5 };

    if let Some(path) = astar(&grid, start, goal) {
        for p in path {
            println!("({}, {})", p.row, p.col);
        }
    } else {
        println!("Path not found.");
    }
}
