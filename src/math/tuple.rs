use std::ops;

#[derive(Copy, Clone)]
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

    pub fn multiply_by(&self, factor: f64) -> Tuple {
        Tuple {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            w: self.w * factor,
        }
    }

    pub fn divide_by(&self, factor: f64) -> Tuple {
        Tuple {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
            w: self.w / factor,
        }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
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
    let multiplied_tuple = my_tuple.multiply_by(3.5);

    assert_eq!(3.5, multiplied_tuple.x);
    assert_eq!(-7.0, multiplied_tuple.y);
    assert_eq!(10.5, multiplied_tuple.z);
    assert_eq!(-14.0, multiplied_tuple.w);
}

#[test]
fn tuple_multiplying_by_fraction() {
    let my_tuple = Tuple::create(1.0, -2.0, 3.0, -4.0);
    let multiplied_tuple = my_tuple.multiply_by(0.5);

    assert_eq!(0.5, multiplied_tuple.x);
    assert_eq!(-1.0, multiplied_tuple.y);
    assert_eq!(1.5, multiplied_tuple.z);
    assert_eq!(-2.0, multiplied_tuple.w);
}

#[test]
fn tuple_divide_by_scalar() {
    let my_tuple = Tuple::create(1.0, -2.0, 3.0, -4.0);
    let multiplied_tuple = my_tuple.divide_by(2.0);

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
