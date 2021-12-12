use ownership::space;

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}


fn main() {
    let str = String::from("a b c");
    let index = space::first_name(&str);
    println!("{}", index);
    println!("{}", &str[..index]);

    let p = Complex { real: 3.3, imag: 3.7 };
    println!("Display :{}", p);
    println!("Debug :{:?}", p);

    let list = main::List(vec![4, 5, 6]);
    println!("list: {}", list);
}
