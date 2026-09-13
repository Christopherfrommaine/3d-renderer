use std::array::from_fn;

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

#[inline(always)]
unsafe fn bitcast<T, U>(inp: T) -> U {
    unsafe { std::ptr::read(&inp as *const _ as *const U) }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix<const N: usize, const M: usize> {
    pub v: [[f64; N]; M],
}

// // Matrix Operations

// Matrix Methods
impl<const M: usize, const N: usize> Matrix<N, M> {
    pub fn from(v: [[f64; N]; M]) -> Self {
        Matrix { v }
    }
    
    pub fn from_cols(cols: [Matrix<N, 1>; M]) -> Self {
        unsafe { bitcast(cols) }
    }

    pub fn transpose(self) -> Matrix<M, N> {
        let mut result = [[0.; M]; N];
        for i in 0..M {
            for j in 0..N {
                result[j][i] = self.v[i][j];
            }
        }
        Matrix { v: result }
    }

    pub fn zero() -> Self {
        Matrix { v: [[0.; N]; M] }
    }

    pub fn identity() -> Self {
        let mut result = [[0.; N]; M];
        for i in 0..(N.min(M)) {
            result[i][i] = 1.;
        }
        Matrix { v: result }
    }

    pub fn hadamard(&self, other: Self) -> Self {
        Matrix { v: from_fn(|i| from_fn(|j| self.v[i][j] * other.v[i][j])) }
    }

    pub fn apply_elementwise(&mut self, f: fn(&mut f64)) {
        self.v.iter_mut().for_each(|col| col.iter_mut().for_each(|elem| f(elem)));
    }

    pub fn col_vectors(self) -> [Matrix<N, 1>; M] {
        unsafe { bitcast(self) }
    }

    pub fn row_vectors(self) -> [Vector<M>; N] {
        // In memory, a 3x3 matrix should be stored like
        // [[a, d, g], [b, e, h], [c, f, i]]
        // But we want to get out:
        // [[[a], [b], [c]], [[d], [e], [f]], [[g], [h], [i]]]
        // Which is exactly the same memory layout as self.transpose.
        
        self.transpose().col_vectors()
    }
}


pub type Vector<const N: usize> = Matrix<N, 1>;
impl<const N: usize> Vector<N> {
    pub fn from_array(v: [f64; N]) -> Self {
        Matrix { v: [v] }
    }

    pub fn to_array(self) -> [f64; N] {
        // seems useless, but nice to distinguish for M == 1
        self.v[0]
    }

    pub fn to_affine_transformation_vector<const K: usize>(&self) -> Matrix<K, 1> {
        const { assert!(N + 1 == K); }
        let mut v = [0.; K];
        v[..N].copy_from_slice(&self.v[0]);
        v[N] = 1.;

        Matrix { v: [v] }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        (0..N).map(|i| self[i] * other[i]).sum()
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.dot(self))
    }

    pub fn normalized(self) -> Self {
        self / self.magnitude()
    }
}
impl Vector<3> {
    pub fn cross(&self, other: Self) -> Self {
        Vector::from_array([
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }
}


// Matrix Math
impl Matrix<2, 2> {
    pub fn det(&self) -> f64 {
        let a = self.v[0][0];
        let b = self.v[1][0];
        let c = self.v[0][1];
        let d = self.v[1][1];
        a * d - b * c
    }

    pub fn inverse(&self) -> Option<Matrix<2, 2>> {
        let det = self.det();
        if det.abs() < 1e-12 { return None; }
        Some(self.inverse_with_det(det))
    }

    pub fn inverse_unchecked(&self) -> Self {
        self.inverse_with_det(self.det())
    }

    #[inline(always)]
    fn inverse_with_det(&self, det: f64) -> Self {
        Matrix::from([
            [ self.v[1][1], -self.v[1][0]],
            [-self.v[0][1],  self.v[0][0]],
        ]) / det
    }
}
impl Matrix<3, 3> {
    pub fn det(&self) -> f64 {
        let v = &self.v;
        v[0][0] * (v[1][1] * v[2][2] - v[2][1] * v[1][2])
            - v[1][0] * (v[0][1] * v[2][2] - v[2][1] * v[0][2])
            + v[2][0] * (v[0][1] * v[1][2] - v[1][1] * v[0][2])
    }

    pub fn inverse(&self) -> Option<Matrix<3, 3>> {
        let det = self.det();
        if det.abs() < 1e-12 { return None; }
        Some(self.inverse_with_det(det))
    }

    pub fn inverse_unchecked(&self) -> Self {
        self.inverse_with_det(self.det())
    }

    #[inline(always)]
    fn inverse_with_det(&self, det: f64) -> Self {
        let v = &self.v;
        Matrix::from([
            [
                 (v[1][1] * v[2][2] - v[2][1] * v[1][2]),
                -(v[0][1] * v[2][2] - v[2][1] * v[0][2]),
                 (v[0][1] * v[1][2] - v[1][1] * v[0][2]),
            ],
            [
                -(v[1][0] * v[2][2] - v[2][0] * v[1][2]),
                 (v[0][0] * v[2][2] - v[2][0] * v[0][2]),
                -(v[0][0] * v[1][2] - v[1][0] * v[0][2]),
            ],
            [
                 (v[1][0] * v[2][1] - v[2][0] * v[1][1]),
                -(v[0][0] * v[2][1] - v[2][0] * v[0][1]),
                 (v[0][0] * v[1][1] - v[1][0] * v[0][1]),
            ],
        ]) / det
    }

    pub fn rotation_matrix(yaw: f64, roll: f64, pitch_orig: f64) -> Self {
        let pitch = pitch_orig + core::f64::consts::PI;

        let rx = Matrix::from([[1., 0., 0.], [0., pitch.cos(), -pitch.sin()], [0., pitch.sin(), pitch.cos()]]);
        let ry = Matrix::from([[yaw.cos(), 0., yaw.sin()], [0., 1., 0.], [-yaw.sin(), 0., yaw.cos()]]);
        let rz = Matrix::from([[roll.cos(), -roll.sin(), 0.], [roll.sin(), roll.cos(), 0.], [0., 0., 1.]]);

        // originally in row-major, so this is a fix that should be "good enough"
        (rz * ry * rx).transpose()
        
    }
    
    pub fn to_affine_translate_last(&self, vec: Matrix<3, 1>) -> Matrix<4, 4> {
        // constructs:
        // [[a11, a12, a13, v1]
        //  [a21, a22, a23, v2]
        //  [a31, a32, a33, v3]
        //  [0.,  0.,  0.,  1.]]

        let mut v = [[0., 0., 0., 1.]; 4];
        for i in 0..3 {
            v[i][..3].copy_from_slice(&self.v[i]);
            v[i][4] = vec[i];
        }
        Matrix::from(v)
    }

    pub fn to_affine_translate_first(&self, vec: Matrix<3, 1>) -> Matrix<4, 4> {
        self.to_affine_translate_last(Matrix::zero()) * Matrix::identity().to_affine_translate_last(vec)
    }
}

// Indexing
impl<const N: usize> std::ops::Index<usize> for Vector<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[0][index]
    }
}
impl<const N: usize> std::ops::IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[0][index]
    }
}

