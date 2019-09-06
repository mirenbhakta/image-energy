mod lab;
mod rgb;
mod energy;
mod config;
mod process;
mod fill_avg_color;


use crate::config::Config;
use crate::process::process;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub fn run_app() -> Result<()> {
    let config = Config::from_args()?;
    process(&config)
}
