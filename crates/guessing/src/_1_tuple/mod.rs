// 反转参数位置
pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    return (boolean, integer);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[cfg(test)]
mod tests {
    #[test]
    fn test_tuple() {
        let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64);
        println!("{:?}", long_tuple);
    }
}