// Scalar Ops
use std::ops::{Mul, MulAssign, Add, AddAssign, Sub, SubAssign, Div, DivAssign, Neg};

impl<const N: usize, const M: usize> Matrix<N, M> {
    fn mul_private(&mut self, rhs: f64) {
        for row in &mut self.v {
            for x in row {
                *x *= rhs;
            }
        }
    }

    fn add_private(&mut self, rhs: f64) {
        for row in &mut self.v {
            for x in row {
                *x += rhs;
            }
        }
    }

    fn sub_private(&mut self, rhs: f64) {
        for row in &mut self.v {
            for x in row {
                *x -= rhs;
            }
        }
    }

    fn div_private(&mut self, rhs: f64) {
        let recip = rhs.recip();
        for row in &mut self.v {
            for x in row {
                *x = recip;
            }
        }
    }

    fn neg_private(&mut self) {
        for row in &mut self.v {
            for x in row {
                *x = -*x;
            }
        }
    }
}

impl<const N: usize, const M: usize> Mul<f64> for Matrix<N, M> {
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self::Output { self.mul_private(rhs); self }
}
impl<const N: usize, const M: usize> Mul<f64> for &Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn mul(self, rhs: f64) -> Self::Output { let mut result = *self; result.mul_private(rhs); result }
}
impl<const N: usize, const M: usize> Mul<Matrix<N, M>> for f64 {
    type Output = Matrix<N, M>;
    fn mul(self, mut rhs: Matrix<N, M>) -> Self::Output { rhs.mul_private(self); rhs }
}
impl<const N: usize, const M: usize> Mul<&Matrix<N, M>> for f64 {
    type Output = Matrix<N, M>;
    fn mul(self, rhs: &Matrix<N, M>) -> Self::Output { let mut result = *rhs; result.mul_private(self); result }
}
impl<const N: usize, const M: usize> MulAssign<f64> for Matrix<N, M> {
    fn mul_assign(&mut self, rhs: f64) { self.mul_private(rhs); }
}

