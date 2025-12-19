use std::fs::File;
use std::cmp::{self, Ordering};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::str::FromStr;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;
use std::num;
use regex::Regex;
use std::collections::HashSet;


//note the joltage in {} will probably be needed in part 2


//read lines function is from the Rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//AI says to use Gaussian Elimination solver
fn main() {
     let mut total_sum = 0;
     if let Ok(lines) = read_lines("query_input_day10.txt") {
       for line in lines.map_while(Result::ok) {
           let inp = line.trim();
	   if inp.is_empty() { continue; }
	   //for part I
           //let start_bracket = inp.find('[').unwrap();
           //let end_bracket = inp.find(']').unwrap();
           //let target_str = &inp[start_bracket+1..end_bracket];
    
           //let target_indices: Vec<usize> = target_str.chars()
             //   .enumerate()
               // .filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
              //  .collect();
	   //for part II
	   let start_brace = line.find('{').unwrap();
           let end_brace = line.find('}').unwrap();
           let target_str = &line[start_brace+1..end_brace];
    
           let target_values: Vec<f64> = target_str
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();
	   let re = Regex::new(r"\(([^)]+)\)").unwrap();
	   let buttons: Vec<Vec<usize>> = re.captures_iter(&inp).map(|cap| {
              cap[1]
                .split(',')
                .map(|s| s.trim().parse::<usize>().unwrap()) // Parse each number
                .collect()}).collect();
	    //we cannot use a DFS since the input is too large <- stated by AI
            //We make a specific machine for each line and find the strict maximum index for this specific machine
            //We check the target lights AND all button effects.
            //let max_target = target_values.iter().max();
            let max_button = buttons.iter().flatten().max();
    
            // The matrix needs rows 0..=max_index
            let search_limit = target_values.iter().cloned().fold(0./0., f64::max) as usize + 1;
            println!("target_indices are {:?} buttons are {:?}", &target_values, &buttons);
            println!("Machine uses indices up to: {}", search_limit);
    
            // 3. Solve with dynamic sizing
	    //for part I
            //match solve_min_presses(max_index, &target_indices, &buttons) 
	    //for part II
	    match solve_min_presses_linear(&target_values, &buttons, search_limit) {
                Some(presses) => { println!("Minimum presses required: {}", &presses); total_sum+=presses; },
                None => println!("This configuration is impossible."),
                }
	   }
    }
    println!("total sum is {:?}", &total_sum);
}

//AI involved solution for part I
fn solve_min_presses(max_index: usize, target: &[usize], buttons: &[Vec<usize>]) -> Option<usize> {
    let num_rows = max_index + 1;
    let num_cols = buttons.len();

    // Matrix: rows = lights, cols = buttons. Last col = target.
    // Using simple boolean matrix (true = 1, false = 0)
    let mut matrix = vec![vec![false; num_cols + 1]; num_rows];

    // Fill Matrix
    for (col, btn_indices) in buttons.iter().enumerate() {
        for &row in btn_indices {
            matrix[row][col] = true;
        }
    }
    for &row in target {
        matrix[row][num_cols] = true;
    }

    // --- Gaussian Elimination (Forward) ---
    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();

    for col in 0..num_cols {
        if pivot_row >= num_rows { break; }

        // Find pivot
        let mut selected_row = None;
        for row in pivot_row..num_rows {
            if matrix[row][col] {
                selected_row = Some(row);
                break;
            }
        }

        if let Some(swap_row) = selected_row {
            matrix.swap(pivot_row, swap_row);
            pivot_cols.push(col);

            // Eliminate other rows
            for row in 0..num_rows {
                if row != pivot_row && matrix[row][col] {
                    // Row XOR Pivot_Row
                    // We need to clone the pivot row to avoid borrow checker issues
                    // or use split_at_mut (as shown in previous answer).
                    // Here is a simpler, slightly slower clone version for clarity:
                    let pivot_vals = matrix[pivot_row].clone();
                    for i in col..=num_cols {
                        matrix[row][i] = matrix[row][i] ^ pivot_vals[i];
                    }
                }
            }
            pivot_row += 1;
        }
    }

    // --- Check Consistency ---
    for row in pivot_row..num_rows {
        if matrix[row][num_cols] {
            return None; // 0 = 1, Impossible
        }
    }
// 4. Find Minimum Solution by iterating Free Variables
    // Any column NOT in pivot_cols is a "Free Variable".
    let free_cols: Vec<usize> = (0..num_cols)
        .filter(|c| !pivot_cols.contains(c))
        .collect();

    let mut min_presses = usize::MAX;
    
    // If there are too many free variables, this could be slow, but for AoC
    // puzzles, K is usually small (< 20).
    let num_combinations = 1 << free_cols.len(); 

    for i in 0..num_combinations {
        let mut solution = vec![false; num_cols];
        
        // A. Set Free Variables based on the bits of 'i'
        for (bit_idx, &col_idx) in free_cols.iter().enumerate() {
            if (i >> bit_idx) & 1 == 1 {
                solution[col_idx] = true;
            }
        }

        // Solve for Pivot Variables
        // Because we did Gauss-Jordan (eliminated above and below), 
        // The equation for row 'r' (which corresponds to pivot_cols[r]) is:
        // Pivot + Sum(FreeVars * MatrixCoeffs) = Target
        // Pivot = Target ^ Sum(...)
        for (r, &p_col) in pivot_cols.iter().enumerate() {
            let mut val = matrix[r][num_cols]; // Start with target
            
            // XOR with the contribution from free variables in this row
            for &f_col in &free_cols {
                if matrix[r][f_col] && solution[f_col] {
                    val = !val;
                }
            }
            solution[p_col] = val;
        }

        // C. Count presses and track minimum
        let count = solution.iter().filter(|&&x| x).count();
        if count < min_presses {
            min_presses = count;
        }
    }

    if min_presses == usize::MAX { None } else { Some(min_presses) }
}

