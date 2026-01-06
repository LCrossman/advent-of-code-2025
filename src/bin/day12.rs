use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point { x: i32, y: i32 }

type Shape = Vec<Point>;

struct Solver {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
}

impl Solver {
    fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            grid: vec![vec![false; w]; h],
        }
    }

    // Normalizes a shape so its top-leftmost point is at (0,0)
    fn normalize(mut shape: Shape) -> Shape {
        let min_x = shape.iter().map(|p| p.x).min().unwrap();
        let min_y = shape.iter().map(|p| p.y).min().unwrap();
        for p in &mut shape {
            p.x -= min_x;
            p.y -= min_y;
        }
        shape.sort_by_key(|p| (p.y, p.x));
        shape
    }

    // Generates all 8 possible symmetries (rotations + flips)
    fn get_all_variants(shape: &Shape) -> Vec<Shape> {
        let mut variants = HashSet::new();
        let mut current = shape.clone();

        for _ in 0..4 {
            // Rotations
            current = Self::normalize(current.iter().map(|p| Point { x: -p.y, y: p.x }).collect());
            variants.insert(current.clone());
            // Flips
            let flipped = Self::normalize(current.iter().map(|p| Point { x: -p.x, y: p.y }).collect());
            variants.insert(flipped);
        }
        variants.into_iter().collect()
    }

    fn can_place(&self, shape: &[Point], r: usize, c: usize) -> bool {
        for p in shape {
            let nr = r as i32 + p.y;
            let nc = c as i32 + p.x;
            if nr < 0 || nr >= self.height as i32 || nc < 0 || nc >= self.width as i32 
               || self.grid[nr as usize][nc as usize] {
                return false;
            }
        }
        true
    }

    fn place(&mut self, shape: &[Point], r: usize, c: usize, val: bool) {
        for p in shape {
            self.grid[(r as i32 + p.y) as usize][(c as i32 + p.x) as usize] = val;
        }
    }

    fn solve(&mut self, counts: &mut Vec<usize>, shapes: &[Vec<Shape>]) -> bool {
    // Base case: all required shapes placed
         if counts.iter().all(|&c| c == 0) {
            return true;
          }
        let remaining_area: usize = counts.iter().zip(shapes).map(|(&n, vars)| n * vars[0].len()).sum();
	let free_cells = self.grid.iter().flatten().filter(|&&c| !c).count();
	if remaining_area > free_cells {
	   return false;
	   }
        // Choose next shape index (simple heuristic: first remaining)
        let i = counts.iter().enumerate().filter(|&(_, &c)| c > 0).max_by_key(|&(i, _)| shapes[i][0].len()).map(|(i, _)| i).unwrap();

        for variant in &shapes[i] {
            for r in 0..self.height {
                 for c in 0..self.width {
                    if self.can_place(variant, r, c) {
                        self.place(variant, r, c, true);
                        counts[i] -= 1;

                        if self.solve(counts, shapes) {
                            return true;
                        }

                         counts[i] += 1;
                        self.place(variant, r, c, false);
                    }
                }
            }
        }
        false
    }
}


// Reusing the read_lines helper
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
let mut shapes_raw: Vec<Vec<Point>> = Vec::new();
    let mut parsing_shapes = true;
    let mut current_shape: Vec<Point> = Vec::new();
    let mut y_offset = 0;
    let mut total_fits = 0;

    if let Ok(lines) = read_lines("query_input_day12.txt") {
        for line in lines.map_while(Result::ok) {
            let line = line.trim();
            if line.is_empty() {
                if !current_shape.is_empty() {
                    shapes_raw.push(Solver::normalize(current_shape.clone()));
                    current_shape.clear();
                    y_offset = 0;
                }
                continue;
            }

            //detect transition to regions
            if line.contains('x') && line.contains(':') {
                if parsing_shapes && !current_shape.is_empty() {
                    shapes_raw.push(Solver::normalize(current_shape.clone()));
                    current_shape.clear();
                }
                parsing_shapes = false;
            }

            if parsing_shapes {
                //if it's a "0:", "1:", etc. header, reset y_offset for the new shape
                if line.ends_with(':') {
                    if !current_shape.is_empty() {
                        shapes_raw.push(Solver::normalize(current_shape.clone()));
                        current_shape.clear();
                    }
                    y_offset = 0;
                    continue;
                }
                
                for (x, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        current_shape.push(Point { x: x as i32, y: y_offset });
                    }
                }
                y_offset += 1;
            } else {
                //region section
                let parts: Vec<&str> = line.split(':').collect();
                let dims: Vec<usize> = parts[0].split('x').map(|s| s.trim().parse().unwrap()).collect();
                let counts: Vec<usize> = parts[1].split_whitespace().map(|s| s.parse().unwrap()).collect();

                let (w, h) = (dims[0], dims[1]);
                
                //pre-generate symmetries only once for efficiency
                let shape_variants: Vec<Vec<Shape>> = shapes_raw.iter().map(|s| Solver::get_all_variants(s)).collect();

                let mut solver = Solver::new(w, h);
		let mut counts = counts;
                if solver.solve(&mut counts, &shape_variants) {
                    println!("Region {}x{} fits!", w, h);
                    total_fits += 1;
                } else {
                    println!("Region {}x{} does NOT fit.", w, h);
                }
            }
        }
    }
    println!("--- Total fitting regions: {} ---", total_fits);
    Ok(())
}
