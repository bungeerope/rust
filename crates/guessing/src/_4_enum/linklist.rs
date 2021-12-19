use crate::_4_enum::linklist::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, element: u32) -> List {
        Cons(element, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

#[test]
fn test_link_list() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("this link-list's length is {}", list.len());
    println!("this link-list's context is {}", list.stringify());

    static LANGUAGE: &'static str = "RustCoding";
    #[allow(dead_code)]
    const THRESHOLD: i32 = 10;

    println!("this language is {}", LANGUAGE);

    let mut multi_integer = 7i32;
    {
        println!("{}", multi_integer);
        let multi_integer = multi_integer;
        println!("{}", multi_integer);
    }
    multi_integer = 50;
    println!("{}", multi_integer)
}