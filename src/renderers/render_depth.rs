use crate::structures::*;

impl Scene {
    pub fn render_depth(&mut self) {
        self.cam.window.buffer = self.cam.window.depth.iter().map(|row| row.iter().map(|j| (-j * 255.).clamp(0., 255.) as u8).map(|i| (i, i, i)).collect()).collect();
    }
}