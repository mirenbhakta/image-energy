use crate::Result;
use crate::config::*;
use crate::process::*;
use image::{DynamicImage, GenericImageView, ColorType};
use crate::lab::Lab;

pub fn energy_component(config: &Config, image: &DynamicImage) -> Vec<u8> {
    match config.color_space {
        ColorSpace::RGB => energy_component_rgb(config, image),
        ColorSpace::LAB => energy_component_lab(config, image),
    }
}

pub fn energy_combined(config: &Config, image: &DynamicImage) -> Vec<u8> {
    match config.color_space {
        ColorSpace::RGB => energy_combined_rgb(config, image),
        ColorSpace::LAB => energy_combined_lab(config, image),
    }
}

fn energy_component_rgb(config: &Config, image: &DynamicImage) -> Vec<u8> {
    let width = image.width();
    let height = image.height();

    let pixels: Vec<_> = image.pixels().map(|x| x.2.data).collect();
    let mut img_energy = Vec::with_capacity(pixels.len());

    let mut max_energy = [std::f32::MIN; 3];

    for y in 0..height {
        for x in 0..width {
            let index = get_index(x, y, width);

            fn do_thing(a: [u8; 4], b: [u8; 4]) -> [f32; 3] {
                [a[0] as f32 - b[0] as f32, a[1] as f32 - b[1] as f32, a[2] as f32 - b[2] as f32]
            }

            let dx = {
                let (a, b) = if x == 0 {
                    (pixels[get_index(x + 1, y, width)], pixels[index])
                } else if x == width - 1 {
                    (pixels[index], pixels[get_index(x - 1, y, width)])
                } else {
                    (pixels[get_index(x + 1, y, width)], pixels[get_index(x - 1, y, width)])
                };
                do_thing(a, b)
            };

            let dy = {
                let (a, b) = if y == 0 {
                    (pixels[get_index(x, y + 1, width)], pixels[index])
                } else if y == height - 1 {
                    (pixels[index], pixels[get_index(x, y - 1, width)])
                } else {
                    (pixels[get_index(x, y + 1, width)], pixels[get_index(x, y - 1, width)])
                };
                do_thing(a, b)
            };

            let energy = [dx[0].abs() + dy[0].abs(), dx[1].abs() + dy[1].abs(), dx[2].abs() + dy[2].abs()];

            for i in 0..3 {
                if energy[i] > max_energy[i] {
                    max_energy[i] = energy[i];
                }
            }

            img_energy.push(energy);
        }
    }

    component_write(config, &img_energy, max_energy)
}

fn component_write(config: &Config, img_energy: &Vec<[f32; 3]>, max_energy: [f32; 3]) -> Vec<u8>
{
    img_energy.iter().map(|v| {
        v.iter().zip(max_energy.iter()).map(|x| {
            ((x.0 / x.1).powf(config.exponent) * 255.0) as u8
        })
    }).flatten().collect()
}

fn energy_component_lab(config: &Config, image: &DynamicImage) -> Vec<u8> {
    let width = image.width();
    let height = image.height();

    let pixels = pixels_to_lab(image);
    let mut img_energy = Vec::with_capacity(pixels.len());

    let mut max_energy = [std::f32::MIN; 3];

    for y in 0..height {
        for x in 0..width {
            let index = get_index(x, y, width);

            let dx = {
                let (a, b) = if x == 0 {
                    (pixels[get_index(x + 1, y, width)], pixels[index])
                } else if x == width - 1 {
                    (pixels[index], pixels[get_index(x - 1, y, width)])
                } else {
                    (pixels[get_index(x + 1, y, width)], pixels[get_index(x - 1, y, width)])
                };
                a - b
            };

            let dy = {
                let (a, b) = if y == 0 {
                    (pixels[get_index(x, y + 1, width)], pixels[index])
                } else if y == height - 1 {
                    (pixels[index], pixels[get_index(x, y - 1, width)])
                } else {
                    (pixels[get_index(x, y + 1, width)], pixels[get_index(x, y - 1, width)])
                };
                a - b
            };

            let energy = (dx.abs_each() + dy.abs_each()).to_array();

            for i in 0..3 {
                if energy[i] > max_energy[i] {
                    max_energy[i] = energy[i];
                }
            }

            img_energy.push(energy);
        }
    }

    if config.convert_lab_to_rgb {
        img_energy.iter().flat_map(|v| -> Vec<_> {
            let rgb = Lab::array_to_rgb(v);

            rgb.iter().zip(max_energy.iter()).into_iter().map(|x| {
                ((*x.0 as f32 / *x.1).powf(config.exponent) * 255.0) as u8
            }).collect()
        }).collect()
    } else {
        component_write(config, &mut img_energy, max_energy)
    }
}

