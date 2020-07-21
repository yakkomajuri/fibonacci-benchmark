fn fibonacci(n: u32) -> u32 {
    if n<=1 {
        1
    }
    else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    // TODO: Not sure why this needs to be one number less to get same value as others.
    println!("{}", fibonacci(39));
}
