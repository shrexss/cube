mod vec3;
mod axis;
mod colors;
mod camera;
mod cube;

use vec3::Vec3;
use axis::Axis;
use colors::Colors;
use camera::Camera;
use cube::Cube;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn draw_line(buf: &mut [u32], w: usize, h: usize, a: (i32, i32), b: (i32, i32), color: u32) {
    let (mut x0, mut y0) = a;
    let (x1, y1) = b;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 && (x0 as usize) < w && (y0 as usize) < h {
            buf[y0 as usize * w + x0 as usize] = color;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

fn main() {
    // CREATE AXES
    let x_axis = Axis::new(Vec3::new(1.5, 0.0, 0.0), Colors::RED);
    let y_axis = Axis::new(Vec3::new(0.0, 1.5, 0.0), Colors::GREEN);
    let z_axis = Axis::new(Vec3::new(0.0, 0.0, 1.5), Colors::BLUE);
    let axes = [&x_axis, &y_axis, &z_axis];

    // CREATE CAMERA
    let mut cam = Camera::new();

    // CREATE MOUSE
    let mut last_mouse: Option<(f32, f32)> = None;

    // CREATE CUBE
    let cube = Cube::new();

    // CREATE BUFFER
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // CREATE WINDOW
    let mut window = Window::new(
        "CUBE", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    ).expect("could not create window");
    window.set_target_fps(120);

    // MAIN LOOP
    while window.is_open() && !window.is_key_down(Key::Escape){
        // INPUT MOUSE DRAG
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Pass) {
                if let Some((lx, ly)) = last_mouse {
                    cam.yaw += (mx - lx) * 0.01;
                    cam.pitch += (my - ly) * 0.01;
                    cam.pitch = cam.pitch.clamp(-1.5, 1.5);
                }
                last_mouse = Some((mx, my));
            }
        } else {
            last_mouse = None;
        }

        // INPUT MOUSE SCROLL
        if let Some((_, scroll_y)) = window.get_scroll_wheel() {
            cam.scale *= 1.03f32.powf(scroll_y);
            cam.scale = cam.scale.clamp(50.0, 5000.0);
        }

        // RESET BUFFER
        buffer.fill(0);

        // DRAW COORDS
        for axis in axes {
            if let (Some(a), Some(b)) = (
                cam.project(axis.start, WIDTH, HEIGHT),
                cam.project(axis.end, WIDTH, HEIGHT),
            ) {
                draw_line(&mut buffer, WIDTH, HEIGHT, a, b, axis.color);
            }
        }

        // DRAW CUBE
        let projected = cube.corners.map(|c| cam.project(c, WIDTH, HEIGHT));

        for &(i, j) in Cube::EDGES.iter() {
            if let (Some(a), Some(b)) = (projected[i], projected[j]) {
                draw_line(&mut buffer, WIDTH, HEIGHT, a, b, Colors::WHITE);
            }
        }


        // UPDATE WINDOW
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}