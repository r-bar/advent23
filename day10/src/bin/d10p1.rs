use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

const MAX_STEPS: usize = 1000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

enum Direction {
    N,
    S,
    E,
    W,
}

struct Grid {
    start: Point,
    grid: Vec<Vec<char>>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut start = Point { x: 0, y: 0 };
        for (lineno, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (colno, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Point { x: colno, y: lineno };
                }
                row.push(c);
            }
            grid.push(row);
        }
        Ok(Grid { start, grid })
    }
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.grid.get(y).and_then(|row| row.get(x)).copied()
    }

    fn rel_direction(&self, origin: Point, adjacent: Point) -> Option<Direction> {
        let dx = adjacent.x as isize - origin.x as isize;
        let dy = adjacent.y as isize - origin.y as isize;
        match (dx, dy) {
            (0, 0) => None,
            (0, 1) => Some(Direction::S),
            (0, -1) => Some(Direction::N),
            (1, 0) => Some(Direction::E),
            (-1, 0) => Some(Direction::W),
            _ => None,
        }
    }

    fn start_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let n_coord = self.start.y.checked_sub(1).map(|y| Point { x: self.start.x, y });
        let n_chr = n_coord.and_then(|p| self.get(p.x, p.y));
        let s_coord = Point { x: self.start.x, y: self.start.y + 1 };
        let s_chr = self.get(s_coord.x, s_coord.y);
        let e_coord = Point { x: self.start.x + 1, y: self.start.y };
        let e_chr = self.get(e_coord.x, e_coord.y);
        let w_coord = self.start.x.checked_sub(1).map(|x| Point { x, y: self.start.y });
        let w_chr = w_coord.and_then(|p| self.get(p.x, self.start.y));
        if n_chr == Some('|') || n_chr == Some('F') || n_chr == Some('7') {
            points.push(n_coord.unwrap());
        }
        if s_chr == Some('|') || s_chr == Some('J') || s_chr == Some('L') {
            points.push(s_coord);
        }
        if e_chr == Some('-') || e_chr == Some('7') || e_chr == Some('J') {
            points.push(e_coord);
        }
        if w_chr == Some('-') || w_chr == Some('L') || w_chr == Some('F') {
            points.push(w_coord.unwrap());
        }
        points
    }

    fn get_next(&self, prev: Point, current: Point) -> Option<Point> {
        let from_direction = self.rel_direction(current, prev)?;
        let current_char = self.get(current.x, current.y)?;
        let next = match (from_direction, current_char) {
            (Direction::N, 'L') => Point{x: current.x + 1, y: current.y},
            (Direction::E, 'L') => Point{x: current.x, y: current.y - 1},
            (Direction::N, 'J') => Point{x: current.x - 1, y: current.y},
            (Direction::W, 'J') => Point{x: current.x, y: current.y - 1},
            (Direction::S, 'F') => Point{x: current.x + 1, y: current.y},
            (Direction::E, 'F') => Point{x: current.x, y: current.y + 1},
            (Direction::S, '7') => Point{x: current.x - 1, y: current.y},
            (Direction::W, '7') => Point{x: current.x, y: current.y + 1},
            (Direction::N, '|') => Point{x: current.x, y: current.y + 1},
            (Direction::S, '|') => Point{x: current.x, y: current.y - 1},
            (Direction::E, '-') => Point{x: current.x - 1, y: current.y},
            (Direction::W, '-') => Point{x: current.x + 1, y: current.y},
            _ => return None,
        };
        if self.get(next.x, next.y).is_some() {
            Some(next)
        } else {
            None
        }
    }
}

fn solve(grid: &Grid) -> anyhow::Result<usize> {
    let mut ends = grid.start_points();
    let mut prev = vec![grid.start; ends.len()];
    for step in 1..MAX_STEPS {
        for i in 0..ends.len() {
            let current_pt = ends[i];
            let prev_pt = prev[i];
            let next_pt = grid.get_next(prev_pt, current_pt).ok_or(anyhow::anyhow!("No next point"))?;
            if ends.contains(&next_pt) {
                return Ok(step + 1);
            }
            prev[i] = current_pt;
            ends[i] = next_pt;
        }
    }
    Err(anyhow::anyhow!("No solution found in {MAX_STEPS} steps"))
}


fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let grid = Grid::from_str(&buffer)?;
    let steps = solve(&grid)?;
    println!("Steps: {steps}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    const EXAMPLE: &str = include_str!("../../example.txt");
    const EXAMPLE2: &str = include_str!("../../example2.txt");

    #[test]
    fn test_example() {
        let grid = Grid::from_str(EXAMPLE).unwrap();
        let steps = solve(&grid).unwrap();
        assert_eq!(steps, 8);
    }

    #[test]
    fn test_example2() {
        let grid = Grid::from_str(EXAMPLE2).unwrap();
        let steps = solve(&grid).unwrap();
        assert_eq!(steps, 4);
    }
}
