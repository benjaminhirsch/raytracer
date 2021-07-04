use crate::color::color::Color;
use std::collections::HashMap;
use std::iter::FromIterator;

struct Canvas {
    width: i64,
    height: i64,
    pixels: HashMap<i64, Color>,
}

impl Canvas {
    pub fn create(width: i64, height: i64) -> Self {
        let mut pixels = HashMap::new();
        for x in 1..width {
            for y in 1..height {
                pixels.insert(x * y, Color::create(0.0, 0.0, 0.0));
            }
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn write(&mut self, x: i64, y: i64, color: Color) {
        self.pixels.insert(x * y, color);
    }

    pub fn pixel_at(&self, x: i64, y: i64) -> Option<&Color> {
        self.pixels.get(&(x * y))
    }

    pub fn to_ppm(&self) -> String {
        let mut header = String::with_capacity((self.width * self.height) as usize);
        header.insert_str(
            0,
            &*String::from_iter([
                "P3",
                "\n",
                &(self.width).to_string(),
                " ",
                &(self.height).to_string(),
                "\n",
                "255",
            ]),
        );

        let mut count = 0u64;

        for color in self.pixels.iter() {
            count += 1;

            header.insert_str(
                count as usize,
                &*String::from_iter([
                    (color.1.red as i64).to_string(),
                    (color.1.green as i64).to_string(),
                    (color.1.blue as i64).to_string(),
                ]),
            );
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

    assert_eq!(1.0, canvas.pixel_at(2, 3).unwrap().red);
    assert_eq!(0.0, canvas.pixel_at(2, 3).unwrap().green);
    assert_eq!(0.0, canvas.pixel_at(2, 3).unwrap().blue);
}
