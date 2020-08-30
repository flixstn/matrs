use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
struct Matrix {
    dim: (usize, usize),    // (row, col)||(width, height)
    data: Vec<f32>
}

// (4,3) -> let mut vector = vec![vec![0.; dim.1]; dim.0]; (4 rows, 3, cols)
impl Matrix {
    fn new(data: &Vec<Vec<f32>>) -> Self {
        Self {
            dim: (data.len(), data[0].len()),
            data: data.iter().cloned().flatten().collect::<Vec<f32>>(),
        }
    }

    fn with_zeros(shape: (usize, usize)) -> Self {
        Self {
            dim: shape,
            data: vec![0.; shape.0 * shape.1],
        }
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // TODO: implement dimension check

        Self {
            dim: self.dim,
            data: self.data.iter().zip(&other.data).map(|(a,b)| *a * *b).collect(),
        }
    }
}

impl Mul<f32> for Matrix {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            dim: self.dim,
            data: self.data.iter().map(|a| *a * other).collect(),
        }
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            dim: self.dim,
            data: self.data.iter().zip(&other.data).map(|(a,b)| *a+*b).collect(),
        }
    }
}

impl Add<f32> for Matrix {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            dim: self.dim,
            data: self.data.iter().map(|a| *a + other).collect(),
        }
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            dim: self.dim,
            data: self.data.iter().zip(&other.data).map(|(a,b)| *a-*b).collect(),
        }
    }
}

impl Sub<f32> for Matrix {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            dim: self.dim,
            data: self.data.iter().map(|a| *a - other).collect(),
        }
    }    
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.dim == other.dim
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // assert_eq!(expected, computed);

    #[test]
    fn new() {
        let matrix = Matrix {
            dim: (2,2),
            data: vec![1.,2.,3.,4.]
        };

        assert_eq!(Matrix::new(&vec![vec![1.,2.], vec![3.,4.]]), matrix);
    }

    #[test]
    fn with_zeros() {
        let matrix = Matrix::with_zeros((4,3));

        assert_eq!(Matrix{dim: (4,3), data: vec![0.; 4 * 3]}, matrix);
    }

    #[test]
    fn mul() {
        let matrix_1 = Matrix{dim:(2,2), data: vec![1.,2.,3.,4.]};
        let matrix_2 = Matrix{dim:(2,2), data: vec![4.,3.,2.,1.]};
        let matrix_3 = matrix_1 * matrix_2;
        assert_eq!(Matrix{dim:(2,2), data: vec![4.,6.,6.,4.]}, matrix_3);
    }

    #[test]
    fn single_mul() {
        let matrix = Matrix{dim:(2,2), data: vec![1.,2.,3.,4.]};
        let matrix_mul = matrix * 2.;
        assert_eq!(Matrix{dim: (2,2), data: vec![2.,4.,6.,8.]}, matrix_mul);
    }

    #[test]
    fn add() {
        let matrix_1 = Matrix{dim:(2,2), data: vec![1.,2.,3.,4.]};
        let matrix_2 = Matrix{dim:(2,2), data: vec![4.,3.,2.,1.]};
        let matrix_3 = matrix_1 + matrix_2;
        assert_eq!(Matrix{dim:(2,2), data: vec![5.,5.,5.,5.]}, matrix_3);
    }

    #[test]
    fn single_add() {
        let matrix = Matrix{dim:(2,2), data: vec![1.,2.,3.,4.]};
        let matrix_mul = matrix + 2.;
        assert_eq!(Matrix{dim: (2,2), data: vec![3.,4.,5.,6.]}, matrix_mul); 
    }

    #[test]
    fn sub() {
        let matrix_1 = Matrix{dim:(2,2), data: vec![1.,2.,3.,4.]};
        let matrix_2 = Matrix{dim:(2,2), data: vec![4.,3.,2.,1.]};
        let matrix_3 = matrix_1 - matrix_2;
        assert_eq!(Matrix{dim:(2,2), data: vec![-3.,-1.,1.,3.]}, matrix_3);
    }

    #[test]
    fn single_sub() {
        let matrix = Matrix{dim:(2,2), data: vec![1.,2.,3.,4.]};
        let matrix_mul = matrix - 2.;
        assert_eq!(Matrix{dim: (2,2), data: vec![-1.,0.,1.,2.]}, matrix_mul);   
    }
}
