use crate::linalg::*;

const EPS: f64 = 1e-10;

fn assert_close(a: f64, b: f64) {
    assert!(
        (a - b).abs() < EPS,
        "expected {b}, got {a}, difference {}",
        (a - b).abs()
    );
}

fn assert_matrix_close<const N: usize, const M: usize>(
    a: Matrix<N, M>,
    b: Matrix<N, M>,
) {
    for j in 0..M {
        for i in 0..N {
            assert_close(a.v[j][i], b.v[j][i]);
        }
    }
}

#[test]
fn test_lerp() {
    assert_close(lerp(0., 10., 0.), 0.);
    assert_close(lerp(0., 10., 0.5), 5.);
    assert_close(lerp(0., 10., 1.), 10.);
    assert_close(lerp(10., 20., 0.25), 12.5);
}

#[test]
fn test_construction_and_storage() {
    let m = Matrix::<3, 2>::from([
        [1., 2., 3.],
        [4., 5., 6.],
    ]);

    assert_eq!(m.v, [
        [1., 2., 3.],
        [4., 5., 6.],
    ]);
}

#[test]
fn test_from_cols() {
    let a = Vector::from_array([1., 2., 3.]);
    let b = Vector::from_array([4., 5., 6.]);
    let c = Vector::from_array([7., 8., 9.]);

    let m = Matrix::<3, 3>::from_cols([a, b, c]);

    assert_eq!(m.v, [
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.],
    ]);
}

#[test]
fn test_to_cols() {
    let m = Matrix::<3, 3>::from([
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.],
    ]);

    assert_eq!(m.to_cols(), [
        Vector::from_array([1., 2., 3.]),
        Vector::from_array([4., 5., 6.]),
        Vector::from_array([7., 8., 9.]),
    ]);
}

#[test]
fn test_to_rows() {
    let m = Matrix::<3, 3>::from([
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.],
    ]);

    assert_eq!(m.to_rows(), [
        Vector::from_array([1., 4., 7.]),
        Vector::from_array([2., 5., 8.]),
        Vector::from_array([3., 6., 9.]),
    ]);
}

#[test]
fn test_transpose() {
    let m = Matrix::<3, 2>::from([
        [1., 2., 3.],
        [4., 5., 6.],
    ]);

    let expected = Matrix::<2, 3>::from([
        [1., 4.],
        [2., 5.],
        [3., 6.],
    ]);

    assert_eq!(m.transpose(), expected);
    assert_eq!(m.transpose().transpose(), m);
}

#[test]
fn test_zero_and_identity() {
    assert_eq!(
        Matrix::<3, 2>::zero(),
        Matrix::from([
            [0., 0., 0.],
            [0., 0., 0.],
        ])
    );

    assert_eq!(
        Matrix::<3, 3>::identity(),
        Matrix::from([
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.],
        ])
    );
}

#[test]
fn test_hadamard() {
    let a = Matrix::<3, 2>::from([
        [1., 2., 3.],
        [4., 5., 6.],
    ]);

    let b = Matrix::<3, 2>::from([
        [2., 3., 4.],
        [5., 6., 7.],
    ]);

    assert_eq!(
        a.hadamard(b),
        Matrix::from([
            [2., 6., 12.],
            [20., 30., 42.],
        ])
    );
}

#[test]
fn test_apply_elementwise() {
    let mut m = Matrix::<3, 2>::from([
        [1., 2., 3.],
        [4., 5., 6.],
    ]);

    m.apply_elementwise(|x| *x *= 2.);

    assert_eq!(
        m,
        Matrix::from([
            [2., 4., 6.],
            [8., 10., 12.],
        ])
    );
}

#[test]
fn test_vector_conversion() {
    let v = Vector::from_array([1., 2., 3.]);

    assert_eq!(v.as_array(), [1., 2., 3.]);
}

#[test]
fn test_affine_transformation_vector() {
    let v = Vector::from_array([1., 2., 3.]);
    let affine = v.to_affine_transformation_vector::<4>();

    assert_eq!(
        affine,
        Matrix::from([
            [1., 2., 3., 1.],
        ])
    );
}

#[test]
fn test_dot() {
    let a = Vector::from_array([1., 2., 3.]);
    let b = Vector::from_array([4., 5., 6.]);

    assert_close(a.dot(&b), 32.);
}

