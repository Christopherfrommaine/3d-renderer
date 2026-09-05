use std::{convert::identity, f64::consts::PI, thread::scope};

use crate::{linalg::Matrix, structures::{Camera, Object, Scene}};

mod linalg;
mod structures;
mod obj;
mod window;
mod renderers;


fn percentile(sorted: &[f32], p: f32) -> f32 {
    assert!(!sorted.is_empty());
    assert!((0.0..=1.0).contains(&p));

    let idx = ((sorted.len() - 1) as f32 * p).round() as usize;
    sorted[idx]
}

fn print_stats(mut frame_times: Vec<f32>) {
    if frame_times.is_empty() {
        println!("No frame times provided");
        return;
    }

    frame_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mean_frame_time = frame_times.iter().sum::<f32>() / frame_times.len() as f32;
    let mean_fps = 1.0 / mean_frame_time;

    let p50 = percentile(&frame_times, 0.50);
    let p95 = percentile(&frame_times, 0.95);
    let p99 = percentile(&frame_times, 0.99);

    log::info!("Frame time stats:");
    log::info!("  avg:  {:.3} ms ({:.2} FPS)", mean_frame_time * 1000.0, mean_fps);
    log::info!("  p50:  {:.3} ms ({:.2} FPS)", p50 * 1000.0, 1.0 / p50);
    log::info!("  p95:  {:.3} ms ({:.2} FPS)", p95 * 1000.0, 1.0 / p95);
    log::info!("  p99:  {:.3} ms ({:.2} FPS)", p99 * 1000.0, 1.0 / p99);

}

fn test_scene_triangle() {
    let mut frame_times = vec![];

    let target_framerate: f64 = 60.;
    let target_frame_time = std::time::Duration::from_secs_f64(target_framerate.recip());
    
    let mut scene = Scene::new(vec![Object::triangle()], Camera::default());
    
    for i in 0.. {
        let start = std::time::Instant::now();

        scene.render_normal();

        if !scene.cam.window.update_frame() {
            break;
        }

        scene.objs[0].rot = Matrix::rotation_matrix(i as f64 * 0.0030, 0., 0.);

        let elapsed = start.elapsed();
        frame_times.push(elapsed.as_secs_f32());
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        } else {
            log::debug!("{:.02} fps", elapsed.as_secs_f64().recip())
        }
    }

    print_stats(frame_times);
}

fn test_scene_with_renderer(renderer: fn(&mut Scene)) {
    let mut frame_times = vec![];

    let target_framerate: f64 = 60.;
    let target_frame_time = std::time::Duration::from_secs_f64(target_framerate.recip());
    
    let mut scene = Scene::new(vec![Object::sphere()], Camera::default());
    // scene.objs[0].rot = Matrix::rotation_matrix(0.4, 0.3, 1.2);
    
    for i in 0.. {
        let start = std::time::Instant::now();

        renderer(&mut scene);
        // scene.render_depth();
        if !scene.cam.window.update_frame() {
            break;
        }

        scene.objs[0].pos[2] = 9. * (i as f64 * -0.01).sin();

        scene.objs[0].rot = Matrix::rotation_matrix(3.14159 * 0.5 + i as f64 * 0.0080, i as f64 * 0.0085, 3.14159 + i as f64 * 0.0087);

        let elapsed = start.elapsed();
        frame_times.push(elapsed.as_secs_f32());
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        } else {
            log::debug!("{:.02} fps", elapsed.as_secs_f64().recip())
        }
    }

    print_stats(frame_times);
}

fn main() {
    env_logger::init();
    
    // test_scene_triangle();
    // test_scene_with_renderer(Scene::render_wireframe);
    test_scene_with_renderer(Scene::render_normal);
    
}