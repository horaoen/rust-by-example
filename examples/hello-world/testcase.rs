use std::fmt::Display;

pub fn test_pi_fmt() {
    println!("PI format to {:.2}", 3.1415926);
}

pub fn test_complex_display() {
   let c1 = Complex { real: 3.3, imag: 7.2};
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
