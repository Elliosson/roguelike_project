pub fn add_or_zero(u: u32, i: i32) -> u32 {
    if (u as i32 + i) < 0 {
        return 0;
    } else {
        return u + i as u32;
    }
}
