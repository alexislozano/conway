mod world;
use world::World;

use gif::{Frame, Encoder, Repeat, SetParameter};
use std::fs::{File, read_to_string};
use std::borrow::Cow;

use clap::{crate_authors, crate_name, crate_version, App, Arg};

fn game(world: World, iterations: u16) -> Vec<Vec<u8>> {
    let mut world: World = world;
    let mut states: Vec<Vec<u8>> = vec![];
    for _ in 0..iterations {
        states.push(world.state());
        world = world.next();
    }
    states
}

fn process_file(filename: &str) -> Option<(u16, u16, Vec<Vec<u8>>)> {
    match read_to_string(filename) {
        Err(_) => None,
        Ok(file) => {
            let cells: Vec<Vec<u8>> = file.split("\n").filter(
                |line| line.len() > 0
            ).map(
                |line| line.split("").filter(
                    |c| c.len() > 0
                ).map(
                    |c| c.parse::<u8>().unwrap()
                ).collect()
            ).collect();
            Some((cells[0].len() as u16, cells.len() as u16, cells))
        }
    }
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("An implementation of Conway's Game of Life")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets a file to process")
                .takes_value(true),
        ).arg(
            Arg::with_name("iterations")
                .short("i")
                .long("iterations")
                .value_name("ITERATIONS")
                .help("Sets the desired number of iterations")
                .takes_value(true),
        ).arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .value_name("SIZE")
                .help("Sets the desired size of one pixel")
                .takes_value(true),
        ).get_matches();

    let filename = matches.value_of("file").unwrap_or("");
    let iterations = matches.value_of("iterations").unwrap_or("10").parse::<u16>().unwrap();
    let size = matches.value_of("size").unwrap_or("1").parse::<u16>().unwrap();

    let (width, height, cells) = process_file(filename).unwrap();

    let world = World::new(width, height, size, cells);
    let states = game(world, iterations);

    let color_map = &[0, 0, 0, 0xFF, 0xFF, 0xFF];
    let mut image = File::create("conway.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width * size, height * size, color_map).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    for state in states {
        let mut frame = Frame::default();
        frame.width = width * size;
        frame.height = height * size;
        frame.buffer = Cow::Borrowed(&state);
        frame.delay = 20;
        encoder.write_frame(&frame).unwrap();
    }
}