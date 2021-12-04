use std::env;
use std::fs;

fn main() {

    // There must be exactly one argument, the path to the input file.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected exactly one argument, the path to the input file.");
    }

    // Input file must exist
    let input_path = &args[1];
    let input_file = fs::read_to_string(input_path).expect("Failed to read input file.");

    // Each of the M lines contains N bits, we want to build a M x N matrix using them.
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    for line in input_file.lines() {
        let mut row: Vec<u8> = Vec::new();
        for bit in line.chars() {
            row.push(bit.to_digit(2).unwrap() as u8);
        }
        matrix.push(row);
    }

    // Get N
    let n = matrix[0].len();

    // Now we want to get the mode value for each column.
    let mut column_modes: Vec<u8> = Vec::new();
    for column in 0..matrix[0].len() {
        let mut mode = 0;
        let mut count = 0;
        for row in 0..matrix.len() {
            if matrix[row][column] == 1 {
                count += 1;
            }
        }
        if count > matrix.len() / 2 {
            mode = 1;
        }
        column_modes.push(mode);
    }

    // Now we're going to build a binary number using the column modes.
    let mut code: u32 = 0;
    for i in 0..column_modes.len() {
        code += column_modes[i] as u32 * 2_u32.pow((n as u32-1)-i as u32);
    }

    // Then we need to apply NOT to the column_modes.
    let mut code_not: u32 = 0;
    for i in 0..column_modes.len() {
        if column_modes[i] == 0 {
            code_not += 2_u32.pow((n as u32 -1)-i as u32);
        }
    }

    // And finally we multiply the two numbers.
    println!("{}", code * code_not);
}
