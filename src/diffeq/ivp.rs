use crate::linalg::ndarray::Vector;

const B_J: [f32; 4] = [1.0 / 6.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 6.0];

const C_J: [f32; 4] = [0.0, 0.5, 0.5, 1.0];

const A_IJ: [[f32; 4]; 4] = [
    [0.0, 0.0, 0.0, 0.0],
    [1.0 / 2.0, 0.0, 0.0, 0.0],
    [0.0, 1.0 / 2.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
];

pub type SolveFun<T> = dyn Fn(f32, T) -> T;

pub struct RungeKutta4<const N: usize> {
    time: f32,
    state: Vector<N>,
}

impl<const N: usize> RungeKutta4<N> {
    pub fn new(time: f32, initial_state: Vector<N>) -> Self {
        Self {
            time,
            state: initial_state,
        }
    }
}

impl<const N: usize> RungeKutta4<N> {
    fn calc_kj(&self, j: usize, fun: &SolveFun<Vector<N>>, delta_time: f32) -> Vector<N> {
        let mut inner_sum = Vector { data: [[0.0]; N] };

        for l in 0..4 {
            if A_IJ[j][l] == 0.0 {
                continue;
            }
            inner_sum += A_IJ[j][l] * self.calc_kj(l, fun, delta_time);
        }

        fun(
            self.time + delta_time * C_J[j],
            self.state + delta_time * inner_sum,
        )
    }

    pub fn next_step(&mut self, fun: &SolveFun<Vector<N>>, delta_time: f32) -> &Vector<N> {
        let mut sum = Vector { data: [[0.0]; N] };
        for (j, b_j) in B_J.iter().enumerate() {
            sum += *b_j * self.calc_kj(j, fun, delta_time);
        }

        self.time += delta_time;
        self.state += delta_time * sum;
        &self.state
    }
}
