/// Return `12` if `n` is even,
/// `13` if `n` is divisible by `3`,
/// `17` otherwise.
pub fn magic_number(n: u32) -> u32 {
    if n % 2 == 0 {
        return 12;
    } else if n % 3 == 0 {
        return 13;
    } else { return 17 }
}
