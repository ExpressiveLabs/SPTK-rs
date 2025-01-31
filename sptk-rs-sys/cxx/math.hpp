#include "../SPTK/include/SPTK/math/matrix.h"
#include "../SPTK/include/SPTK/math/matrix2d.h"

#include <memory>

namespace sptk {

    // MATRIX
    std::unique_ptr<Matrix> new_matrix(int num_row = 0, int num_column = 0) {
        return std::make_unique<Matrix>(num_row, num_column);
    }
    std::unique_ptr<Matrix> new_matrix_from_vector(int num_row, int num_column, const std::vector<double>& vector) {
        return std::make_unique<Matrix>(num_row, num_column, vector);
    }
    std::unique_ptr<Matrix> new_matrix_from_matrix(const Matrix& matrix) {
        return std::make_unique<Matrix>(matrix);
    }

    const double& matrix_at(const Matrix& matrix, int row, int column) {
        return matrix.At(row, column);
    }
    // -----------------------------------------------------


    // MATRIX 2D
    std::unique_ptr<Matrix2D> new_matrix2D() {
        return std::make_unique<Matrix2D>();
    }
    std::unique_ptr<Matrix2D> new_matrix2D_from_matrix2D(const Matrix2D& matrix2D) {
        return std::make_unique<Matrix2D>(matrix2D);
    }
    // -----------------------------------------------------

}
