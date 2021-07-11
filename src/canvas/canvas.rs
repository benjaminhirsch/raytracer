use std::iter::FromIterator;

use crate::color::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn create(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![vec![Color::create(0.0, 0.0, 0.0); width]; height],
        }
    }

    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
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
        for x in &self.pixels {
            for y in x {
                header.push_str(&*y.to_string());
                pixels += 1;

                if pixels == 5 {
                    header.push_str("\n");
                    pixels = 0;
                } else {
                    header.push_str(" ");
                }
            }
        }

        header
    }
}

#[test]
fn canvas() {
    let mut canvas = Canvas::create(10, 20);
    assert_eq!(10, canvas.width);
    assert_eq!(20, canvas.height);

    canvas.write(2, 3, Color::create(1.0, 0.0, 0.0));

    assert_eq!(1.0, canvas.pixel_at(2, 3).red);
    assert_eq!(0.0, canvas.pixel_at(2, 3).green);
    assert_eq!(0.0, canvas.pixel_at(2, 3).blue);
}

#[test]
fn canvas_as_string() {
    let mut canvas = Canvas::create(5, 3);

    canvas.write(0, 0, Color::create(1.5, 0.0, 0.0));
    canvas.write(2, 1, Color::create(0.0, 0.5, 0.0));
    canvas.write(4, 2, Color::create(-0.5, 0.0, 1.0));

    assert_eq!("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n", canvas.to_string());
}
