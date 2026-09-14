use core::f64;
use crate::linalg::*;
use crate::window::Window;
use crate::obj::Object;

#[derive(Debug)]
pub struct Scene {
    pub objs: Vec<Object>,
    pub cam: Camera,
}

#[derive(Debug)]
pub struct Camera {
    pub pos: Vector<3>,
    pub rot: Matrix<3, 3>,
    pub rotinv: Matrix<3, 3>,
    pub p: f64,
    pub window: Window,
}

impl Camera {
    pub fn from(pos: Vector<3>, rot: Matrix<3, 3>, rotinv: Matrix<3, 3>, p: f64, window: Window) -> Self {
        Camera { pos, rot, rotinv, p, window }
    }

    pub fn with_window(window: Window) -> Self {
        Self::from(Vector::from_array([0., 0., -5.]), Matrix::identity(), Matrix::identity(), 45. * std::f64::consts::PI / 180., window)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::with_window(crate::window::Window::default())
    }
}

impl Scene {
    pub fn new(objs: Vec<Object>, cam: Camera) -> Self {
        Scene { objs, cam }
    }

    pub fn plot(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        debug_assert!(x < self.cam.window.width);
        debug_assert!(y < self.cam.window.height);

        self.cam.window.buffer[y][x] = color;
    }

    pub fn plot_in_bounds(&mut self, x: i32, y: i32, color: (u8, u8, u8)) {
        if self.cam.window.in_bounds(x, y) {
            self.plot(x as usize, y as usize, color);
        }
    }
    
    pub fn plot_line(&mut self, points: [[i32; 2]; 2], color: (u8, u8, u8)) {
        log::trace!("points: {points:?}");

        let (x0, x1, y0, y1) = (points[0][0],  points[1][0], points[0][1], points[1][1]);

        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.plot_in_bounds(x, y, color);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn plot_triangle(&mut self, points: [(f64, f64, f64); 3], color: [f64; 3]) {
        log::trace!("points: {points:?}");
        let color_u8 = ((color[0].clamp(0., 1.) * 255.) as u8, (color[1].clamp(0., 1.) * 255.) as u8, (color[2].clamp(0., 1.) * 255.) as u8);

        // todo: skip lines outside of the frame

        let mut pts = points;
        pts.sort_unstable_by(|p1, p2| p1.1.partial_cmp(&p2.1).unwrap_or(std::cmp::Ordering::Equal));

        let (x0, y0, d0) = pts[0];
        let (x1, y1, d1) = pts[1];
        let (x2, y2, d2) = pts[2];

        let (w, h) = (self.cam.window.width as f64, self.cam.window.height as f64);
        if x0.is_nan() || x1.is_nan() || x2.is_nan() || y0.is_nan() || y1.is_nan() || y2.is_nan() || d0.is_nan() || d1.is_nan() || d2.is_nan() { return; }
        if (x0 < 0. && x1 < 0. && x2 < 0.) ||
           (x0 > w  && x1 > w  && x2 > w ) ||
           (y0 < 0. && y1 < 0. && y2 < 0.) ||
           (y0 > h  && y1 > h  && y2 > h ) ||
           (d0 > 0. && d1 > 0. && d2 > 0.) {
            return;
        }

        for scan_y in (y0 as i32).max(0)..(y1 as i32).min(self.cam.window.height as i32) {
            let percent_1 = ((scan_y as f64 - y0) / (y1 - y0)).clamp(0., 1.);
            let percent_2 = ((scan_y as f64 - y0) / (y2 - y0)).clamp(0., 1.);

            let mut lower_end_x = lerp(x0, x1, percent_1);
            let mut upper_end_x = lerp(x0, x2, percent_2);

            let mut lower_end_d = lerp(d0, d1, percent_1);
            let mut upper_end_d = lerp(d0, d2, percent_2);

            if upper_end_x < lower_end_x {
                (lower_end_x, upper_end_x) = (upper_end_x, lower_end_x);
                (lower_end_d, upper_end_d) = (upper_end_d, lower_end_d);
            }

            
            for scan_x in (lower_end_x as i32).max(0)..=(upper_end_x as i32).min(self.cam.window.width as i32 - 1) {
                let percent = ((scan_x as f64 - lower_end_x) / (upper_end_x - lower_end_x)).clamp(0., 1.);
                let d = lerp(lower_end_d, upper_end_d, percent);

                if self.cam.window.check_depth(scan_x as usize, scan_y as usize, d) {
                    // bounds check guarenteed by depth check
                    self.plot(scan_x as usize, scan_y as usize, color_u8);
                }
            }
        }

        for scan_y in (y1 as i32).max(0)..=(y2 as i32).min(self.cam.window.height as i32 - 1) {
            let percent_1 = ((scan_y as f64 - y1) / (y2 - y1)).clamp(0., 1.);
            let percent_2 = ((scan_y as f64 - y0) / (y2 - y0)).clamp(0., 1.);

            let mut lower_end_x = lerp(x1, x2, percent_1);
            let mut upper_end_x = lerp(x0, x2, percent_2);

            let mut lower_end_d = lerp(d1, d2, percent_1);
            let mut upper_end_d = lerp(d0, d2, percent_2);

            if upper_end_x < lower_end_x {
                (lower_end_x, upper_end_x) = (upper_end_x, lower_end_x);
                (lower_end_d, upper_end_d) = (upper_end_d, lower_end_d);
            }

            for scan_x in (lower_end_x as i32).max(0)..=(upper_end_x as i32).min(self.cam.window.width as i32 - 1) {
                let percent = ((scan_x as f64 - lower_end_x) / (upper_end_x - lower_end_x)).clamp(0., 1.);
                let d = lerp(lower_end_d, upper_end_d, percent);

                if self.cam.window.check_depth(scan_x as usize, scan_y as usize, d) {
                    // bounds check guarenteed by depth check
                    self.plot(scan_x as usize, scan_y as usize, color_u8);
                }
            }
        }
    }
}