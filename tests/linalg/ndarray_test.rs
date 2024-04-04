use csl::linalg::ndarray::{Matrix, Vector};

#[test]
fn matrix_multiplication_test() {
    let a = Matrix {
        data: [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ],
    };

    let test_res_matrix = Matrix {
        data: [
            [90.0, 100.0, 110.0, 120.0],
            [202.0, 228.0, 254.0, 280.0],
            [314.0, 356.0, 398.0, 440.0],
            [426.0, 484.0, 542.0, 600.0],
        ],
    };

    let c = a * a.clone();

    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(c[(i, j)], test_res_matrix[(i, j)]);
        }
    }
}

#[test]
fn matrix_multiplication_difference_test() {
    let a = Matrix {
        data: [[3.0, 2.0, 1.0], [1.0, 0.0, 2.0]],
    };
    let b = Matrix {
        data: [[1.0, 2.0], [0.0, 1.0], [4.0, 0.0]],
    };

    let test_res_matrix = Matrix {
        data: [[7.0, 8.0], [9.0, 2.0]],
    };

    let c = a * b;

    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(c[(i, j)], test_res_matrix[(i, j)]);
        }
    }
}

#[test]
fn vector_multiplication_test() {
    let a = Matrix {
        data: [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ],
    };

    let out_vec = Vector {
        data: [30.0, 70.0, 110.0, 150.0],
    };

    let v = Vector {
        data: [1.0, 2.0, 3.0, 4.0],
    };

    let c = a * v;

    for i in 0..4 {
        assert_eq!(c.data[i], out_vec.data[i]);
    }
}
