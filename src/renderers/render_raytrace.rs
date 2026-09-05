use std::array::from_fn;

use crate::{linalg::Matrix, structures::*};


impl Scene {
    pub fn render_raytrace(&mut self) {

        let (w, h) = (self.cam.window.width, self.cam.window.height);
        let buffer = (0..h).map(|row| (0..w).map(|col| {

            for obj in &self.objs {
                for tri in &obj.tri {
                    
                }
            }


            (0, 0, 0)







        }).collect()).collect();

        self.cam.window.buffer = buffer;
    }
}