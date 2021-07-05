use std::ops;

#[derive(Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub(crate) fn create(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn multiply_by(&self, factor: f64) -> Color {
        Color {
            red: self.red * factor,
            green: self.green * factor,
            blue: self.blue * factor,
        }
    }

    pub fn as_string(&self) -> String {
        let mut color_as_string = String::new();
        color_as_string.push_str(&*String::from(float_to_int(self.red).to_string()));
        color_as_string.push_str(" ");
        color_as_string.push_str(&*String::from(float_to_int(self.green).to_string()));
        color_as_string.push_str(" ");
        color_as_string.push_str(&*String::from(float_to_int(self.blue).to_string()));

        color_as_string
    }
}

pub fn float_to_int(color: f64) -> u32 {
    let color_as_int = f64::round(color * 255.) as i32;

    if color_as_int < 0 {
        return 0;
    }

    if color_as_int > 255 {
        return 255;
    }

    return color_as_int as u32;
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

#[test]
fn color_from_rgb() {
    let color = Color::create(-0.5, 0.4, 1.7);
    assert_eq!(-0.5, color.red);
    assert_eq!(0.4, color.green);
    assert_eq!(1.7, color.blue);
}

#[test]
fn color_add() {
    let color1 = Color::create(0.9, 0.6, 0.75);
    let color2 = Color::create(0.7, 0.1, 0.25);
    let new_color = color1 + color2;
    assert_eq!(1.6, new_color.red);
    assert_eq!(0.7, new_color.green);
    assert_eq!(1.0, new_color.blue);
}

#[test]
fn color_subtract() {
    let color1 = Color::create(0.9, 0.6, 0.75);
    let color2 = Color::create(0.7, 0.1, 0.25);
    let new_color = color1 - color2;
    assert_eq!(0.20000000000000007, new_color.red);
    assert_eq!(0.5, new_color.green);
    assert_eq!(0.5, new_color.blue);
}

#[test]
fn color_multiply() {
    let color1 = Color::create(0.2, 0.3, 0.4);
    let new_color = color1.multiply_by(2.0);
    assert_eq!(0.4, new_color.red);
    assert_eq!(0.6, new_color.green);
    assert_eq!(0.8, new_color.blue);
}

#[test]
fn color_multiply_with_color() {
    let color1 = Color::create(1.0, 0.2, 0.4);
    let color2 = Color::create(0.9, 1.0, 0.1);
    let new_color = color1 * color2;
    assert_eq!(0.9, new_color.red);
    assert_eq!(0.2, new_color.green);
    assert_eq!(0.04000000000000001, new_color.blue);
}
