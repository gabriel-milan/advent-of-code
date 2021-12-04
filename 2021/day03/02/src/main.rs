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

    // Now we need to generate a deep copy of the matrix.
    // We need to do this because we may have conflicts when
    // computing CO2 and O2 ratings.
    let mut matrix_co2: Vec<Vec<u8>> = Vec::new();
    for row in &matrix {
        let mut row_co2: Vec<u8> = Vec::new();
        for bit in row {
            row_co2.push(*bit);
        }
        matrix_co2.push(row_co2);
    }

    // Then we iterate over the N columns of the matrices,
    // getting the mode for each and then filtering out the
    // rows whose nth bit is not the mode.
    for nth_bit in 0..n {

        // Get M
        let m_o2 = matrix.len();
        let m_co2 = matrix_co2.len();

        if (m_o2 == 1) && (m_co2 == 1) {
            break;
        }

        // Finding mode for nth bit
        // If counts are equal, O2 must have mode 1
        // and CO2 must have mode 0.
        let mut mode_o2 = 0;
        let mut mode_co2 = 0;
        let mut count_o2 = 0;
        let mut count_co2 = 0;
        for row in 0..matrix.len() {
            if matrix[row][nth_bit] == 1 {
                count_o2 += 1;
            }
        }
        for row in 0..matrix_co2.len() {
            if matrix_co2[row][nth_bit] == 1 {
                count_co2 += 1;
            }
        }
        if count_o2 as f64 >= matrix.len() as f64 / 2.0 {
            mode_o2 = 1;
        }
        if (count_co2 as f64) < matrix_co2.len() as f64 / 2.0 {
            mode_co2 = 1 // CO2 mode is reversed
        }

        // Filtering out rows if M > 1
        if m_o2 > 1 {
            matrix.retain(|row| row[nth_bit] == mode_o2);
        }
        if m_co2 > 1 {
            matrix_co2.retain(|row| row[nth_bit] == mode_co2);
        }

    }

    // Convert the only remaining row from binary to decimal
    let mut sum_o2 = 0;
    let mut sum_co2 = 0;
    for i in 0..matrix[0].len() {
        sum_o2 += matrix[0][i] as u32 * 2_u32.pow((matrix[0].len() as u32-1)-i as u32);
    }
    for i in 0..matrix_co2[0].len() {
        sum_co2 += matrix_co2[0][i] as u32 * 2_u32.pow((matrix_co2[0].len() as u32-1)-i as u32);
    }

    // Finally, multiply the two sums
    println!("{}", sum_o2 * sum_co2);

}
