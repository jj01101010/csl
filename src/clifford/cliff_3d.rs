// This is only a test module to see how this will evolve
// This will change in the future!

use std::ops::{Add, AddAssign};

// TODO: Implement with ndarrays

#[derive(Clone, Copy)]
pub struct Clifford {
    factors: [f32; 8], // these are the factors of the Clifford algebra in 3D
}

impl Add for Clifford {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut factors = [0.0; 8];

        for (i, (a, b)) in self.factors.iter().zip(rhs.factors).enumerate() {
            factors[i] = a + b;
        }

        Clifford { factors }
    }
}

impl AddAssign for Clifford {
    fn add_assign(&mut self, rhs: Self) {
        self.factors = (*self + rhs).factors;
    }
}
