use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::canvas::canvas::Canvas;
use std::fs::{create_dir, File};
use std::io::{Error, Write};
use std::path::Path;

#[allow(dead_code, unused_must_use)]
pub fn write_ppm(canvas: Canvas) -> Result<Box<String>, Error> {
    let mut rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    rand_string.push_str(".ppm");

    let mut path = String::new();
    path.push_str("ppm/");
    path.push_str(&rand_string);

    let target_path = Path::new(&path);
    if !target_path.exists() {
        create_dir("ppm");
    }

    let output = File::create(target_path)?;
    write!(&output, "{}", canvas.to_string())?;

    Ok(Box::new(target_path.to_str().unwrap().to_string()))
}
