use std::ops::{Add, AddAssign, Index, IndexMut, Mul};

#[derive(Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize> {
    pub data: [[f32; N]; M], // TODO: This needs to be generalized with Ndarray<Ndarray, 1>
}

pub type Vector<const N: usize> = Matrix<N, 1>;

impl<const L: usize, const M: usize, const N: usize> Mul<Matrix<M, N>> for Matrix<L, M> {
    type Output = Matrix<L, N>;
    fn mul(self, rhs: Matrix<M, N>) -> Self::Output {
        //TODO: THIS NEEDS TO BE REPLACED WITH A MORE SOPHISTICATED ALGORITHM
        //  to reduce O(N^3) and increase caching efficiency
        let mut output = [[0.0; N]; L];
        for i in 0..L {
            for k in 0..N {
                let mut sum = 0.0;
                for j in 0..M {
                    sum += self.data[i][j] * rhs.data[j][k];
                }
                output[i][k] = sum;
            }
        }
        Matrix { data: output }
    }
}

impl<const M: usize, const N: usize> Add for Matrix<M, N> {
    type Output = Matrix<M, N>;
    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..M {
            for j in 0..N {
                self[(i, j)] += rhs[(i, j)];
            }
        }
        self
    }
}

impl<const M: usize, const N: usize> AddAssign for Matrix<M, N> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                self[(i, j)] += rhs[(i, j)];
            }
        }
    }
}

impl<const M: usize, const N: usize> Mul<Matrix<M, N>> for f32 {
    type Output = Matrix<M, N>;
    fn mul(self, mut rhs: Matrix<M, N>) -> Matrix<M, N> {
        for i in 0..M {
            for j in 0..N {
                rhs[(i, j)] *= self;
            }
        }
        rhs
    }
}

impl<const M: usize, const N: usize> Mul<f32> for Matrix<M, N> {
    type Output = Matrix<M, N>;
    fn mul(mut self, rhs: f32) -> Matrix<M, N> {
        for i in 0..M {
            for j in 0..N {
                self[(i, j)] *= rhs;
            }
        }
        self
    }
}

impl<const M: usize, const N: usize> Index<(usize, usize)> for Matrix<M, N> {
    type Output = f32;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index][0]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index][1]
    }
}

impl<const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<M, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<const N: usize> From<[f32; N]> for Vector<N> {
    fn from(value: [f32; N]) -> Self {
        Self {
            data: value.map(|val| [val]),
        }
    }
}
