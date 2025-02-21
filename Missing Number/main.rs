use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Please provide the input in correct format");
    let n: i32 = input.trim().parse().expect("input must be an integer");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read array");
    let array: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a valid number"))
        .collect();

    let expected_sum: i64 = (n as i64 * (n as i64 + 1))/2;
    let actual_sum: i64 = array.iter().map(|&x| x as i64).sum();
    let missing_number = expected_sum - actual_sum;
    println!("{}", missing_number);
}