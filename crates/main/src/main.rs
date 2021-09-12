use ownership::space;

fn main() {
    let str = String::from("a b c");
    let index = space::first_name(&str);
    println!("{}", index);
    println!("{}", &str[..index])
}
