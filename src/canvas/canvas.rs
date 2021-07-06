use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::color::color::Color;

#[derive(Debug)]
pub struct Canvas {
    width: i64,
    height: i64,
    pixels: BTreeMap<String, Color>,
}

impl Canvas {
    pub fn create(width: i64, height: i64) -> Self {
        let mut pixels = BTreeMap::new();
        for x in 0..width {
            for y in 0..height {
                pixels.insert(key_from_coordinates(x, y), Color::create(0.0, 0.0, 0.0));
            }
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn write(&mut self, x: i64, y: i64, color: Color) {
        self.pixels.insert(key_from_coordinates(x, y), color);
    }

    pub fn pixel_at(&self, x: i64, y: i64) -> Option<&Color> {
        self.pixels.get(key_from_coordinates(x, y).as_str())
    }
}

impl ToString for Canvas {
    fn to_string(&self) -> String {
        let mut header = String::new();

        header.push_str(&*String::from_iter([
            "P3",
            "\n",
            &(self.width).to_string(),
            " ",
            &(self.height).to_string(),
            "\n",
            "255",
            "\n",
        ]));

        let mut pixels = 0;
        for color in &self.pixels {
            header.push_str(&*color.1.to_string());
            pixels += 1;

            if pixels == 5 {
                header.push_str("\n");
                pixels = 0;
            } else {
                header.push_str(" ");
            }
        }

        header
    }
}

fn key_from_coordinates(x: i64, y: i64) -> String {
    format!("{}_{}", x, y)
}

#[test]
fn canvas() {
    let mut canvas = Canvas::create(10, 20);
    assert_eq!(10, canvas.width);
    assert_eq!(20, canvas.height);

    canvas.write(2, 3, Color::create(1.0, 0.0, 0.0));

    assert_eq!(1.0, canvas.pixel_at(2, 3).unwrap().red);
    assert_eq!(0.0, canvas.pixel_at(2, 3).unwrap().green);
    assert_eq!(0.0, canvas.pixel_at(2, 3).unwrap().blue);
}

#[test]
fn canvas_as_string() {
    let mut canvas = Canvas::create(5, 3);

    canvas.write(0, 0, Color::create(1.5, 0.0, 0.0));
    canvas.write(2, 1, Color::create(0.0, 0.5, 0.0));
    canvas.write(4, 2, Color::create(-0.5, 0.0, 1.0));

    assert_eq!("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n", canvas.to_string());
}
