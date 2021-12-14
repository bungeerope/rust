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


pub struct City {
    pub name: &'static str,
    pub lat: f32,
    pub lon: f32,
}


impl fmt::Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_char = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_char = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_char, self.lon, lon_char)
    }
}