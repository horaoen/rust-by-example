use std::fmt::Display;

fn main() {
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));
    println!("{}", Matrix(1.1, 1.2, 2.1, 2.2));
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!();
    println!("{}", transpose(&matrix)); 
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn transpose(matrix: &Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}