impl<const N: usize, const M: usize> Add<f64> for Matrix<N, M> {
    type Output = Self;
    fn add(mut self, rhs: f64) -> Self::Output { self.add_private(rhs); self }
}
impl<const N: usize, const M: usize> Add<f64> for &Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn add(self, rhs: f64) -> Self::Output { let mut result = *self; result.add_private(rhs); result }
}
impl<const N: usize, const M: usize> Add<Matrix<N, M>> for f64 {
    type Output = Matrix<N, M>;
    fn add(self, mut rhs: Matrix<N, M>) -> Self::Output { rhs.add_private(self); rhs }
}
impl<const N: usize, const M: usize> Add<&Matrix<N, M>> for f64 {
    type Output = Matrix<N, M>;
    fn add(self, rhs: &Matrix<N, M>) -> Self::Output { let mut result = *rhs; result.add_private(self); result }
}
impl<const N: usize, const M: usize> AddAssign<f64> for Matrix<N, M> {
    fn add_assign(&mut self, rhs: f64) { self.add_private(rhs); }
}

impl<const N: usize, const M: usize> Sub<f64> for Matrix<N, M> {
    type Output = Self;
    fn sub(mut self, rhs: f64) -> Self::Output { self.sub_private(rhs); self }
}
impl<const N: usize, const M: usize> Sub<f64> for &Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn sub(self, rhs: f64) -> Self::Output { let mut result = *self; result.sub_private(rhs); result }
}
impl<const N: usize, const M: usize> Sub<Matrix<N, M>> for f64 {
    type Output = Matrix<N, M>;
    fn sub(self, mut rhs: Matrix<N, M>) -> Self::Output {
        for row in &mut rhs.v {
            for x in row {
                *x = self - *x;
            }
        }
        rhs
    }
}
impl<const N: usize, const M: usize> Sub<&Matrix<N, M>> for f64 {
    type Output = Matrix<N, M>;
    fn sub(self, rhs: &Matrix<N, M>) -> Self::Output {
        let mut result = *rhs;
        for row in &mut result.v {
            for x in row {
                *x = self - *x;
            }
        }
        result
    }
}
impl<const N: usize, const M: usize> SubAssign<f64> for Matrix<N, M> {
    fn sub_assign(&mut self, rhs: f64) { self.sub_private(rhs); }
}

impl<const N: usize, const M: usize> Div<f64> for Matrix<N, M> {
    type Output = Self;
    fn div(mut self, rhs: f64) -> Self::Output { self.div_private(rhs); self }
}
impl<const N: usize, const M: usize> Div<f64> for &Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn div(self, rhs: f64) -> Self::Output { let mut result = *self; result.div_private(rhs); result }
}
impl<const N: usize, const M: usize> DivAssign<f64> for Matrix<N, M> {
    fn div_assign(&mut self, rhs: f64) { self.div_private(rhs); }
}

impl<const N: usize, const M: usize> Neg for Matrix<N, M> {
    type Output = Self;
    fn neg(mut self) -> Self::Output { self.neg_private(); self }
}
impl<const N: usize, const M: usize> Neg for &Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn neg(self) -> Self::Output { let mut result = *self; result.neg_private(); result }
}

