pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w > 0.0
    }

    pub fn equal_to(&self, that: Tuple) -> bool {
        if f64::abs(self.x - that.x) > f64::EPSILON {
            return false;
        }

        if f64::abs(self.y - that.y) > f64::EPSILON {
            return false;
        }

        if f64::abs(self.z - that.z) > f64::EPSILON {
            return false;
        }

        return true;
    }
}

#[test]
fn point_creation() {
    let my_point = Tuple::point(4.0, -4.0, 3.0);
    assert_eq!(my_point.is_point(), true);
    assert_eq!(my_point.is_vector(), false);

    assert_eq!(4.0, my_point.x);
    assert_eq!(-4.0, my_point.y);
    assert_eq!(3.0, my_point.z);
    assert_eq!(1.0, my_point.w);
}

#[test]
fn vector_creation() {
    let my_vector = Tuple::vector(4.0, -4.0, 3.0);
    assert_eq!(my_vector.is_point(), false);
    assert_eq!(my_vector.is_vector(), true);

    assert_eq!(4.0, my_vector.x);
    assert_eq!(-4.0, my_vector.y);
    assert_eq!(3.0, my_vector.z);
    assert_eq!(0.0, my_vector.w);
}

#[test]
fn tuple_comparison() {
    let vec1 = Tuple::vector(4.0, -4.0, 3.0);
    let vec2 = Tuple::vector(4.0, -4.0, 3.0);

    assert_eq!(true, vec1.equal_to(vec2));

    let vec1 = Tuple::vector(4.0, -4.0, 3.0);
    let vec2 = Tuple::vector(2.0, -4.0, 3.0);

    assert_eq!(false, vec1.equal_to(vec2));

    let vec1 = Tuple::vector(4.0, -4.0, 3.0);
    let vec2 = Tuple::vector(4.0, -3.0, 3.0);

    assert_eq!(false, vec1.equal_to(vec2));

    let vec1 = Tuple::vector(4.0, -4.0, 3.0);
    let vec2 = Tuple::vector(4.0, -4.0, 2.0);

    assert_eq!(false, vec1.equal_to(vec2));
}
