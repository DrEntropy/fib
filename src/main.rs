use std::io;

fn main() {
    println!("Generate the nth fibonacci number.");
    println!("Enter n:");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read a line");
    let n:u64 = buffer.trim().parse().expect("Enter a number");

    println!("You chose {n}");
    println!("Fib(n): {}",fib(n));

}

fn fib(n:u64) -> u64 {
    if n==0 {
        0
    } else if n==1 {
        1
    } else {
    fib(n - 1) + fib(n - 2)
    }
}