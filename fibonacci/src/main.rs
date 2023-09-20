use std::io;

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("Enter index:");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read stdin.");

    let n: u32 = n.trim().parse().expect("Enter a valid number.");

    let res = fib(n - 1);

    println!("The {n}th fibonacci number is {res}");
}
