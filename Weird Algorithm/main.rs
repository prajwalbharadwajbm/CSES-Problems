use std::io;

fn main(){
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut n: i64 = input.trim().parse()
        .expect("Please enter a valid number");

    print!("{} ",n);
    while n!=1{
        if n % 2 == 0{
            n = n/2;
        } else {
            n = n * 3 + 1;
        }
        print!("{} ",n);
    }
    println!();
}