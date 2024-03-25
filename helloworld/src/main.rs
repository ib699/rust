// Tuples can be used as function arguments and as return values.
fn transpose(pair: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int1_param, int2_param, int3_param, int4_param) = pair;

    (int1_param, int2_param, int3_param, int4_param)
}

use std::fmt;

// The following struct is for the activity.
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write each element of the matrix in the desired format.
        write!(
            f,
            "({} {}, {} {})",
            self.0, self.1, self.2, self.3
        )
    }
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
