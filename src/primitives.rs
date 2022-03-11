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


fn save_primitive(){

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:  \n{}", matrix);

    println!("Transpose: \n{}", transpose(matrix));
}


use std::mem;

// This function borrows a slice
#[warn(dead_code)]
#[warn(unused_variables)]
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}


fn test_value (){
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let _ys: [i32; 500] = [0; 500];


    // // `len` returns the count of elements in the array
    // println!("number of elements in array: {}", xs.len());
    //
    // // Arrays are stack allocated
    // println!("array occupies {} bytes", mem::size_of_val(&xs));
    //
    // // Arrays can be automatically borrowed as slices
    // println!("borrow the whole array as a slice");
    // analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&_ys[1 .. 4]);

    // Out of bound indexing causes compile error
    println!("{}", xs[0]);
}