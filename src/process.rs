use crate::Result;
use crate::config::{Config, Function, Mode};
use image::{DynamicImage, GenericImageView, ColorType};
use crate::energy::*;
use std::path::PathBuf;
use crate::lab::Lab;

pub fn process(config: &Config) -> Result<()> {
    if config.path.is_dir() {
        directory(config)
    } else if config.path.is_file() {
        single_image(config)
    } else {
        Err(failure::err_msg("path is not a directory or a file"))
    }
}

fn single_image(config: &Config) -> Result<()> {
    let image = image::open(&config.path)?;
    match config.function {
        Function::Energy => energy(config, new_path(&config.path), &image)?,
        Function::FillAvgColor => {},
        Function::EnergyAsAlpha => {},
    }
    Ok(())
}

fn directory(config: &Config) -> Result<()> {

    Ok(())
}

fn energy(config: &Config, path: PathBuf, image: &DynamicImage) -> Result<()> {
    let width = image.width();
    let height = image.height();

    match config.mode {
        Mode::Component => {
            let write = energy_component(config, image);
            Ok(image::save_buffer(path, &write, width, height, ColorType::RGB(8))?)
        },
        Mode::Combined => {
            let write = energy_combined(config, image);
            Ok(image::save_buffer(path, &write, width, height, ColorType::Gray(8))?)
        },
    }
}

// shared helper methods
pub fn new_path(path: &PathBuf) -> PathBuf {
    let mut p = path.clone();
    p.set_file_name(
        format!(
            "{} energy.png",
            path.file_stem().unwrap().to_str().unwrap()
        )
    );
    p
}

pub fn pixels_to_lab(image: &DynamicImage) -> Vec<Lab>
{
    let mut lab = Vec::with_capacity((image.width() * image.height()) as usize);

    image.pixels().for_each(|i| {
        lab.push(Lab::from_rgba(&i.2.data));
    });

    lab
}

pub fn get_index(x: u32, y: u32, width: u32) -> usize
{
    (x + y * width) as usize
}

