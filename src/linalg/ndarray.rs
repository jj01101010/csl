use std::ops::{Add, AddAssign, Index, Mul, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Matrix {
    pub data: [[f32; 4]; 4], // TODO: This needs to be generalized with Ndarray<Ndarray, 1>
}

#[derive(Clone, Copy, Debug)]
pub struct Vector<const N: usize> {
    pub data: [f32; N],
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        //TODO: THIS NEEDS TO BE REPLACED WITH A MORE SOPHISTICATED ALGORITHM
        //  to reduce O(N^3) and increase caching efficiency
        let mut output = [[0.0; 4]; 4];
        for i in 0..4 {
            for k in 0..4 {
                let mut sum = 0.0;
                for j in 0..4 {
                    sum += self.data[i][j] * rhs.data[j][k];
                }
                output[i][k] = sum;
            }
        }
        Matrix { data: output }
    }
}

impl Mul<Vector<4>> for Matrix {
    type Output = Vector<4>;
    fn mul(self, rhs: Vector<4>) -> Self::Output {
        let mut output = [0.0; 4];
        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..4 {
                sum += self.data[i][j] * rhs.data[j];
            }
            output[i] = sum;
        }
        Vector { data: output }
    }
}

impl<const N: usize> Add<Vector<N>> for Vector<N> {
    type Output = Vector<N>;
    fn add(self, rhs: Vector<N>) -> Self::Output {
        let mut out = [0.0; N];
        for i in 0..N {
            out[i] = self.data[i] + rhs.data[i];
        }
        Vector { data: out }
    }
}

impl<const N: usize> AddAssign<Vector<N>> for Vector<N> {
    fn add_assign(&mut self, rhs: Vector<N>) {
        for i in 0..N {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<const N: usize> Sub<Vector<N>> for Vector<N> {
    type Output = Vector<N>;
    fn sub(self, rhs: Vector<N>) -> Self::Output {
        let mut out = [0.0; N];
        for i in 0..N {
            out[i] = self.data[i] - rhs.data[i];
        }
        Vector { data: out }
    }
}

impl<const N: usize> SubAssign<Vector<N>> for Vector<N> {
    fn sub_assign(&mut self, rhs: Vector<N>) {
        for i in 0..N {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<const N: usize> Mul<Vector<N>> for f32 {
    type Output = Vector<N>;
    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let mut out = [0.0; N];
        for i in 0..N {
            out[i] = self * rhs.data[i];
        }
        Vector { data: out }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
