pub use crate::math::{Vector2, Line};

pub struct Triangle {
    pub points: [Vector2; 3],
    pub center: Vector2
}

impl Triangle {
    pub fn new(points: [Vector2; 3]) -> Triangle {
        let mut mid_x = 0.0;
        let mut mid_y = 0.0;
        for point in &points {
            mid_x += point.x;
            mid_y += point.y;
        }

        mid_x /= 3.0;
        mid_y /= 3.0;

        Triangle {
            points,
            center: Vector2::new(mid_x, mid_y)
        }
    }

    pub fn line_intersection_test(&self, line: &Line) -> Vec<Vector2> {
        let mut intersection_points : Vec<Vector2> = vec![];
        for i in 0..3 {
            let a = self.points[i];
            let b = self.points[(i  + 1) % 3];
            let tri_line = Line::new(a, b);
            let result = tri_line.other_does_line_intersect(&line);

            match result {
                Some(p) => {
                    intersection_points.push(p);
                },
                None => {}
            }
             
        }
        return intersection_points;
    }

    pub fn rotate_triangle(&mut self, degree: f32 ) {
        for point in &mut self.points {
            point.x -= self.center.x;
            point.y -= self.center.y;
            point.x = point.x * degree.cos() - point.y * degree.sin();
            point.y = point.y * degree.cos() + point.x * degree.sin();
            point.x += self.center.x;
            point.y += self.center.y;
        }
    }
}