//AI proposed with dynamic search limit
fn solve_min_presses_linear(target: &[f64], buttons: &[Vec<usize>], limit: usize) -> Option<u64> {
    let num_rows = target.len();
    let num_cols = buttons.len();

    // Augmented Matrix: [Buttons | Target]
    let mut matrix = vec![vec![0.0; num_cols + 1]; num_rows];

    // Fill Matrix
    for (col, btn_indices) in buttons.iter().enumerate() {
        for &row in btn_indices {
            if row < num_rows {
                matrix[row][col] = 1.0;
            }
        }
    }
    for (row, &val) in target.iter().enumerate() {
        matrix[row][num_cols] = val;
    }

    // --- 1. Gaussian Elimination (Forward) ---
    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();

    for col in 0..num_cols {
        if pivot_row >= num_rows { break; }

        let mut selected_row = None;
        for row in pivot_row..num_rows {
            if matrix[row][col].abs() > 1e-9 {
                selected_row = Some(row);
                break;
            }
        }

        if let Some(swap_row) = selected_row {
            matrix.swap(pivot_row, swap_row);
            pivot_cols.push(col);

            let divisor = matrix[pivot_row][col];
            for j in col..=num_cols {
                matrix[pivot_row][j] /= divisor;
            }

            for row in 0..num_rows {
                if row != pivot_row && matrix[row][col].abs() > 1e-9 {
                    let factor = matrix[row][col];
                    for j in col..=num_cols {
                        matrix[row][j] -= factor * matrix[pivot_row][j];
                    }
                }
            }
            pivot_row += 1;
        }
    }

    // --- 2. Check Consistency ---
    for row in pivot_row..num_rows {
        if matrix[row][num_cols].abs() > 1e-9 {
            return None; // Impossible
        }
    }

    // --- 3. Identify Free Variables ---
    let free_cols: Vec<usize> = (0..num_cols)
        .filter(|c| !pivot_cols.contains(c))
        .collect();

    // --- 4. Search for Integer Solutions with Dynamic Limit ---
    let mut min_total_presses = u64::MAX;
    let mut solution_found = false;
    let mut free_values = vec![0.0; free_cols.len()];
    
    // Recursive closure to handle arbitrary number of free variables
    // We pass `limit` down to control the loop size
    fn search(
        idx: usize, 
        free_vals: &mut [f64], 
        free_cols: &[usize], 
        pivot_cols: &[usize], 
        matrix: &[Vec<f64>], 
        num_cols: usize,
        limit: usize,
        min_presses: &mut u64,
        found: &mut bool
    ) {
        if idx == free_cols.len() {
            // Calculate Pivots based on Free values
            let mut current_solution = vec![0.0; num_cols];
            for (i, &f_col) in free_cols.iter().enumerate() {
                current_solution[f_col] = free_vals[i];
            }

            let mut valid = true;

            // Solve for pivots
            for (r, &p_col) in pivot_cols.iter().enumerate() {
                let mut val = matrix[r][matrix[0].len() - 1]; // RHS
                for (i, &f_col) in free_cols.iter().enumerate() {
                    val -= matrix[r][f_col] * free_vals[i];
                }

                // PRUNING: If a pivot becomes negative, this branch is invalid.
                // Since coefficients are positive, increasing free vars will only make this more negative.
                if val < -1e-9 {
                    valid = false;
                    break;
                }
                
                // Check Integer
                if (val.round() - val).abs() > 1e-9 {
                    valid = false;
                    break;
                }
                current_solution[p_col] = val.round();
            }

            if valid {
                let sum: u64 = current_solution.iter().map(|&x| x as u64).sum();
                if sum < *min_presses {
                    *min_presses = sum;
                    *found = true;
                }
            }
            return;
        }

        // Loop up to the Dynamic Limit
        for val in 0..=limit { 
            free_vals[idx] = val as f64;
            search(idx + 1, free_vals, free_cols, pivot_cols, matrix, num_cols, limit, min_presses, found);
            
            // Optimization: If we found a valid solution and adding more to this free var 
            // drives the total sum way above min_presses, we could break here.
            // But with max=136 and few free vars, simple recursion is usually fast enough.
        }
    }

    search(
        0, 
        &mut free_values, 
        &free_cols, 
        &pivot_cols, 
        &matrix, 
        num_cols,
        limit,
        &mut min_total_presses, 
        &mut solution_found
    );

    if solution_found {
        Some(min_total_presses)
    } else {
        None
    }
}