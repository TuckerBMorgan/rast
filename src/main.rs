extern crate minifb;
pub mod math;

use self::math::*;

use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let top = Vector2::new(320.0, 100.0);
    let bottom_left = Vector2::new(100.0, 300.0);
    let bottom_right = Vector2::new(540.0, 300.0);

    let triangle = Triangle::new([top, bottom_left, bottom_right]);

    let mut inter_points = vec![];

    for i in 0..HEIGHT {
        let a = Vector2::new(0.0, i as f32);
        let b = Vector2::new(WIDTH as f32, i as f32);
        let line = Line::new(a, b);
        inter_points.push(triangle.line_intersection_test(&line));
    }

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

   // println!("{:?}", inter_points);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for points in &inter_points {
            if points.len() == 0 {
                continue;
            }
            let start;
            let end;
            if points[0].x < points[1].x {
                start = points[0];
                end = points[1];
            }
            else {
                start = points[1];
                end = points[0];
            }

            let y = start.y as usize;
            let start_x = start.x as usize;

            let first_pos = (y * WIDTH as usize) + start_x;
            for i in 0..(end.x as u32 - start.x as u32) {
                buffer[first_pos + i as usize] = 255; 
            }
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}