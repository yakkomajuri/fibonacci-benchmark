fn fibonacci(n: u32) -> u32 {
    if n==1 {
        1
    }
    else if n==0 {
        0
    }
    else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("{}", fibonacci(40));
}