fn energy_combined_rgb(config: &Config, image: &DynamicImage) -> Vec<u8> {
    let width = image.width();
    let height = image.height();

    let pixels: Vec<_> = image.pixels().map(|x| x.2.data).collect();
    let mut img_energy = Vec::with_capacity(pixels.len());

    let mut max_energy = std::f32::MIN;

    for y in 0..height {
        for x in 0..width {
            let index = get_index(x, y, width);

            fn do_thing(a: [u8; 4], b: [u8; 4]) -> f32 {
                let v = [a[0] as f32 - b[0] as f32, a[1] as f32 - b[1] as f32, a[2] as f32 - b[2] as f32];
                v.iter().map(|x| x.powi(2)).sum()
            }

            let dx = {
                let (a, b) = if x == 0 {
                    (pixels[get_index(x + 1, y, width)], pixels[index])
                } else if x == width - 1 {
                    (pixels[index], pixels[get_index(x - 1, y, width)])
                } else {
                    (pixels[get_index(x + 1, y, width)], pixels[get_index(x - 1, y, width)])
                };
                do_thing(a, b)
            };

            let dy = {
                let (a, b) = if y == 0 {
                    (pixels[get_index(x, y + 1, width)], pixels[index])
                } else if y == height - 1 {
                    (pixels[index], pixels[get_index(x, y - 1, width)])
                } else {
                    (pixels[get_index(x, y + 1, width)], pixels[get_index(x, y - 1, width)])
                };
                do_thing(a, b)
            };

            let energy = dx.abs() + dy.abs();

            if energy > max_energy {
                max_energy = energy;
            }

            img_energy.push(energy);
        }
    }

    combined_write(config, &mut img_energy, max_energy)
}

fn energy_combined_lab(config: &Config, image: &DynamicImage) -> Vec<u8> {
    let width = image.width();
    let height = image.height();

    let pixels = pixels_to_lab(image);
    let mut img_energy = Vec::with_capacity(pixels.len());

    let mut max_energy = std::f32::MIN;

    for y in 0..height {
        for x in 0..width {
            let index = get_index(x, y, width);

            let dx = if x == 0 {
                (pixels[get_index(x + 1, y, width)] - pixels[index]).squared_len()
            } else if x == width - 1 {
                (pixels[index] - pixels[get_index(x - 1, y, width)]).squared_len()
            } else {
                (pixels[get_index(x + 1, y, width)] - pixels[get_index(x - 1, y, width)]).squared_len()
            };

            let dy = if y == 0 {
                (pixels[get_index(x, y + 1, width)] - pixels[index]).squared_len()
            } else if y == height - 1 {
                (pixels[index] - pixels[get_index(x, y - 1, width)]).squared_len()
            } else {
                (pixels[get_index(x, y + 1, width)] - pixels[get_index(x, y - 1, width)]).squared_len()
            };

            let energy = dx.abs() + dy.abs();

            if energy > max_energy {
                max_energy = energy;
            }

            img_energy.push(energy);
        }
    }

    combined_write(config, &mut img_energy, max_energy)
}

fn combined_write(config: &Config, img_energy: &mut Vec<f32>, max_energy: f32) -> Vec<u8> {
    img_energy.iter().map(|x| {
        ((x / max_energy).powf(config.exponent) * 255.0) as u8
    }).collect()
}
