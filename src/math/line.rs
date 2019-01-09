use crate::math::Vector2;

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub a: Vector2,
    pub b: Vector2
}

impl Line {
    pub fn new(a: Vector2, b: Vector2) -> Line{
        Line {
            a,
            b
        }
    }

    pub fn does_line_intersect(&self, other: &Line) -> Option<Vector2> {
        let a1 = self.b.y - self.a.y;
        let b1 = self.a.x - self.b.x;
        let c1 = a1 * self.a.x + b1 * self.a.y;

        let a2 = other.b.y - other.a.y;
        let b2 = other.a.x - other.b.x;
        let c2 = a2 * other.a.x + b2 * other.a.y;
        
        let det = a1 * b2 - a2 * b1;
        
        if det == 0.0 {
            return None;
        }
        else {
            return Some(Vector2::new(
                (b2 * c1 - b1 * c2) / det,
                (a1 * c2 - a2 * c1) / det
            ));
        }
    }
}