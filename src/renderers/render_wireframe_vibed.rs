use crate::structures::*;

impl Scene {
    pub fn render_wireframe_vibed(&mut self) {
        let w = self.cam.window.width as f64;
        let h = self.cam.window.height as f64;
        let aspect = w / h;
        let f = self.cam.p;

        let mut lines = Vec::new();

        for obj in &self.objs {
            for tri in &obj.tri {
                let mut pts = [(0i32, 0i32); 3];
                let mut ok = true;

                for i in 0..3 {
                    // object -> world
                    let world = obj.rot * tri[i] + obj.pos;

                    // world -> camera
                    let cam_rel = world - self.cam.pos;
                    let cam_p = self.cam.rotinv * cam_rel;

                    // camera forward = +x
                    let z = cam_p[0];
                    let x = cam_p[1];
                    let y = cam_p[2];

                    if z <= 0.0 {
                        ok = false;
                        break;
                    }

                    let ndc_x = (x / z) * f / aspect;
                    let ndc_y = (y / z) * f;

                    let sx = ((ndc_x + 1.0) * 0.5 * w) as i32;
                    let sy = ((1.0 - (ndc_y + 1.0) * 0.5) * h) as i32;

                    pts[i] = (sx, sy);
                }

                if ok {
                    lines.push((pts[0], pts[1]));
                    lines.push((pts[1], pts[2]));
                    lines.push((pts[2], pts[0]));
                }
            }
        }

        for (a, b) in lines {
            self.plot_line([[a.0, a.1], [b.0, b.1]], (255, 255, 255));
        }
    }
}