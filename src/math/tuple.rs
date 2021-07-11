use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn create(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

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

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
    }

    pub fn normalize(&self) -> Tuple {
        Tuple {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude(),
        }
    }

    pub fn dot(&self, that: Tuple) -> f64 {
        self.x * that.x + self.y * that.y + self.z * that.z + self.w * that.w
    }

    pub fn cross(&self, that: Tuple) -> Tuple {
        Tuple::vector(
            self.y * that.z - self.z * that.y,
            self.z * that.x - self.x * that.z,
            self.x * that.y - self.y * that.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, that: &Self) -> bool {
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

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, factor: f64) -> Self::Output {
        Tuple {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
            w: self.w / factor,
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -1.0 * self.x,
            y: -1.0 * self.y,
            z: -1.0 * self.z,
            w: -1.0 * self.w,
        }
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, factor: f64) -> Self::Output {
        Tuple {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            w: self.w * factor,
        }
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

    assert_eq!(vec1, vec2);

    let vec1 = Tuple::vector(4.0, -4.0, 3.0);
    let vec2 = Tuple::vector(2.0, -4.0, 3.0);

    assert_ne!(vec1, vec2);

    let vec1 = Tuple::vector(4.0, -4.0, 3.0);
    let vec2 = Tuple::vector(4.0, -3.0, 3.0);

    assert_ne!(vec1, vec2);

    let vec1 = Tuple::vector(4.0, -4.0, 3.0);
    let vec2 = Tuple::vector(4.0, -4.0, 2.0);

    assert_ne!(vec1, vec2);
}

#[test]
fn tuple_addition() {
    let tuple_1 = Tuple::point(3.0, -2.0, 5.0);
    let tuple_2 = Tuple::vector(-2.0, 3.0, 1.0);
    let new_tuple = tuple_1 + tuple_2;

    assert_eq!(1.0, new_tuple.x);
    assert_eq!(1.0, new_tuple.y);
    assert_eq!(6.0, new_tuple.z);
    assert_eq!(true, new_tuple.is_point());
    assert_eq!(false, new_tuple.is_vector());
}

#[test]
fn tuple_subtraction() {
    let tuple_1 = Tuple::point(3.0, 2.0, 1.0);
    let tuple_2 = Tuple::point(5.0, 6.0, 7.0);
    let new_tuple = tuple_1 - tuple_2;

    assert_eq!(-2.0, new_tuple.x);
    assert_eq!(-4.0, new_tuple.y);
    assert_eq!(-6.0, new_tuple.z);
    assert_eq!(false, new_tuple.is_point());
    assert_eq!(true, new_tuple.is_vector());

    let tuple_1 = Tuple::point(3.0, 2.0, 1.0);
    let tuple_2 = Tuple::vector(5.0, 6.0, 7.0);
    let new_tuple = tuple_1 - tuple_2;

    assert_eq!(-2.0, new_tuple.x);
    assert_eq!(-4.0, new_tuple.y);
    assert_eq!(-6.0, new_tuple.z);
    assert_eq!(true, new_tuple.is_point());
    assert_eq!(false, new_tuple.is_vector());

    let tuple_1 = Tuple::vector(3.0, 2.0, 1.0);
    let tuple_2 = Tuple::vector(5.0, 6.0, 7.0);
    let new_tuple = tuple_1 - tuple_2;

    assert_eq!(-2.0, new_tuple.x);
    assert_eq!(-4.0, new_tuple.y);
    assert_eq!(-6.0, new_tuple.z);
    assert_eq!(false, new_tuple.is_point());
    assert_eq!(true, new_tuple.is_vector());
}

#[test]
fn tuple_overloading() {
    let tup = Tuple::create(1.0, -2.0, 3.0, -4.0);
    let negated_tuple = -tup;
    assert_eq!(-1.0, negated_tuple.x);
    assert_eq!(2.0, negated_tuple.y);
    assert_eq!(-3.0, negated_tuple.z);
    assert_eq!(4.0, negated_tuple.w);
}

#[test]
fn tuple_multiplying_by_scalar() {
    let my_tuple = Tuple::create(1.0, -2.0, 3.0, -4.0);
    let multiplied_tuple = my_tuple * 3.5;

    assert_eq!(3.5, multiplied_tuple.x);
    assert_eq!(-7.0, multiplied_tuple.y);
    assert_eq!(10.5, multiplied_tuple.z);
    assert_eq!(-14.0, multiplied_tuple.w);
}

#[test]
fn tuple_multiplying_by_fraction() {
    let my_tuple = Tuple::create(1.0, -2.0, 3.0, -4.0);
    let multiplied_tuple = my_tuple * 0.5;

    assert_eq!(0.5, multiplied_tuple.x);
    assert_eq!(-1.0, multiplied_tuple.y);
    assert_eq!(1.5, multiplied_tuple.z);
    assert_eq!(-2.0, multiplied_tuple.w);
}

#[test]
fn tuple_divide_by_scalar() {
    let my_tuple = Tuple::create(1.0, -2.0, 3.0, -4.0);
    let multiplied_tuple = my_tuple / 2.0;

    assert_eq!(0.5, multiplied_tuple.x);
    assert_eq!(-1.0, multiplied_tuple.y);
    assert_eq!(1.5, multiplied_tuple.z);
    assert_eq!(-2.0, multiplied_tuple.w);
}

#[test]
fn vector_magnitude() {
    let my_vector = Tuple::vector(1.0, 0.0, 0.0);
    assert_eq!(1.0, my_vector.magnitude());

    let my_vector = Tuple::vector(0.0, 1.0, 0.0);
    assert_eq!(1.0, my_vector.magnitude());

    let my_vector = Tuple::vector(0.0, 0.0, 1.0);
    assert_eq!(1.0, my_vector.magnitude());

    let my_vector = Tuple::vector(1.0, 2.0, 3.0);
    assert_eq!(f64::sqrt(14.0), my_vector.magnitude());

    let my_vector = Tuple::vector(-1.0, -2.0, -3.0);
    assert_eq!(f64::sqrt(14.0), my_vector.magnitude());
}

#[test]
#[allow(unused_must_use)]
fn vector_normalize() {
    let my_vector = Tuple::vector(4.0, 0.0, 0.0).normalize();
    assert_eq!(1.0, my_vector.x);
    assert_eq!(0.0, my_vector.y);
    assert_eq!(0.0, my_vector.z);

    let my_vector = Tuple::vector(1.0, 2.0, 3.0).normalize();
    abs_diff_eq!(0.26726, my_vector.x);
    abs_diff_eq!(0.53452, my_vector.y);
    abs_diff_eq!(0.80178, my_vector.z);
    assert_eq!(1.0, my_vector.magnitude());
}

#[test]
fn vector_to_dot() {
    let vec1 = Tuple::vector(1.0, 2.0, 3.0);
    let vec2 = Tuple::vector(2.0, 3.0, 4.0);

    assert_eq!(20.0, vec1.dot(vec2));
}

#[test]
fn vector_cross() {
    let vec1 = Tuple::vector(1.0, 2.0, 3.0);
    let vec2 = Tuple::vector(2.0, 3.0, 4.0);

    let cross = vec1.cross(vec2);
    assert_eq!(-1.0, cross.x);
    assert_eq!(2.0, cross.y);
    assert_eq!(-1.0, cross.z);

    let cross = vec2.cross(vec1);
    assert_eq!(1.0, cross.x);
    assert_eq!(-2.0, cross.y);
    assert_eq!(1.0, cross.z);
}
