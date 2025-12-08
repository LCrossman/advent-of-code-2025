use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // 1. Read lines into a Grid (Vector of Vector of chars)
    // better not to use split_whitespace because the spaces can be used as place values
    let lines = read_lines("query_input_day6.txt").unwrap();
    let raw_rows: Vec<String> = lines.map_while(Result::ok).collect();

    if raw_rows.is_empty() { return; }

    // Calculate dimensions
    let height = raw_rows.len();
    // Find the widest row to create a square grid
    let width = raw_rows.iter().map(|s| s.len()).max().unwrap_or(0);

    // Create the grid and pad it with spaces
    let mut grid = vec![vec![' '; width]; height];
    for (y, row) in raw_rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            grid[y][x] = ch;
        }
    }

    let mut grand_total: u64 = 0;
    
    // we can use a State machine for the problem block
    let mut current_operands: Vec<u64> = Vec::new();
    let mut current_operator = None;

    // 2. Scan Column by Column (Left to Right)
    for x in 0..width {
        let mut col_digits = String::new();
        let mut column_is_empty = true;

        // Check the bottom row for an operator (Sign Row)
        // If input has 4 number rows + 1 sign row, height-1 is the sign row.
        let op_char = grid[height - 1][x];
        if op_char == '+' || op_char == '*' {
            current_operator = Some(op_char);
            column_is_empty = false;
        }

        // Check the rows above the sign for digits
        // This loop whether there are 3 rows or 4 rows of numbers
        for y in 0..height - 1 {
            let ch = grid[y][x];
            if ch.is_digit(10) {
                col_digits.push(ch); // Most significant digit is at top (y=0)
                column_is_empty = false;
            } else if ch != ' ' {
                column_is_empty = false; // Non-space, non-digit char
            }
        }

        // 3. Logic: Did we find a number, an empty separator, or just continue?
        if !col_digits.is_empty() {
            // We found a number in this column! 
            // Building the string top to bottom, for normal reading order.
            let num = col_digits.parse::<u64>().unwrap();
            current_operands.push(num);
        } else if column_is_empty {
            // A completely empty column (spaces) marks the end of a problem
            if !current_operands.is_empty() {
                grand_total += solve_problem(&current_operands, current_operator);
                
                // Reset for next problem
                current_operands.clear();
                current_operator = None;
            }
        }
    }

    // Don't forget the very last problem if the file doesn't end with spaces
    if !current_operands.is_empty() {
        grand_total += solve_problem(&current_operands, current_operator);
    }

    println!("Grand Total: {}", grand_total);
}

fn solve_problem(nums: &[u64], op: Option<char>) -> u64 {
    // Default to + if something is weird, but puzzle implies op is always present
    let operator = op.unwrap_or('+');
    
    match operator {
        '+' => nums.iter().sum(),
        '*' => nums.iter().product(),
        _ => 0,
    }
}

//read lines function from the Rust book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}