// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    let i = n;
    for i in 1..=i {
        result *= i;
    }
    result
}
