// window.rs

use std::{debug_assert, fmt};

use minifb::{Scale, Window as MiniWindow, WindowOptions};

pub struct Window {
    pub width: usize,
    pub height: usize,
    title: String,
    pub buffer: Vec<Vec<(u8, u8, u8)>>,
    pub depth: Vec<Vec<f64>>,
    frame: Vec<u32>,
    window: MiniWindow,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Self {
        let title = String::from("Window");

        let window = MiniWindow::new(
            &title,
            width,
            height,
            WindowOptions {
                resize: false,
                scale: Scale::X1,
                ..WindowOptions::default()
            },
        )
        .expect("failed to create window");

        let buffer = vec![
            vec![(0u8, 0u8, 0u8); width];
            height
        ];

        let depth = vec![vec![0.; width]; height];

        let frame = vec![0u32; width * height];

        Self {
            width,
            height,
            title,
            buffer,
            depth,
            frame,
            window,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.window.set_title(&self.title);
    }

    // Passing by reference is fine here.
    // This clones the pixel data into the window's internal buffer.
    pub fn set_buffer(&mut self, buffer: Vec<Vec<(u8, u8, u8)>>) {
        debug_assert_eq!(
            buffer.len(),
            self.height,
            "buffer height does not match window height"
        );

        for row in buffer.iter() {
            debug_assert_eq!(
                row.len(),
                self.width,
                "buffer width does not match window width"
            );
        }

        self.buffer = buffer;
    }

    pub fn update_frame(&mut self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                let (r, g, b) = self.buffer[y][x];
                self.frame[y * self.width + x] =
                    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }

        self.window
            .update_with_buffer(&self.frame, self.width, self.height)
            .expect("failed to update window buffer");

        self.buffer = vec![vec![(0, 0, 0); self.width]; self.height];
        self.depth = vec![vec![f64::INFINITY; self.width]; self.height];

        return self.window.is_open();
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        return x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32;
    }

    #[inline(always)]
    pub fn check_depth(&mut self, x: usize, y: usize, d: f64) -> bool {
        debug_assert!(self.in_bounds(x as i32, y as i32));

        if d > 0. { return false; }

        let b = self.depth[y as usize][x as usize] > d;
        if b {
            self.depth[y as usize][x as usize] = d;
        }
        return b;
    }
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Window").field("width", &self.width).field("height", &self.height).field("title", &self.title).finish()
    }
}

impl Default for Window {
    fn default() -> Self {
        // Self::new(640, 480)
        Self::new(1920, 1080)
    }
}

#[allow(dead_code)]
pub fn window_test() {
    let mut window = Window::new(1920, 1080);
    window.set_title("Raytracer");
    
    for t in 0u8..255 {
        let tf = t as f64 / 10.;

        let mut buffer = vec![vec![(0, 0, 0); 1920]; 1080];

        for i in 0..1080 {
            for j in 0..1920 {
                buffer[i][j] = ((255. * (0.5 + 0.5 * (0.01 * i as f64 + 0.01 * j as f64 + tf).sin())).floor() as u8, 0, 0);
            }
        }

        window.set_buffer(buffer);
        if !window.update_frame() {
            break;
        }
        
        println!("frame {t}");

    }
}