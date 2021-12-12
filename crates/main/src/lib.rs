use std::fmt;
use std::fmt::{Formatter};

pub struct List(pub Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (index, ele) in vec.iter().enumerate() {
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}:{}", index, ele)?;
        }

        write!(f, "]")
    }
}