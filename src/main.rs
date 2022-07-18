use figlet_rs::FIGfont;
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
    let figure = font.convert("Hackerman Mode 030");

    print!("{}", figure.unwrap().to_string())
}
