use crossterm::terminal::*;
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
    let space_width: u32 = font.convert(" ").unwrap().characters.first().unwrap().width;

    let intro_string: String = "Hackerman Mode 030".to_string();
    let str_words: Vec<&str> = intro_string.split(" ").collect();

    let main_fig = font.convert("Hackerman Mode 030");

    let (cols, rows) = size()?;

    Ok(())
}

fn calculate_width(figure: FIGure) -> u32 {
    let mut width: u32 = 0;

    for char in figure.characters {
        width += char.width;
    }

    return width;
}

