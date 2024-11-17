fn main() {
    let mut sum: u32 = 0;
    let mut i: u32 = 0;
    while fibonacci(i) < 4_000_000 {
        if fibonacci(i) % 2 == 0 {
            sum += fibonacci(i);
        }
        i += 1;
    }
    println!("{}", sum);
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
