use csl::{diffeq::ivp::RungeKutta4, linalg::ndarray::Vector};

fn main() {
    let initial_state = Vector { data: [[100.0]] };
    let mut solver = RungeKutta4::new(0.0, initial_state);

    let delta_time = 0.1;

    println!("{:>5.2}s: {:>7.3}°C", 0.0, initial_state[0]);
    for time in (0..=100).into_iter().map(|i| i as f32 * delta_time) {
        let curr_state = solver.next_step(
            &|_time, curr_t| -0.07 * (curr_t + -1.0 * Vector { data: [[20.0]] }),
            delta_time,
        );

        println!("{:>5.2}s: {:>7.3}°C", time, curr_state[0]);
    }
}
