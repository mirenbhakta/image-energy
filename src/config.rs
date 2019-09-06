use crate::Result;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub path: PathBuf,
    pub exponent: f32,
    pub function: Function,
    pub mode: Mode,
    pub color_space: ColorSpace,
    pub convert_lab_to_rgb: bool,
}

#[derive(Clone)]
pub enum Function {
    Energy, // the usual
    EnergyAsAlpha, // treat energy as alpha and show the image according to energy
    FillAvgColor, // flood fill everything and fill the regions with the average color of that region
}

#[derive(Clone)]
pub enum Mode {
    Component,
    Combined,
}

#[derive(Clone)]
pub enum ColorSpace {
    RGB,
    LAB,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Self> {
        args.next(); // executable path, ignore

        let path = PathBuf::from(args.next().ok_or(failure::err_msg("Path not found"))?);

        let exponent = parse_exponent(args.next());

        let function = parse_function(args.next());

        let mode = parse_mode(args.next());

        let color_space = parse_color_space(args.next());

        let convert_lab_to_rgb = parse_component_convert_lab_to_rgb(args.next());

        Ok(Self { path, exponent, function, mode, color_space, convert_lab_to_rgb })
    }

    pub fn from_args() -> Result<Self> {
        Config::new(std::env::args())
    }
}

fn parse_exponent(args: Option<String>) -> f32 {
    const DEFAULT: f32 = 3.0;

    1.0 / match args {
        Some(v) => v.parse().unwrap_or(DEFAULT),
        None => DEFAULT,
    }
}

fn parse_function(args: Option<String>) -> Function {
    const DEFAULT: Function = Function::Energy;

    match args {
        Some(v) => {
            match &v[..] {
                "alpha" => Function::EnergyAsAlpha,
                "fillavg" => Function::FillAvgColor,
                //"mono" => Function::Monochrome,
                _ => DEFAULT,
            }
        },
        None => DEFAULT,
    }
}

fn parse_mode(args: Option<String>) -> Mode {
    const DEFAULT: Mode = Mode::Combined;

    match args {
        Some(v) => {
            match &v[..] {
                "component" => Mode::Component,
                //"combined" => Mode::Combined,
                _ => DEFAULT,
            }
        },
        None => DEFAULT,
    }
}

fn parse_color_space(args: Option<String>) -> ColorSpace {
    const DEFAULT: ColorSpace = ColorSpace::LAB;

    match args {
        Some(v) => {
            match &v[..] {
                "rgb" => ColorSpace::RGB,
                //"lab" => ColorSpace::LAB,
                _ => DEFAULT,
            }
        },
        None => DEFAULT,
    }
}

fn parse_component_convert_lab_to_rgb(args: Option<String>) -> bool {
    const DEFAULT: bool = false;

    match args {
        Some(v) => {
            match &v[..] {
                "yes" => true,
                //"no" => false,
                _ => DEFAULT,
            }
        },
        None => DEFAULT,
    }
}
