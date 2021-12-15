#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub struct Nil;

pub struct Pair(i32, i32);

#[allow(dead_code)]
pub struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
pub struct Rectangle {
    pox: Point,
    poy: Point,
}

pub fn get_rectangle() -> Rectangle {
    return Rectangle {
        pox: Point { x: 12, y: 10 },
        poy: Point { x: 24, y: 9 },
    };
}


pub fn get_nil() -> Nil {
    return Nil;
}

pub fn init_pair(x: i32, y: i32) -> Pair {
    return Pair(x, y);
}