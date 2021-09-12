pub fn first_name(str: &String) -> usize {
    let str_byte = str.as_bytes();
    for (index, &item) in str_byte.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }
    str_byte.len()
}