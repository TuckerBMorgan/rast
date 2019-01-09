pub use crate::math::{Vector2, Line};

pub struct Triangle {
    pub points: [Vector2; 3]
}

impl Triangle {
    pub fn new(points: [Vector2; 3]) -> Triangle {
        Triangle {
            points
        }
    }

    pub fn line_intersection_test(&self, line: &Line) -> Vec<Vector2> {
        let mut intersection_points : Vec<Vector2> = vec![];
        for i in 0..3 {
            let a = self.points[i];
            let b = self.points[(i  + 1) % 3];
            let tri_line = Line::new(a, b);
            let result = tri_line.does_line_intersect(&line);

            match result {
                Some(p) => {
                    intersection_points.push(p);
                },
                None => {}
            }
             
        }
        return intersection_points;
    }
}