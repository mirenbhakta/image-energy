extern crate failure;
extern crate exitfailure;
extern crate rayon;
extern crate walkdir;
extern crate lab;
extern crate image;

extern crate energy;
use energy::*;

fn main() -> std::result::Result<(), exitfailure::ExitFailure> {
    Ok(run_app()?)
}
