use figlet_rs::FIGfont;
use figlet_rs::{FIGfont, FIGure};
use std::io::Result;
use std::vec::Vec;

#[derive(Debug)]
struct FIGDimmensions {
    width: u32,
    rows: u32,
    line_height: u32,
    characters: u32,
    breakpoints: Vec<u32>,
}


fn main() -> Result<()> {
    let font = FIGfont::from_file("resources/roman.flf").unwrap();
    Ok(())
}

fn calculate_width(figure: FIGure) -> u32 {
    let mut width: u32 = 0;

    for char in figure.characters {
        width += char.width;
    }

    return width;
}

