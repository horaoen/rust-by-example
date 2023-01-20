/// fmt::Display  automagically provides ToString and
/// also allows printing the type.
use std::fmt;
use std::str::FromStr;

pub struct Circle {
    pub radius: i32,
}

impl Circle {
    pub fn new(radius: i32) -> Self {
        Self { radius }
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// This will convert the string into the type specified
// as long as the FromStr trait is implemented for that type.
impl FromStr for Circle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: i32 = s.parse().unwrap();

        Ok(Circle { radius: s })
    }
}
