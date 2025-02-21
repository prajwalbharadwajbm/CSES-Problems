use std::io;

fn main(){
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable Read Line");

    let mut max_length = 1;
    let mut current_length = 1;
    let mut prev_char: Option<char> = None;
    
    for current_char in input.trim().chars() {
        if let Some(prev) = prev_char{
            if prev == current_char {
                current_length += 1;
                max_length = max_length.max(current_length);
            } else {
                current_length = 1;
            }
        }
        prev_char = Some(current_char);
    }
    println!("{}", max_length);
}