#[test]
fn test_magnitude() {
    let v = Vector::from_array([3., 4.]);

    assert_close(v.magnitude(), 5.);
}

#[test]
fn test_normalized() {
    let v = Vector::from_array([3., 4.]);
    let n = v.normalized();

    assert_close(n[0], 0.6);
    assert_close(n[1], 0.8);
    assert_close(n.magnitude(), 1.);
}

#[test]
fn test_cross() {
    let x = Vector::from_array([1., 0., 0.]);
    let y = Vector::from_array([0., 1., 0.]);
    let z = Vector::from_array([0., 0., 1.]);

    assert_eq!(x.cross(&y), z);
    assert_eq!(y.cross(&z), x);
    assert_eq!(z.cross(&x), y);
}

#[test]
fn test_cross_anti_commutative() {
    let a = Vector::from_array([2., 3., 5.]);
    let b = Vector::from_array([7., 11., 13.]);

    assert_eq!(a.cross(&b), -b.cross(&a));
}

#[test]
fn test_scalar_multiplication() {
    let m = Matrix::<3, 2>::from([
        [1., 2., 3.],
        [4., 5., 6.],
    ]);

    let expected = Matrix::from([
        [2., 4., 6.],
        [8., 10., 12.],
    ]);

    assert_eq!(m * 2., expected);
    assert_eq!(&m * 2., expected);
    assert_eq!(2. * m, expected);
    assert_eq!(2. * &m, expected);

    let mut x = m;
    x *= 2.;
    assert_eq!(x, expected);
}

#[test]
fn test_scalar_addition() {
    let m = Matrix::<2, 2>::from([
        [1., 2.],
        [3., 4.],
    ]);

    let expected = Matrix::from([
        [6., 7.],
        [8., 9.],
    ]);

    assert_eq!(m + 5., expected);
    assert_eq!(&m + 5., expected);
    assert_eq!(5. + m, expected);
    assert_eq!(5. + &m, expected);

    let mut x = m;
    x += 5.;
    assert_eq!(x, expected);
}

#[test]
fn test_scalar_division() {
    let m = Matrix::<2, 2>::from([
        [2., 4.],
        [6., 8.],
    ]);

    let expected = Matrix::from([
        [1., 2.],
        [3., 4.],
    ]);

    assert_eq!(m / 2., expected);
    assert_eq!(&m / 2., expected);

    let mut x = m;
    x /= 2.;
    assert_eq!(x, expected);
}

#[test]
fn test_negation() {
    let m = Matrix::<2, 2>::from([
        [1., -2.],
        [3., -4.],
    ]);

    let expected = Matrix::from([
        [-1., 2.],
        [-3., 4.],
    ]);

    assert_eq!(-m, expected);
    assert_eq!(-&m, expected);
}

#[test]
fn test_matrix_multiplication() {
    let a = Matrix::<3, 2>::from([
        [1., 2., 3.],
        [4., 5., 6.],
    ]);

    let b = Matrix::<2, 3>::from([
        [7., 8.],
        [9., 10.],
        [11., 12.],
    ]);

    let expected = Matrix::<3, 3>::from([
        [39., 54., 69.],
        [49., 68., 87.],
        [59., 82., 105.],
    ]);

    assert_eq!(a * b, expected);
    assert_eq!(&a * b, expected);
    assert_eq!(a * &b, expected);
    assert_eq!(&a * &b, expected);
}

#[test]
fn test_matrix_multiplication_identity() {
    let a = Matrix::<3, 3>::from([
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 10.],
    ]);

    assert_eq!(a * Matrix::identity(), a);
    assert_eq!(Matrix::identity() * a, a);
}

#[test]
fn test_matrix_addition() {
    let a = Matrix::<2, 2>::from([
        [1., 2.],
        [3., 4.],
    ]);

    let b = Matrix::<2, 2>::from([
        [5., 6.],
        [7., 8.],
    ]);

    let expected = Matrix::from([
        [6., 8.],
        [10., 12.],
    ]);

    assert_eq!(a + b, expected);
    assert_eq!(&a + b, expected);
    assert_eq!(a + &b, expected);
    assert_eq!(&a + &b, expected);

    let mut x = a;
    x += b;
    assert_eq!(x, expected);
}

