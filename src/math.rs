use super::matrix::Matrix;

fn mat_mul(src1: &Matrix, src2: &Matrix) -> Matrix {
    let mut matrix = Matrix::with_zeros((src1.dim.0, src2.dim.1));

    for i in 0..src1.dim.0 {
        for j in 0..src2.dim.1 {
            let mut sum: f32 = 0.;
            for k in 0..src2.dim.0 {
                sum += src1.data[k + i*src1.dim.1] * src2.data[j + k*src2.dim.1];
            }
            matrix.data[j + i*matrix.dim.1] = sum;
        }
    }
    matrix
}

fn transpose(src: &Matrix) -> Matrix {
    let mut matrix = Matrix::with_zeros((src.dim.1, src.dim.0));
    for i in 0..src.dim.0 {
        for j in 0..src.dim.1 {
            matrix.data[i + j*matrix.dim.1] = src.data[j + i*src.dim.1]; 
        }
    }
    matrix
}
mod test {
    use super::*;

    #[test]
    fn test_mat_mul() {
        let matrix_1 = Matrix::new(&vec![vec![1.,2.,3.], vec![4.,5.,6.]]);
        let matrix_2 = Matrix::new(&vec![vec![1.,2.], vec![3.,4.], vec![5.,6.]]);
        
        assert_eq!(Matrix::new(&vec![vec![22.,28.], vec![49.,64.]]), mat_mul(&matrix_1, &matrix_2));
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix::new(&vec![vec![1.,2.,3.,4.,5.], vec![6.,7.,8.,9.,10.]]);
        assert_eq!(Matrix::new(&vec![vec![1.,6.], vec![2.,7.], vec![3.,8.], vec![4.,9.], vec![5.,10.]]), transpose(&matrix));
    }
}