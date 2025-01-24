use cxx::{CxxVector, UniquePtr};
use sptk_sys::ffi as sys;

pub struct Dimensions {
    pub col_offset: i32,
    pub num_col: i32,
    pub row_offset: i32,
    pub num_row: i32,
}

pub struct Matrix {
    foreign: UniquePtr<sys::Matrix>,
}

impl Matrix {
    pub fn new(num_row: i32, num_column: i32) -> Self {
        let foreign = sys::new_matrix(num_row, num_column);
        Self { foreign }
    }

    pub fn new_from_vec(num_row: i32, num_column: i32, data: &Vec<f64>) -> Self {
        let mut v = CxxVector::new();
        for d in data {
            v.as_mut().unwrap().push(*d);
        }

        let foreign = sys::new_matrix_from_vector(num_row, num_column, &v);
        Self { foreign }
    }

    pub fn get_num_row(&self) -> i32 {
        self.foreign.GetNumRow()
    }

    pub fn get_num_column(&self) -> i32 {
        self.foreign.GetNumColumn()
    }

    pub fn get_submatrix(&self, row_offset: i32, col_offset: i32, submatrix: &mut Self) {
        unsafe {
            self.foreign.GetSubmatrix(
                row_offset,
                submatrix.get_num_row(),
                col_offset,
                submatrix.get_num_column(),
                submatrix.foreign.as_mut_ptr(),
            );
        }
    }
}

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix")
            .field("num_row", &self.get_num_row())
            .field("num_column", &self.get_num_column())
            .finish()
    }
}

// impl std::fmt::Display for Matrix {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Write the contents of the matrix in a grid
//         let mut s = String::new();
//         for i in 0..self.get_num_row() {
//             for j in 0..self.get_num_column() {
//                 s.push_str(&format!("{:.2} ", self.foreign.At(i, j)));
//             }
//             s.push_str("\n");
//         }

//         write!(f, "{}", s)
//     }
// }

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    fn generate_noise_matrix() -> Matrix {
        let r = 20;
        let c = 20;
        let mut matrix = Matrix::new(r, c);

        let mut rng = rand::thread_rng();

        for i in 0..r {
            for j in 0..c {
                *matrix.foreign.as_mut().unwrap().At(i, j) = rng.gen_range(0.0..1.0);
            }
        }

        matrix
    }

    #[test]
    fn test_matrix_impl() {
        let matrix = generate_noise_matrix();

        println!("{:?}", matrix);
        println!("Num rows: {}", matrix.get_num_row());

        let mut submatrix = Matrix::new(1, 5);
        matrix.get_submatrix(0, 0, &mut submatrix);

        println!("{:?}", submatrix);
    }
}