// Matrix Ops
impl<const N: usize, const M: usize> Matrix<N, M> {
    #[inline(always)]
    fn mul_private_mat<const K: usize>(&self, rhs: &Matrix<M, K>) -> Matrix<N, K> {
        let mut result = [[0.; N]; K];
        for j in 0..K {
            for k in 0..M {
                let b = rhs.v[j][k];
                for i in 0..N {
                    result[j][i] += self.v[k][i] * b;
                }
            }
        }
        Matrix { v: result }
    }

    #[inline(always)]
    fn add_private_mat(&mut self, rhs: &Self) {
        for j in 0..M {
            for i in 0..N {
                self.v[j][i] += rhs.v[j][i];
            }
        }
    }

    #[inline(always)]
    fn sub_private_mat(&mut self, rhs: &Self) {
        for j in 0..M {
            for i in 0..N {
                self.v[j][i] -= rhs.v[j][i];
            }
        }
    }
}

impl<const N: usize, const M: usize, const K: usize> Mul<Matrix<M, K>> for Matrix<N, M> {
    type Output = Matrix<N, K>;
    fn mul(self, rhs: Matrix<M, K>) -> Self::Output { self.mul_private_mat(&rhs) }
}
impl<const N: usize, const M: usize, const K: usize> Mul<Matrix<M, K>> for &Matrix<N, M> {
    type Output = Matrix<N, K>;
    fn mul(self, rhs: Matrix<M, K>) -> Self::Output { self.mul_private_mat(&rhs) }
}
impl<const N: usize, const M: usize, const K: usize> Mul<&Matrix<M, K>> for Matrix<N, M> {
    type Output = Matrix<N, K>;
    fn mul(self, rhs: &Matrix<M, K>) -> Self::Output { self.mul_private_mat(rhs) }
}
impl<const N: usize, const M: usize, const K: usize> Mul<&Matrix<M, K>> for &Matrix<N, M> {
    type Output = Matrix<N, K>;
    fn mul(self, rhs: &Matrix<M, K>) -> Self::Output { self.mul_private_mat(rhs) }
}

impl<const N: usize, const M: usize> Add for Matrix<N, M> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output { self.add_private_mat(&rhs); self }
}
impl<const N: usize, const M: usize> Add<&Matrix<N, M>> for Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn add(mut self, rhs: &Matrix<N, M>) -> Self::Output { self.add_private_mat(rhs); self }
}
impl<const N: usize, const M: usize> Add<Matrix<N, M>> for &Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn add(self, rhs: Matrix<N, M>) -> Self::Output { let mut result = *self; result.add_private_mat(&rhs); result }
}
impl<const N: usize, const M: usize> Add<&Matrix<N, M>> for &Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn add(self, rhs: &Matrix<N, M>) -> Self::Output { let mut result = *self; result.add_private_mat(rhs); result }
}
impl<const N: usize, const M: usize> AddAssign for Matrix<N, M> {
    fn add_assign(&mut self, rhs: Self) { self.add_private_mat(&rhs); }
}

impl<const N: usize, const M: usize> Sub for Matrix<N, M> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output { self.sub_private_mat(&rhs); self }
}
impl<const N: usize, const M: usize> Sub<&Matrix<N, M>> for Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn sub(mut self, rhs: &Matrix<N, M>) -> Self::Output { self.sub_private_mat(rhs); self }
}
impl<const N: usize, const M: usize> Sub<Matrix<N, M>> for &Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn sub(self, rhs: Matrix<N, M>) -> Self::Output { let mut result = *self; result.sub_private_mat(&rhs); result }
}
impl<const N: usize, const M: usize> Sub<&Matrix<N, M>> for &Matrix<N, M> {
    type Output = Matrix<N, M>;
    fn sub(self, rhs: &Matrix<N, M>) -> Self::Output { let mut result = *self; result.sub_private_mat(rhs); result }
}
impl<const N: usize, const M: usize> SubAssign for Matrix<N, M> {
    fn sub_assign(&mut self, rhs: Self) { self.sub_private_mat(&rhs); }
}