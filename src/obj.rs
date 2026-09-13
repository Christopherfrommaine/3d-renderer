use std::array::from_fn;

use tobj;
use crate::linalg::*;

#[derive(Clone, Debug)]
pub struct Object {
    pub tri: Vec<[Vector<3>; 3]>,
    pub bounding: [Vector<3>; 2],
    pub rot: Matrix<3, 3>,
    pub pos: Vector<3>,
    pub mat: Material,
}

pub fn get_model(filename: &str) -> Vec<[[f64; 3]; 3]> {
    let (models, _) = tobj::load_obj(String::from("obj/") + filename, &tobj::LoadOptions::default()).unwrap();
    let mesh = &models[0].mesh;
    let positions = &mesh.positions;
    let indices = &mesh.indices;

    let triangles: Vec<[[f64; 3]; 3]> = indices
        .chunks(3)
        .map(|i| [
            [
                positions[3 * i[0] as usize] as f64,
                positions[3 * i[0] as usize + 1] as f64,
                positions[3 * i[0] as usize + 2] as f64,
            ],
            [
                positions[3 * i[1] as usize] as f64,
                positions[3 * i[1] as usize + 1] as f64,
                positions[3 * i[1] as usize + 2] as f64,
            ],
            [
                positions[3 * i[2] as usize] as f64,
                positions[3 * i[2] as usize + 1] as f64,
                positions[3 * i[2] as usize + 2] as f64,
            ],
        ])
        .collect();

    triangles
}


impl Object {
    pub fn from(tri: Vec<[Vector<3>; 3]>, rot: Matrix<3, 3>, pos: Vector<3>, mat: Material) -> Self {
        let mut bounding = [Vector::from_array([f64::INFINITY; 3]), Vector::from_array([0.; 3])];
        
        for v in tri.iter().copied() {
            for point in v {
                bounding[0][0] = bounding[0][0].min(point[0]);
                bounding[0][1] = bounding[0][1].min(point[1]);
                bounding[0][2] = bounding[0][2].min(point[2]);

                bounding[1][0] = bounding[1][0].max(point[0]);
                bounding[1][1] = bounding[1][1].max(point[1]);
                bounding[1][2] = bounding[1][2].max(point[2]);
            }
        }

        Object { tri, bounding, rot, pos, mat }

    }

    pub fn from_array(tri: Vec<[[f64; 3]; 3]>, rot: Matrix<3, 3>, pos: Vector<3>, mat: Material) -> Self {
        Self::from(
            tri.into_iter().map(|v| std::array::from_fn(|i| Vector::from_array(v[i]))).collect(),
            rot, pos, mat
        )
    }

    fn from_array_default(tri: Vec<[[f64; 3]; 3]>) -> Self {
        Self::from_array(tri, Matrix::identity(), Matrix::zero(), Material::default())
    }

    pub fn scale(&mut self, x: f64) {
        for tri in self.tri.iter_mut() {
            for point in tri {
                *point = x * *point;
            }
        }
    }

    pub fn scale_xyz(&mut self, xyz: Vector<3>) {
        for tri in self.tri.iter_mut() {
            for point in tri {
                *point = (*point).hadamard(xyz);
            }
        }
    }


    pub fn cube() -> Self {     Self::from_array_default(crate::obj::get_model("cube-tex.obj")) }
    pub fn sphere() -> Self {   Self::from_array_default(crate::obj::get_model("icosphere.obj")) }
    pub fn triangle() -> Self { Self::from_array_default(crate::obj::get_model("triangle.obj")) }
    pub fn teapot() -> Self { Self::from_array_default(crate::obj::get_model("teapot.obj")) }
    pub fn dragon() -> Self { Self::from_array_default(crate::obj::get_model("dragon.obj")) }

}

#[derive(Clone, Debug)]
pub struct Material {
    pub color: [f64; 3],
}

impl Material {
    pub fn from(color: [f64; 3]) -> Self {
        Material { color }
    }

    pub fn from_u8(color: [u8; 3]) -> Self {
        Self::from(from_fn(|i| color[i] as f64 / 255.))
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::from([1., 1., 1.])
    }
}
