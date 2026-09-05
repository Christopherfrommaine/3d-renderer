use std::{array::from_fn, debug_assert_eq};

#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<const N: usize, const M: usize> {
    pub v: [[f64; M]; N],
}

impl<const N: usize, const M: usize> std::fmt::Debug for Matrix<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (N, M) {
            (1, 1) => f.debug_struct("Scalar").field("v", &self.v[0][0]).finish(),
            (_, 1) => f.debug_struct("Vector").field("v", &self.v.iter().map(|r| r[0]).collect::<Vec<f64>>()).finish(),
            (1, _) => f.debug_struct("RowVector").field("v", &self.v[0]).finish(),
            (_, _) => f.debug_struct("Matrix").field("v", &self.v).finish(),
        }
    }
}

impl<const M: usize, const N: usize> Matrix<N, M> {
    pub fn from(v: [[f64; M]; N]) -> Self {
        Matrix { v }
    }

    pub fn transpose(self) -> Matrix<M, N> {
        let mut result = [[0.; N]; M];

        for i in 0..N {
            for j in 0..M {
                result[j][i] = self.v[i][j];
            }
        }

        Matrix { v: result }
    }

    pub fn identity() -> Self {
        let mut result = [[0.; M]; N];
        for i in 0..(N.min(M)) {
            result[i][i] = 1.;
        }
        Matrix { v: result }
    }

    pub fn zero() -> Self {
        Matrix { v: [[0.; M]; N] }
    }
}

pub type Vector<const N: usize> = Matrix<N, 1>;
impl<const N: usize> Vector<N> {
    pub fn from_array(v: [f64; N]) -> Self {
        let mut result = [[0.]; N];
        for i in 0..N {
            result[i] = [v[i]];
        }
        Matrix { v: result }
    }

    pub fn as_array(&self) -> [f64; N] {
        from_fn(|i| self[i])
    }

    pub fn to_affine_transformation_vector<const K: usize>(&self) -> Matrix<K, 1> {
        debug_assert_eq!(N + 1, K);
        Matrix { v: from_fn(|i| [if i == N {1.} else {self[i]}])}
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.v.iter().map(|v| v[0] * v[0]).sum())
    }

    pub fn normalized(self) -> Self {
        (1. / self.magnitude()) * self
    }

    pub fn dot(&self, other: Self) -> f64 {
        (0..N).map(|i| self[i] * other[i]).sum()
    }
}
impl<const N: usize> std::ops::Index<usize> for Vector<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index][0]
    }
}
impl<const N: usize> std::ops::IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index][0]
    }
}

impl<const N: usize, const M: usize> std::ops::Mul<f64> for Matrix<N, M> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self.v;
        for i in 0..N {
            for j in 0..M {
                result[i][j] *= rhs;
            }
        }
        Matrix {v: result}
    }
}
impl<const N: usize, const M: usize> std::ops::Mul<Matrix<N, M>> for f64 {
    type Output = Matrix<N, M>;
    fn mul(self, rhs: Matrix<N, M>) -> Self::Output { rhs * self }
}

impl<const N: usize, const M: usize> std::ops::Add<f64> for Matrix<N, M> {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        let mut result = self.v;
        for i in 0..N {
            for j in 0..M {
                result[i][j] += rhs;
            }
        }
        Matrix {v: result}
    }
}
impl<const N: usize, const M: usize> std::ops::Add<Matrix<N, M>> for f64 {
    type Output = Matrix<N, M>;
    fn add(self, rhs: Matrix<N, M>) -> Self::Output { rhs + self }
}

impl<const N: usize, const M: usize, const K: usize> std::ops::Mul<Matrix<M, K>> for Matrix<N, M> {
    type Output = Matrix<N, K>;

    fn mul(self, rhs: Matrix<M, K>) -> Self::Output {
        let mut result = [[0.; K]; N];
        for i in 0..N {
            for j in 0..K {
                for n in 0..M {
                    result[i][j] += self.v[i][n] * rhs.v[n][j];
                }
            }
        }
        Matrix {v: result}
    }
}

