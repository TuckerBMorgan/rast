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

    pub fn other_does_line_intersect(&self, other: &Line) -> Option<Vector2> {
        
        let a1  = other.a.y - other.b.y;
        let a2 = self.a.x - other.a.x;
        let a3 = other.b.x - other.a.x;
        let a4 = self.a.y - other.a.y;

        let a5 = self.a.y - self.b.y;
        let a6 = self.a.x - self.b.x;
        let a7 = other.b.y - other.a.y;

        let ta = (a1 * a2 + a3 * a4) / (a3 * a5 - a6 * a7);

        


        return None;
    }

    pub fn does_line_intersect(&self, other: &Line) -> Option<Vector2> {
     /*
        println!("============");
        println!("{} {} {} {}", self.a.x, self.a.y, self.b.x, self.b.y);
        println!("{} {} {} {}", other.a.x, other.a.y, other.b.x, other.b.y);
    */
        let a1 = self.b.y - self.a.y;
        let b1 = self.a.x - self.b.x;
        let c1 = a1 * self.a.x + b1 * self.a.y;

        let a2 = other.b.y - other.a.y;
        let b2 = other.a.x - other.b.x;
        let c2 = a2 * other.a.x + b2 * other.a.y;
        
        let det = a1 * b2 - a2 * b1;
      //  println!("{} {} {} {}, {}", det, a1, a2, b1, b2);
      //  println!("============");
        if det == 0.0 {
            return None;
        }
        else {

            let ta = (b2 * c1 - b1 * c2) / det;
            let tb = (a1 * c2 - a2 * c1) / det;
            if ta >= 0.0 && ta <= 1.0 && tb >= 0.0 && tb <= 1.0{
                return Some(Vector2::new(
                    ta, 
                    tb
                ));
            }
            return None;
        }
    }
}