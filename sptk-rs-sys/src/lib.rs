use cxx::{CxxVector, UniquePtr, WeakPtr};
use ffi::new_matrix;

// pub mod wrapper;

#[cxx::bridge(namespace = "sptk")]
pub mod ffi {

    unsafe extern "C++" {
        include!("sptk-rs/cxx/math.hpp");

        pub type Matrix;
        pub fn GetNumRow(&self) -> i32;
        pub fn GetNumColumn(&self) -> i32;
        pub fn Fill(self: Pin<&mut Self>, value: f64);
        pub fn FillDiagonal(self: Pin<&mut Self>, value: f64);

        pub fn At(self: Pin<&mut Self>, row: i32, column: i32) -> &mut f64;
        // pub fn At(&self, row: i32, column: i32) -> f64;

        pub unsafe fn GetSubmatrix(
            &self,
            row_offset: i32,
            num_row_of_submatrix: i32,
            column_offset: i32,
            num_column_of_submatrix: i32,
            submatrix: *mut Matrix,
        ) -> bool;

        pub fn new_matrix(num_row: i32, num_column: i32) -> UniquePtr<Matrix>;
        pub fn new_matrix_from_vector(
            num_row: i32,
            num_column: i32,
            vector: &CxxVector<f64>,
        ) -> UniquePtr<Matrix>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let matrix = ffi::new_matrix(2, 10);
        println!("Num rows: {}", matrix.GetNumRow());
    }
}
