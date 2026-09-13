use std::{debug_assert, debug_assert_eq, array::from_fn};

use crate::{linalg::*, obj::*, structures::*};
use rayon::prelude::*;

/// length of component of u in the direction of v
fn scal<const N: usize>(u: Vector<N>, v: Vector<N>) -> f64 {
    u.dot(v) / v.magnitude()
}

/// component of u in the direction of v
fn proj<const N: usize>(u: Vector<N>, v: Vector<N>) -> Vector<N> {
    (u.dot(v) / v.dot(v)) * v
}

#[derive(Clone, Copy, Debug)]
struct Ray {
    pos: Vector<3>,
    dir: Vector<3>,
}

impl Ray {
    fn trace(&self, objs: &[Object]) -> Option<(Ray, Material)> {
        // let mut min_t = core::f64::INFINITY;
        // let mut min_tri = from_fn(|_| Vector::zero());

        for obj in objs {
            // bounding box check
            // TODO

            // triangle check
            for tri in obj.tri.iter().copied() {
                
                let offset = obj.pos - self.pos;
                
                let [p1, p2, p3] = from_fn(|i| obj.rot * tri[i] + offset);  
                let m = Matrix::from(from_fn(|i| (obj.rot * tri[i] + offset).as_array()));
                
                if let Some(m_inv) = m.inverse() {
                    let abc = m_inv * Vector::from_array([1., 1., 1.]);
                    let t = abc.dot(self.dir).recip();
                    if !t.is_finite() { continue; }

                    let ip = t * self.dir;  // intersection point relative to the ray origin
                    

                }
            }
        }

        Some((self.clone(), Material::default()))
    }
}


impl Scene {
    pub fn render_raytrace(&mut self) {
        let (w, h) = (self.cam.window.width, self.cam.window.height);
        let objs = self.objs.clone();
        let buffer: Vec<Vec<(u8, u8, u8)>> = (0..h).into_par_iter().map(|row| { (0..w).map(|col| {
            
            let camvec = Vector::from_array([(col as f64 - 0.5 * w as f64) / w as f64, (row as f64 - 0.5 * h as f64) / w as f64, self.cam.p]);
            for obj in objs.iter() {
                for tri in obj.tri.iter().copied() {
                    let offset = obj.pos - self.cam.pos;
                    let tri_m = Matrix::from(from_fn);
                    // offset.apply_elementwise(|x| *x += rand::random::<f64>() * 0.0001);
                    
                    let [p0, p1, p2] = std::array::from_fn(|i| (obj.rot * tri[i]) + offset);
                    
                    let u = p1 - p0;
                    let v = p2 - p0;
                    let w = u.cross(v);

                    let m = Matrix::from([p0.as_array(), p1.as_array(), p2.as_array()]);

                    if let Some(mi) = m.inverse() {
                        let abc = mi * Vector::from_array([1., 1., 1.]);
                        let t = 1. / abc.dot(camvec);
                        if t.is_infinite() || t.is_nan() { break; }

                        let ip = t * camvec;
                        let ip_rel = ip - p0;

                        let uv_m = Matrix::from([u.as_array(), v.as_array(), w.as_array()]).transpose();
                        if let Some(uv_mi) = uv_m.inverse() {
                            let [uc, vc, wc] = (uv_mi * ip_rel).as_array();

                            // log::trace!("{uc} {vc} {wc} {ip_rel:?}");
                            debug_assert!((ip_rel - (uc * u + vc * v + wc * w)).magnitude() < 0.001, "vector unable to be reconstructed: {:?}, {:?} (t={t})", ip_rel, uc * u + vc * v + wc * w);
                            debug_assert!(wc < 1., "intersection is not on the triangle plane: {wc}, {t}, {ip:?}, {ip_rel:?}");

                            if uc > 0. && vc > 0. && uc + vc < 1. {
                                return obj.mat.color;
                            }
                        } else {
                            log::trace!("matrix uv_m is singular")
                        }
                    } else {
                        log::trace!("matrix m is singular")
                    }
                }
            }

            return [0., 0., 0.];
        }).map(|[r, g, b]| (
            (255. * r.clamp(0., 1.)) as u8,
            (255. * g.clamp(0., 1.)) as u8,
            (255. * b.clamp(0., 1.)) as u8)
        ).collect() }).collect();

        self.cam.window.buffer = buffer;
    }
}
