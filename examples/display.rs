//! This example displays QOI images using the embedded-graphics simulator.
//!
//! Usage: cargo run --example display QOI_IMAGE

use clap::Parser;
use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
use std::{fs, path::PathBuf};
use tinyqoi::Qoi;

#[derive(Parser)]
struct Args {
    qoi_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let data = fs::read(&args.qoi_file).unwrap();
    let qoi = Qoi::new(&data).unwrap();

    let mut display = SimulatorDisplay::<Rgb888>::new(qoi.size());
    Image::new(&qoi, Point::zero()).draw(&mut display).unwrap();

    let mut window = Window::new("qoi viewer", &OutputSettings::default());
    window.show_static(&display);
}