#[test]
fn test_matrix_subtraction() {
    let a = Matrix::<2, 2>::from([
        [1., 2.],
        [3., 4.],
    ]);

    let b = Matrix::<2, 2>::from([
        [5., 6.],
        [7., 8.],
    ]);

    let expected = Matrix::from([
        [-4., -4.],
        [-4., -4.],
    ]);

    assert_eq!(a - b, expected);
    assert_eq!(&a - b, expected);
    assert_eq!(a - &b, expected);
    assert_eq!(&a - &b, expected);

    let mut x = a;
    x -= b;
    assert_eq!(x, expected);
}

#[test]
fn test_det_2x2() {
    let m = Matrix::<2, 2>::from([
        [4., 7.],
        [2., 6.],
    ]);

    assert_close(m.det(), 10.);
}

#[test]
fn test_inverse_2x2() {
    let m = Matrix::<2, 2>::from([
        [4., 2.],
        [7., 6.],
    ]);

    let expected = Matrix::<2, 2>::from([
        [0.6, -0.2],
        [-0.7, 0.4],
    ]);

    assert_matrix_close(m.inverse().unwrap(), expected);
    assert_matrix_close(m.inverse_unchecked(), expected);
}

#[test]
fn test_inverse_2x2_identity() {
    let m = Matrix::<2, 2>::from([
        [4., 7.],
        [2., 6.],
    ]);

    let inv = m.inverse().unwrap();

    println!("m: {:?}\ninv: {:?}\nm * inv: {:?}\ninv * m: {:?}", m, inv, m * inv, inv * m);

    assert_matrix_close(m * inv, Matrix::identity());
    assert_matrix_close(inv * m, Matrix::identity());
}

#[test]
fn test_singular_2x2() {
    let m = Matrix::<2, 2>::from([
        [1., 2.],
        [2., 4.],
    ]);

    assert_close(m.det(), 0.);
    assert!(m.inverse().is_none());
}

#[test]
fn test_det_3x3() {
    let m = Matrix::<3, 3>::from([
        [6., 1., 1.],
        [4., -2., 5.],
        [2., 8., 7.],
    ]);

    assert_close(m.det(), -306.);
}

#[test]
fn test_inverse_3x3() {
    let m = Matrix::<3, 3>::from([
        [1., 2., 3.],
        [0., 1., 4.],
        [5., 6., 0.],
    ]);

    let expected = Matrix::<3, 3>::from([
        [-24., 18., 5.],
        [20., -15., -4.],
        [-5., 4., 1.],
    ]);

    assert_matrix_close(m.inverse().unwrap(), expected);
    assert_matrix_close(m.inverse_unchecked(), expected);
}

#[test]
fn test_inverse_3x3_identity() {
    let m = Matrix::<3, 3>::from([
        [1., 2., 3.],
        [0., 1., 4.],
        [5., 6., 0.],
    ]);

    let inv = m.inverse().unwrap();

    assert_matrix_close(m * inv, Matrix::identity());
    assert_matrix_close(inv * m, Matrix::identity());
}

#[test]
fn test_singular_3x3() {
    let m = Matrix::<3, 3>::from([
        [1., 2., 3.],
        [2., 4., 6.],
        [3., 6., 9.],
    ]);

    assert_close(m.det(), 0.);
    assert!(m.inverse().is_none());
}

#[test]
fn test_affine_translation_last() {
    let r = 2. * Matrix::<3, 3>::identity();
    let t = Vector::from_array([10., 20., 30.]);

    let a = r.to_affine_translate_last(t);

    let expected = Matrix::<4, 4>::from([
        [2.,  0.,  0.,  0.],
        [0.,  2.,  0.,  0.],
        [0.,  0.,  2.,  0.],
        [10., 20., 30., 1.],
    ]);

    assert_eq!(a, expected);
}

#[test]
fn test_affine_translation_first() {
    let r = 2. * Matrix::<3, 3>::identity();
    let t = Vector::from_array([10., 20., 30.]);

    let a = r.to_affine_translate_first(t);

    let expected = Matrix::<4, 4>::from([
        [2.,  0.,  0.,  0.],
        [0.,  2.,  0.,  0.],
        [0.,  0.,  2.,  0.],
        [20., 40., 60., 1.],
    ]);

    assert_eq!(a, expected);
}
