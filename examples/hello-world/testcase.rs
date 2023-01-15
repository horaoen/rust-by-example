use std::fmt::{Display, UpperHex};

pub fn test_pi_fmt() {
    println!("PI format to {:.2}", 3.1415926);
}

pub fn test_complex_display() {
    let c1 = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", c1);
    println!("Debug: {:?}", c1);
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1} + {:.1}i", self.real, self.imag)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = &self.0;

        write!(f, "[")?;

        for (count, v) in start.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

pub fn show_list() {
    let list = List(vec![1, 2, 3, 4]);
    let list2 = vec![5, 6];
    println!("{}", list);

    println!("{:?}", list2);

}

struct Length(i32);

impl UpperHex for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

pub fn test_hex_fmt() {
    let l = Length(123456);
    println!("l as hex: {:X}", l);
}
