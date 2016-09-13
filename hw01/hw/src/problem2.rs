/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    assert_eq!(mat1[0].len(), mat2.len());
    mat1.iter().enumerate().map(|(row_ind, row)| {
        (0..mat2[row_ind].len()).map(|col_ind| {
            row.iter().enumerate().fold(0., |acc, (i, item)| {
                acc + (item * mat2[i][col_ind])
            })
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}
