use clap::Parser;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, image::Image};
use embedded_graphics_simulator::{SimulatorDisplay, OutputSettings, Window};
use tinyqoi::Qoi;
use std::{path::PathBuf, fs};

#[derive(Parser)]
struct Args {
    qoi_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let data = fs::read(&args.qoi_file).unwrap();
    let qoi = Qoi::new(&data);

    let mut display = SimulatorDisplay::<Rgb888>::new(qoi.size());
    Image::new(&qoi, Point::zero()).draw(&mut display).unwrap();
    
    let mut window = Window::new("qoi viewer", &OutputSettings::default());
    window.show_static(&display);
}
