use std::array::from_fn;

use crate::{linalg::Matrix, structures::*};

/* 
Transformations and Coordinate Systems:

Object Coordinates (from .obj file, not relative to a scene)

World Coordinates (placement in the scene. Usually obj_coords + obj.pos, plus rotation, etc)
- world_coord = obj_rot_mat * obj_coord + pos

Camera Coordinates (transform world so camera is at origin, facing in -z axis)
- camera_coord = cam_rot_mat.inverse() * (world_coord - camera_coord)

Screen Coordinates ((-1, -1) to (1, 1), projected in some way)
- (x', y') = (x, y) * p / z  # p is frustum distance

Pixel Coordinates ((0, 0) to (width, height))
*/


impl Scene {
    pub fn render_wireframe(&mut self) {
        let world_to_camera =  self.cam.rotinv.to_affine_translate_first(-1. * self.cam.pos);
        let screen_to_pixel = ((self.cam.window.width as f64 * 0.5) * Matrix::identity()).to_affine_translate_last(Matrix::from_array([0.5 * self.cam.window.width as f64, 0.5 * self.cam.window.height as f64, 0.]));

        let objs = self.objs.clone();
        for obj in objs {
            let obj_to_world = obj.rot.to_affine_translate_last(obj.pos);
            let obj_to_camera = world_to_camera * obj_to_world;
            

            for tri in obj.tri.iter() {
                let mut skip_tri = false;
                let points: [[i32; 2]; 3] = from_fn(|i| {
                    let point_obj_coords = tri[i].to_affine_transformation_vector();
                    let point_cam_coords = obj_to_camera * point_obj_coords;

                    if point_cam_coords[2] < self.cam.p {
                        skip_tri = true;
                    }
                    
                    let camera_to_screen = self.cam.p / point_cam_coords[2];
                    let mut point_screen_coords = camera_to_screen * point_cam_coords;
                    point_screen_coords.v[3][0] = 1.;

                    let point_pixel_coords = screen_to_pixel * point_screen_coords;

                    log::trace!("obj_to_world: \t{obj_to_world:?}");
                    log::trace!("world_to_camera: \t{world_to_camera:?}");
                    log::trace!("camera_to_screen: \t{camera_to_screen:?}");
                    log::trace!("screen_to_pixel: \t{screen_to_pixel:?}");
                    log::trace!("");
                    log::trace!("point_obj_coords: \t{:?}", point_obj_coords);
                    log::trace!("point_world_coords: \t{:?}", obj_to_world * point_obj_coords);
                    log::trace!("point_cam_coords: \t{:?}", point_cam_coords);
                    log::trace!("point_screen_coords:\t{:?}", point_screen_coords);
                    log::trace!("point_pixel_coords: \t{:?}", point_pixel_coords);
                    log::trace!("\n");


                    [point_pixel_coords[0] as i32, point_pixel_coords[1] as i32]
                });

                if skip_tri { continue; }
                for (i, j) in [(0, 1), (1, 2), (2, 0)] {
                    self.plot_line([points[i], points[j]], (255, 255, 255));
                }
            }
        }
    }
}