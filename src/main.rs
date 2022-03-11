use std::fmt::{Display, Formatter};

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix{

    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}
// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "( {a} {b} )\n", a=self.0, b=self.1)?;
        write!(f, "( {a} {b} )", a=self.2, b=self.3)
    }
}

fn main(){

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:  \n{}", matrix);

    println!("Transpose: \n{}", transpose(matrix));
}