impl<const N: usize, const M: usize> std::ops::Add<Matrix<N, M>> for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn add(self, rhs: Matrix<N, M>) -> Self::Output {
        let mut result = self;
        for i in 0..N {
            for j in 0..M {
                result.v[i][j] += rhs.v[i][j];
            }
        }
        result
    }
}

impl<const N: usize, const M: usize> std::ops::Sub<Matrix<N, M>> for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn sub(self, rhs: Matrix<N, M>) -> Self::Output {
        let mut result = self;
        for i in 0..N {
            for j in 0..M {
                result.v[i][j] -= rhs.v[i][j];
            }
        }
        result
    }
}

impl Matrix<2, 2> {
    pub fn det(&self) -> f64 {
        let a = self.v[0][0];
        let b = self.v[0][1];
        let c = self.v[1][0];
        let d = self.v[1][1];
        a * d - b * c
    }

    pub fn inverse(&self) -> Option<Matrix<2, 2>> {
        let det = self.det();
        if det.abs() < 1e-12 { return None; }
        Some(Matrix {
            v: [
                [ self.v[1][1] / det, -self.v[0][1] / det],
                [-self.v[1][0] / det,  self.v[0][0] / det],
            ]
        })
    }
}

impl Matrix<3, 3> {
    pub fn det(&self) -> f64 {
        let m = &self.v;
        m[0][0]*(m[1][1]*m[2][2] - m[1][2]*m[2][1])
        - m[0][1]*(m[1][0]*m[2][2] - m[1][2]*m[2][0])
        + m[0][2]*(m[1][0]*m[2][1] - m[1][1]*m[2][0])
    }

    pub fn inverse(&self) -> Option<Matrix<3, 3>> {
        let det = self.det();
        if det.abs() < 1e-12 { return None; }
        let m = &self.v;

        let mut inv = [[0.; 3]; 3];

        inv[0][0] =  (m[1][1]*m[2][2] - m[1][2]*m[2][1]) / det;
        inv[0][1] = -(m[0][1]*m[2][2] - m[0][2]*m[2][1]) / det;
        inv[0][2] =  (m[0][1]*m[1][2] - m[0][2]*m[1][1]) / det;

        inv[1][0] = -(m[1][0]*m[2][2] - m[1][2]*m[2][0]) / det;
        inv[1][1] =  (m[0][0]*m[2][2] - m[0][2]*m[2][0]) / det;
        inv[1][2] = -(m[0][0]*m[1][2] - m[0][2]*m[1][0]) / det;

        inv[2][0] =  (m[1][0]*m[2][1] - m[1][1]*m[2][0]) / det;
        inv[2][1] = -(m[0][0]*m[2][1] - m[0][1]*m[2][0]) / det;
        inv[2][2] =  (m[0][0]*m[1][1] - m[0][1]*m[1][0]) / det;

        Some(Matrix { v: inv })
    }

    pub fn rotation_matrix(pitch: f64, yaw: f64, roll: f64) -> Self {
        let rx = Matrix::from([[1., 0., 0.], [0., roll.cos(), -roll.sin()], [0., roll.sin(), roll.cos()]]);
        let ry = Matrix::from([[pitch.cos(), 0., pitch.sin()], [0., 1., 0.], [-pitch.sin(), 0., pitch.cos()]]);
        let rz = Matrix::from([[yaw.cos(), -yaw.sin(), 0.], [yaw.sin(), yaw.cos(), 0.], [0., 0., 1.]]);

        rz * ry * rx
        
    }
    
    pub fn to_affine_translate_last(&self, vec: Matrix<3, 1>) -> Matrix<4, 4> {
        Matrix::from([
            [self.v[0][0], self.v[0][1], self.v[0][2], vec[0]],
            [self.v[1][0], self.v[1][1], self.v[1][2], vec[1]],
            [self.v[2][0], self.v[2][1], self.v[2][2], vec[2]],
            [0., 0., 0., 1.],
        ])
    }

    pub fn to_affine_translate_first(&self, vec: Matrix<3, 1>) -> Matrix<4, 4> {
        self.to_affine_translate_last(Matrix::zero()) * Matrix::identity().to_affine_translate_last(vec)
    }
}

impl Matrix<3, 1> {
    pub fn cross(&self, other: Self) -> Self {
        Matrix::from_array([
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }
}










pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}