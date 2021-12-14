use ownership::space;

use std::fmt;
use std::fmt::Formatter;
use main::Color;

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

    let city = main::City { name: "unknown", lat: 126.00334, lon: 123.8873747 };
    println!("{}", city);

    for city in [
        main::City { name: "beijing", lat: 116.402993, lon: 39.90009 },
        main::City { name: "shanghai", lat: 121.478773, lon: 31.23 },
        main::City { name: "hangzhou", lat: 120.152998, lon: 30.28983 }
    ].iter() {
        println!("{}", city)
    }

    for color in [
        Color { red: 255, green: 255, blue: 255 },
        Color { red: 255, green: 0, blue: 0 },
        Color { red: 0, green: 0, blue: 0 }
    ].iter() {
        println!("{:?}", *color);
        println!("RGB({}, {}, {}) 0x{:02X}{:02X}{:02X}", color.red, color.green, color.blue, color.red, color.green, color.blue);
    }

    // 字面量和字符串
    println!("1 + 2 = {}", 1u32 + 2);

    let res = guessing::tuple::reverse((1, true));
    println!("({},{})", res.0, res.1)
}